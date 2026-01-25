// src/render/tui.rs
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
    Terminal,
};
use std::{io, time::Duration};

use crate::data::SystemInfo;

/// Launch the TUI interface
pub async fn launch_tui(update_interval: u64) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = TuiApp::new(update_interval);

    // Main TUI loop
    let result = run_tui_loop(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result?;
    Ok(())
}

async fn run_tui_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut TuiApp,
) -> Result<()> {
    loop {
        terminal.draw(|frame| app.draw(frame))?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.toggle_pause(),
                        KeyCode::Char('r') => app.force_refresh(),
                        KeyCode::Tab => app.next_section(),
                        KeyCode::Up => app.scroll_up(),
                        KeyCode::Down => app.scroll_down(),
                        _ => {}
                    }
                }
            }
        }

        // Update data if not paused
        if !app.paused {
            app.update();
            tokio::time::sleep(Duration::from_secs(app.update_interval)).await;
        } else {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    Ok(())
}

/// TUI Application State
struct TuiApp {
    system_info: SystemInfo,
    paused: bool,
    selected_section: usize,
    update_interval: u64,
    process_scroll: usize,
    max_processes: usize,
}

impl TuiApp {
    fn new(update_interval: u64) -> Self {
        Self {
            system_info: SystemInfo::new(),
            paused: false,
            selected_section: 0,
            update_interval,
            process_scroll: 0,
            max_processes: 10,
        }
    }

    fn update(&mut self) {
        self.system_info.refresh();
    }

    fn force_refresh(&mut self) {
        self.system_info.refresh();
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn next_section(&mut self) {
        self.selected_section = (self.selected_section + 1) % 6; // 6 sections
    }

    fn scroll_up(&mut self) {
        if self.process_scroll > 0 {
            self.process_scroll -= 1;
        }
    }

    fn scroll_down(&mut self) {
        self.process_scroll += 1;
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        // Main layout - matches your old screenshot
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // CPU
                Constraint::Length(3), // RAM
                Constraint::Length(3), // SWAP
                Constraint::Length(6), // DISKS
                Constraint::Length(4), // NETWORK
                Constraint::Min(10),   // TOP PROCESSES
                Constraint::Length(1), // Status bar
            ])
            .split(frame.area());

        self.draw_cpu(frame, chunks[0]);
        self.draw_memory(frame, chunks[1], "RAM");
        self.draw_memory(frame, chunks[2], "SWAP");
        self.draw_disks(frame, chunks[3]);
        self.draw_network(frame, chunks[4]);
        self.draw_processes(frame, chunks[5]);
        self.draw_status_bar(frame, chunks[6]);
    }

    fn draw_cpu(&self, frame: &mut ratatui::Frame, area: Rect) {
        let cpu_usage = self.system_info.cpu_usage();
        let load_avg = self.system_info.load_average();

        let block = Block::default()
            .title(" CPU ")
            .borders(Borders::ALL)
            .border_style(if self.selected_section == 0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        // Create gauge with custom bar
        let gauge = Gauge::default()
            .block(block)
            .gauge_style(Style::default().fg(Color::Green))
            .percent(cpu_usage as u16)
            .label(format!(" {:.1}%", cpu_usage));

        frame.render_widget(gauge, area);

        // Add load average as text below the gauge
        let load_text = format!(
            "Cores: {} Load: {:.2} {:.2} {:.2}",
            self.system_info.cpu_count(),
            load_avg.0,
            load_avg.1,
            load_avg.2
        );

        let load_rect = Rect {
            x: area.x + 2,
            y: area.y + 1,
            width: area.width.saturating_sub(4),
            height: 1,
        };

        let load_paragraph = Paragraph::new(load_text).style(Style::default().fg(Color::Gray));

        frame.render_widget(load_paragraph, load_rect);
    }

    fn draw_memory(&self, frame: &mut ratatui::Frame, area: Rect, title: &str) {
        let (used, total, percent) = if title == "RAM" {
            let (used_mem, total_mem) = self.system_info.memory_usage();
            let percent = if total_mem > 0 {
                (used_mem as f64 / total_mem as f64 * 100.0) as f32
            } else {
                0.0
            };
            (used_mem, total_mem, percent)
        } else {
            let (used_swap, total_swap) = self.system_info.swap_usage();
            let percent = if total_swap > 0 {
                (used_swap as f64 / total_swap as f64 * 100.0) as f32
            } else {
                0.0
            };
            (used_swap, total_swap, percent)
        };

        let section_index = if title == "RAM" { 1 } else { 2 };

        let block = Block::default()
            .title(format!(" {} ", title))
            .borders(Borders::ALL)
            .border_style(if self.selected_section == section_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_gb = total as f64 / 1024.0 / 1024.0 / 1024.0;

        let gauge = Gauge::default()
            .block(block)
            .gauge_style(Style::default().fg(Color::Blue))
            .percent(percent as u16)
            .label(format!(" {:.1}%", percent));

        frame.render_widget(gauge, area);

        // Add memory usage text
        let mem_text = format!("Used: {:.1}G / {:.1}G", used_gb, total_gb);
        let text_rect = Rect {
            x: area.x + 2,
            y: area.y + 1,
            width: area.width.saturating_sub(4),
            height: 1,
        };

        let mem_paragraph = Paragraph::new(mem_text).style(Style::default().fg(Color::Gray));

        frame.render_widget(mem_paragraph, text_rect);
    }

    fn draw_disks(&self, frame: &mut ratatui::Frame, area: Rect) {
        let block = Block::default()
            .title(" DISKS ")
            .borders(Borders::ALL)
            .border_style(if self.selected_section == 3 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let disk_stats = self.system_info.disk_stats();
        let mut lines = vec![];

        for (_name, total, available, mount_point) in disk_stats.iter().take(2) {
            // Show 2 disks max
            let used = total.saturating_sub(*available);
            let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;
            let _total_gb = *total as f64 / 1024.0 / 1024.0 / 1024.0;
            let percent = if *total > 0 {
                (used as f64 / *total as f64 * 100.0) as u16
            } else {
                0
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("{} {:.1}%", mount_point, percent),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(" "),
                Span::styled(format!("{:.1}G", used_gb), Style::default().fg(Color::Gray)),
            ]);
            lines.push(line);
        }

        let paragraph = Paragraph::new(lines).block(block).style(Style::default());

        frame.render_widget(paragraph, area);
    }

    fn draw_network(&self, frame: &mut ratatui::Frame, area: Rect) {
        let block = Block::default()
            .title(" NETWORK ")
            .borders(Borders::ALL)
            .border_style(if self.selected_section == 4 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let network_stats = self.system_info.network_stats();
        let mut lines = vec![];

        for (interface, received, transmitted) in network_stats.iter().take(2) {
            // Show 2 interfaces max
            let received_mb = *received as f64 / 1024.0 / 1024.0;
            let transmitted_mb = *transmitted as f64 / 1024.0 / 1024.0;

            let line = Line::from(vec![
                Span::styled(
                    format!("{} ", interface),
                    Style::default().fg(Color::Magenta),
                ),
                Span::styled("↓", Style::default().fg(Color::Blue)),
                Span::raw(format!("{:.1}M ", received_mb)),
                Span::styled("↑", Style::default().fg(Color::Green)),
                Span::raw(format!("{:.1}M", transmitted_mb)),
            ]);
            lines.push(line);
        }

        let paragraph = Paragraph::new(lines).block(block).style(Style::default());

        frame.render_widget(paragraph, area);
    }

    fn draw_processes(&self, frame: &mut ratatui::Frame, area: Rect) {
        let block = Block::default()
            .title(" TOP PROCESSES ")
            .borders(Borders::ALL)
            .border_style(if self.selected_section == 5 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let top_processes = self.system_info.top_processes(10);
        let header = Row::new(vec![
            Cell::from("PID"),
            Cell::from("CPU%"),
            Cell::from("MEM"),
            Cell::from("NAME"),
        ])
        .style(Style::default().fg(Color::Yellow));

        let rows: Vec<Row> = top_processes
            .iter()
            .skip(self.process_scroll)
            .take(self.max_processes)
            .map(|(name, pid, cpu, memory)| {
                let memory_mb = *memory as f64 / 1024.0 / 1024.0;
                Row::new(vec![
                    Cell::from(pid.to_string()),
                    Cell::from(format!("{:.1}%", cpu)),
                    Cell::from(format!("{:.0}M", memory_mb)),
                    Cell::from(name.clone()),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Min(20),
        ];

        let table = Table::new(rows, widths).header(header).block(block);

        frame.render_widget(table, area);
    }

    fn draw_status_bar(&self, frame: &mut ratatui::Frame, area: Rect) {
        let status = if self.paused {
            "PAUSED - Press SPACE to resume"
        } else {
            "Q:Quit | SPACE:Pause | TAB:Navigate | R:Refresh | ↑↓:Scroll"
        };

        let status_line = Line::from(vec![Span::styled(
            status,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        )]);

        let paragraph = Paragraph::new(status_line).block(Block::default());

        frame.render_widget(paragraph, area);
    }
}
