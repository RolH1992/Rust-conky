// src/render/mod.rs
pub mod simple; // Your current render logic
pub mod tui; // New ratatui render logic

// Remove or comment out the unused parts:
// pub use simple::*;  // Remove this line if you're not using it elsewhere

// If you want to keep the old API, use conditional compilation:
#[cfg(not(feature = "tui"))]
pub use simple::*;

// Remove or comment out the unused enum and function:
/*
pub enum RenderMode {
    Simple,
    Tui,
}

pub fn render(system_info: &crate::data::SystemInfo, mode: RenderMode) -> anyhow::Result<()> {
    match mode {
        RenderMode::Simple => simple::draw(system_info),
        RenderMode::Tui => {
            anyhow::bail!("TUI mode should be launched via launch_tui() function");
        }
    }
}
*/

// Keep these functions - they're used by your main.rs
pub fn clear_screen() -> anyhow::Result<()> {
    simple::clear_screen()
}

pub fn draw(system_info: &crate::data::SystemInfo) -> anyhow::Result<()> {
    simple::draw(system_info)
}
