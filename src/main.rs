mod config;
mod data;
mod render;
mod widgets;

use anyhow::Result;
use clap::Parser;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about = "A system monitor written in Rust")]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(short, long, help = "Output JSON format for shell script")]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.json {
        // For JSON mode, skip config loading entirely
        launch_json_output(1).await?;
    } else {
        // For terminal mode, load config normally
        let config = config::load_config(&cli.config).await?;
        let update_interval = config.update_interval;
        launch_terminal(update_interval).await?;
    }

    Ok(())
}

async fn launch_json_output(update_interval: u64) -> Result<()> {
    let mut system_info = data::SystemInfo::new();

    loop {
        system_info.refresh();

        // Get all system data
        let system_data = data::SystemData::from(&system_info);

        // Convert to JSON and output - ONLY JSON
        let json_output = serde_json::to_string(&system_data)?;
        println!("{}", json_output);

        // Flush stdout
        use std::io::{self, Write};
        io::stdout().flush()?;

        // Wait for next update
        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }
}

async fn launch_terminal(update_interval: u64) -> Result<()> {
    let mut system_info = data::SystemInfo::new();

    // Clear screen only once at start
    render::clear_screen()?;
    println!(
        "🚀 Rust Conky System Monitor - Update every {}s - Ctrl+C to stop",
        update_interval
    );
    println!();

    loop {
        system_info.refresh();
        render::draw(&system_info)?;
        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }
}
