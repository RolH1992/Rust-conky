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

    #[arg(long, help = "Use TUI interface (ratatui)")]
    tui: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match (cli.json, cli.tui) {
        (true, _) => {
            // JSON mode for scripts
            launch_json_output(1).await?;
        }
        (false, true) => {
            // NEW: TUI mode with ratatui
            launch_tui(&cli.config).await?;
        }
        (false, false) => {
            // Legacy terminal mode (your current simple render)
            let config = config::load_config(&cli.config).await?;
            let update_interval = config.update_interval;
            launch_terminal(update_interval).await?;
        }
    }

    Ok(())
}

async fn launch_json_output(update_interval: u64) -> Result<()> {
    let mut system_info = data::SystemInfo::new();

    loop {
        system_info.refresh();
        let system_data = data::SystemData::from(&system_info);
        let json_output = serde_json::to_string(&system_data)?;
        println!("{}", json_output);

        std::io::Write::flush(&mut std::io::stdout())?;
        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }
}

async fn launch_terminal(update_interval: u64) -> Result<()> {
    let mut system_info = data::SystemInfo::new();
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

// NEW: TUI mode function
async fn launch_tui(config_path: &str) -> Result<()> {
    // Load config to get update interval
    let config = config::load_config(config_path).await?;
    let update_interval = config.update_interval;

    // Launch TUI using the new render::tui module
    render::tui::launch_tui(update_interval).await?;

    Ok(())
}
