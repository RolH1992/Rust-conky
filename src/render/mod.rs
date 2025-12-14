use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

pub fn draw(system_info: &crate::data::SystemInfo) -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    // Move cursor to top-left and clear from cursor to end of screen
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    // Header
    println!("=== Rust Conky System Monitor ===");
    println!();

    // CPU Information
    let cpu_usage = system_info.cpu_usage();
    let cpu_count = system_info.cpu_count();
    let load_avg = system_info.load_average();

    println!("CPU: {:.1}% ({} cores)", cpu_usage, cpu_count);
    println!(
        "Load Average: {:.2}, {:.2}, {:.2}",
        load_avg.0, load_avg.1, load_avg.2
    );
    println!();

    // Memory Information
    let (used_mem, total_mem) = system_info.memory_usage();
    let (used_swap, total_swap) = system_info.swap_usage();
    let used_mem_gb = used_mem as f64 / 1024.0 / 1024.0 / 1024.0;
    let total_mem_gb = total_mem as f64 / 1024.0 / 1024.0 / 1024.0;
    let mem_percentage = (used_mem as f64 / total_mem as f64) * 100.0;

    println!(
        "Memory: {:.2}GB / {:.2}GB ({:.1}%)",
        used_mem_gb, total_mem_gb, mem_percentage
    );

    if total_swap > 0 {
        let used_swap_gb = used_swap as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_swap_gb = total_swap as f64 / 1024.0 / 1024.0 / 1024.0;
        let swap_percentage = (used_swap as f64 / total_swap as f64) * 100.0;

        println!(
            "Swap:   {:.2}GB / {:.2}GB ({:.1}%)",
            used_swap_gb, total_swap_gb, swap_percentage
        );
    }
    println!();

    // Disk Information
    let disk_stats = system_info.disk_stats();
    if !disk_stats.is_empty() {
        println!("Disks:");

        for (name, total, available, mount_point) in disk_stats {
            let used = total - available;
            let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;
            let total_gb = total as f64 / 1024.0 / 1024.0 / 1024.0;
            let percentage = (used as f64 / total as f64) * 100.0;

            println!(
                "  {} ({}) {:.1}GB / {:.1}GB ({:.1}%)",
                name, mount_point, used_gb, total_gb, percentage
            );
        }
        println!();
    }

    // Network Information
    let network_stats = system_info.network_stats();
    if !network_stats.is_empty() {
        println!("Network Interfaces:");

        for (interface, received, transmitted) in network_stats {
            let received_mb = received as f64 / 1024.0 / 1024.0;
            let transmitted_mb = transmitted as f64 / 1024.0 / 1024.0;

            println!(
                "  {}: ↓ {:.2}MB ↑ {:.2}MB",
                interface, received_mb, transmitted_mb
            );
        }
        println!();
    }

    // Top Processes
    let top_processes = system_info.top_processes(5);
    if !top_processes.is_empty() {
        println!("Top Processes (by CPU):");

        for (name, pid, cpu, memory) in top_processes {
            let memory_mb = memory as f64 / 1024.0 / 1024.0;
            println!("  {:6} {:.1}% {:.1}MB {}", pid, cpu, memory_mb, name);
        }
        println!();
    }

    // System Uptime
    let uptime = system_info.uptime();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    println!("Uptime: {} hours, {} minutes", hours, minutes);

    stdout.flush()?;
    Ok(())
}

pub fn clear_screen() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.flush()?;
    Ok(())
}
