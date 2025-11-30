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

    #[arg(short, long, default_value_t = 1)]
    interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = config::load_config(&cli.config).await?;

    // Initialize system info
    let mut system_info = data::SystemInfo::new();

    // Clear screen only once at start
    render::clear_screen()?;
    println!(
        "🚀 Rust Conky System Monitor - Update every {}s - Ctrl+C to stop",
        config.update_interval
    );
    println!(); // Empty line

    // Main loop - use the interval from config instead of CLI
    let interval = config.update_interval;
    loop {
        system_info.refresh();

        // This will update in place without creating new lines
        render::draw(&system_info)?;

        // Wait for next update (use config interval)
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}
