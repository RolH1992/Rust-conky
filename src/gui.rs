use eframe::egui;
use std::time::{Duration, Instant};

pub struct GuiApp {
    system_info: crate::data::SystemInfo,
    last_update: Instant,
    update_interval: Duration,
}

impl GuiApp {
    pub fn new(update_interval: u64) -> Self {
        Self {
            system_info: crate::data::SystemInfo::new(),
            last_update: Instant::now(),
            update_interval: Duration::from_secs(update_interval),
        }
    }

    fn update_if_needed(&mut self) {
        if self.last_update.elapsed() >= self.update_interval {
            self.system_info.refresh();
            self.last_update = Instant::now();
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_if_needed();

        // Set retro terminal theme with simple visuals
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120), // More transparent
            panel_fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
            window_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 0)),
            widgets: egui::style::Widgets::default(), // Use default widgets
            ..egui::Visuals::dark()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // 8-bit style header
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(5.0);
                ui.monospace("╔══════════════╗");
                ui.monospace("║ RUST CONKY   ║");
                ui.monospace("╚══════════════╝");
                ui.add_space(3.0);
            });

            // Main content with very transparent background - fixed width
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80)) // Very transparent
                .inner_margin(egui::Margin::symmetric(12, 8)) // Reduced horizontal padding
                .show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(2.0, 1.0);

                    // CPU Section - Green terminal style
                    ui.push_id("cpu", |ui| {
                        ui.horizontal(|ui| {
                            ui.monospace("CPU:");
                            colored_monospace(
                                ui,
                                retro_green(),
                                &format!("{:5.1}%", self.system_info.cpu_usage()),
                            );
                            ui.monospace(format!("[{} cores]", self.system_info.cpu_count()));
                        });

                        ui.horizontal(|ui| {
                            ui.monospace("LOAD:");
                            ui.monospace(format!(
                                "{:.1} {:.1} {:.1}",
                                self.system_info.load_average().0,
                                self.system_info.load_average().1,
                                self.system_info.load_average().2
                            ));
                        });

                        // ASCII progress bar
                        add_ascii_progress_bar(
                            ui,
                            self.system_info.cpu_usage() / 100.0,
                            retro_green(),
                        );
                    });

                    ui.add_space(2.0);
                    ui.monospace("────────────────");
                    ui.add_space(2.0);

                    // Memory Section - Cyan terminal style
                    ui.push_id("memory", |ui| {
                        let (used_mem, total_mem) = self.system_info.memory_usage();
                        let (used_swap, total_swap) = self.system_info.swap_usage();
                        let mem_percentage = (used_mem as f64 / total_mem as f64) * 100.0;
                        let used_gb = used_mem as f64 / 1024.0 / 1024.0 / 1024.0;
                        let total_gb = total_mem as f64 / 1024.0 / 1024.0 / 1024.0;

                        ui.horizontal(|ui| {
                            ui.monospace("RAM:");
                            colored_monospace(
                                ui,
                                retro_cyan(),
                                &format!("{:5.1}%", mem_percentage),
                            );
                            ui.monospace(format!("{:4.1}/{:4.1}G", used_gb, total_gb));
                        });

                        add_ascii_progress_bar(ui, mem_percentage as f32 / 100.0, retro_cyan());

                        // Swap memory if available
                        if total_swap > 0 {
                            let swap_percentage = (used_swap as f64 / total_swap as f64) * 100.0;
                            let used_swap_gb = used_swap as f64 / 1024.0 / 1024.0 / 1024.0;
                            let total_swap_gb = total_swap as f64 / 1024.0 / 1024.0 / 1024.0;

                            ui.horizontal(|ui| {
                                ui.monospace("SWAP:");
                                colored_monospace(
                                    ui,
                                    retro_blue(),
                                    &format!("{:5.1}%", swap_percentage),
                                );
                                ui.monospace(format!(
                                    "{:4.1}/{:4.1}G",
                                    used_swap_gb, total_swap_gb
                                ));
                            });

                            add_ascii_progress_bar(
                                ui,
                                swap_percentage as f32 / 100.0,
                                retro_blue(),
                            );
                        }
                    });

                    ui.add_space(2.0);
                    ui.monospace("────────────────");
                    ui.add_space(2.0);

                    // Disk Section - Yellow terminal style
                    ui.push_id("disk", |ui| {
                        let disk_stats = self.system_info.disk_stats();
                        if !disk_stats.is_empty() {
                            ui.monospace("DISKS:");

                            for (_name, total, available, mount_point) in disk_stats.iter().take(2)
                            {
                                let used = *total - *available;
                                let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;
                                let percentage = (used as f64 / *total as f64) * 100.0;

                                ui.horizontal(|ui| {
                                    ui.monospace(format!(
                                        "{}:",
                                        shorten_mount_point(&mount_point, 8)
                                    ));
                                    colored_monospace(
                                        ui,
                                        retro_yellow(),
                                        &format!("{:4.1}%", percentage),
                                    );
                                    ui.monospace(format!("{:4.1}G", used_gb));
                                });

                                add_ascii_progress_bar(
                                    ui,
                                    percentage as f32 / 100.0,
                                    retro_yellow(),
                                );
                            }
                        }
                    });

                    ui.add_space(2.0);
                    ui.monospace("────────────────");
                    ui.add_space(2.0);

                    // Network Section - Magenta terminal style
                    ui.push_id("network", |ui| {
                        let network_stats = self.system_info.network_stats();
                        if !network_stats.is_empty() {
                            ui.monospace("NET:");

                            for (interface, received, transmitted) in network_stats.iter().take(1) {
                                let received_mb = *received as f64 / 1024.0 / 1024.0;
                                let transmitted_mb = *transmitted as f64 / 1024.0 / 1024.0;

                                ui.horizontal(|ui| {
                                    ui.monospace(format!("{}:", shorten_interface(&interface, 6)));
                                    colored_monospace(
                                        ui,
                                        retro_magenta(),
                                        &format!("↓{:4.1}M", received_mb),
                                    );
                                    colored_monospace(
                                        ui,
                                        retro_magenta(),
                                        &format!("↑{:4.1}M", transmitted_mb),
                                    );
                                });
                            }
                        }
                    });

                    ui.add_space(2.0);
                    ui.monospace("────────────────");
                    ui.add_space(2.0);

                    // Processes Section - Red terminal style
                    ui.push_id("processes", |ui| {
                        let top_processes = self.system_info.top_processes(4);
                        if !top_processes.is_empty() {
                            ui.monospace("PROC:");

                            for (name, pid, cpu, memory) in top_processes {
                                let memory_mb = memory as f64 / 1024.0 / 1024.0;

                                ui.horizontal(|ui| {
                                    ui.monospace(format!("{:>4}", pid));
                                    colored_monospace(ui, retro_red(), &format!("{:3.0}%", cpu));
                                    ui.monospace(format!("{:3.0}M", memory_mb));
                                    ui.monospace(shorten_name(&name, 8));
                                });
                            }
                        }
                    });

                    ui.add_space(2.0);
                    ui.monospace("────────────────");
                    ui.add_space(2.0);

                    // System Info - White terminal style
                    ui.push_id("system", |ui| {
                        let uptime = self.system_info.uptime();
                        let hours = uptime / 3600;
                        let minutes = (uptime % 3600) / 60;

                        ui.horizontal(|ui| {
                            ui.monospace("UP:");
                            ui.monospace(format!("{:3}h{:2}m", hours, minutes));
                        });

                        ui.horizontal(|ui| {
                            ui.monospace("RATE:");
                            ui.monospace(format!("{}s", self.update_interval.as_secs()));
                        });
                    });
                });
        });

        // Request repaint for continuous updates
        ctx.request_repaint_after(self.update_interval);
    }
}

// Helper function for colored monospace text
fn colored_monospace(ui: &mut egui::Ui, color: egui::Color32, text: &str) {
    ui.add(egui::Label::new(
        egui::RichText::new(text).monospace().color(color),
    ));
}

// ASCII progress bar function
fn add_ascii_progress_bar(ui: &mut egui::Ui, progress: f32, color: egui::Color32) {
    let width = 16; // Reduced from 20 to save space
    let filled = (progress * width as f32).round() as usize;
    let empty = width - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

    ui.horizontal(|ui| {
        colored_monospace(ui, color, &bar);
        ui.monospace(format!("{:2.0}%", progress * 100.0));
    });
}

// Helper function to shorten mount points
fn shorten_mount_point(mount_point: &str, max_length: usize) -> String {
    if mount_point == "/" {
        "root".to_string()
    } else if mount_point.len() > max_length {
        let parts: Vec<&str> = mount_point.split('/').collect();
        if let Some(last) = parts.last() {
            if !last.is_empty() {
                return last[..std::cmp::min(last.len(), max_length)].to_string();
            }
        }
        mount_point[..max_length].to_string()
    } else {
        mount_point.to_string()
    }
}

// Helper function to shorten interface names
fn shorten_interface(interface: &str, max_length: usize) -> String {
    if interface.len() > max_length {
        interface[..max_length].to_string()
    } else {
        interface.to_string()
    }
}

// Helper function to shorten process names
fn shorten_name(name: &str, max_length: usize) -> String {
    if name.len() > max_length {
        format!("{}...", &name[..max_length - 3])
    } else {
        name.to_string()
    }
}

// Retro color palette
fn retro_green() -> egui::Color32 {
    egui::Color32::from_rgb(0, 255, 0)
}

fn retro_cyan() -> egui::Color32 {
    egui::Color32::from_rgb(0, 255, 255)
}

fn retro_blue() -> egui::Color32 {
    egui::Color32::from_rgb(0, 100, 255)
}

fn retro_yellow() -> egui::Color32 {
    egui::Color32::from_rgb(255, 255, 0)
}

fn retro_magenta() -> egui::Color32 {
    egui::Color32::from_rgb(255, 0, 255)
}

fn retro_red() -> egui::Color32 {
    egui::Color32::from_rgb(255, 50, 50)
}
