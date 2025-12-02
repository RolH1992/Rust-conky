mod config;
mod data;
mod gui;
mod render;

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

    #[arg(short, long)]
    gui: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = config::load_config(&cli.config).await?;

    if cli.gui {
        // Launch GUI version - pass the interval directly
        launch_gui(config.update_interval).await?;
    } else {
        // Launch terminal version
        launch_terminal(config.update_interval).await?;
    }

    Ok(())
}

async fn launch_gui(update_interval: u64) -> Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 600.0])
            .with_min_inner_size([300.0, 400.0])
            .with_title("Rust Conky")
            .with_transparent(true) // Enable transparency
            .with_decorations(false), // Remove window decorations for Conky look

        ..Default::default()
    };

    // Move the interval into the closure to satisfy lifetime requirements
    let update_interval = update_interval;
    eframe::run_native(
        "Rust Conky",
        native_options,
        Box::new(move |_cc| Ok(Box::new(gui::GuiApp::new(update_interval)))),
    )
    .map_err(|e| anyhow::anyhow!("GUI error: {}", e))
}

async fn launch_terminal(update_interval: u64) -> Result<()> {
    // Initialize system info
    let mut system_info = data::SystemInfo::new();

    // Clear screen only once at start
    render::clear_screen()?;
    println!(
        "🚀 Rust Conky System Monitor - Update every {}s - Ctrl+C to stop",
        update_interval
    );
    println!(); // Empty line

    // Main loop
    loop {
        system_info.refresh();

        // This will update in place without creating new lines
        render::draw(&system_info)?;

        // Wait for next update
        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }
}
