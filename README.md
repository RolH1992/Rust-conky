🛡️ ClamAV TUI Scanner

A modern Terminal User Interface (TUI) for ClamAV antivirus scans. This tool provides a real-time, interactive dashboard to monitor and control system-wide virus scans with ease.

https://screenshot.png
✨ Features

    Real-time Scan Monitoring - Watch scans progress in real-time with live output

    Interactive Dashboard - Clean, tabbed interface showing scan status and progress

    Virus Detection - Automatically captures and displays detected threats

    Database Updates - One-key virus definition updates via freshclam

    Progress Tracking - Visual progress bar with file count tracking

    Multi-view Interface - Switch between Dashboard, Output, Viruses, and Summary views

    Live Output - See scan output as it happens, with color-coded messages

📋 Prerequisites

    ClamAV must be installed on your system

    Rust (for building from source)

    sudo access (required for system-wide scans)

Install ClamAV

Ubuntu/Debian:
bash

sudo apt update
sudo apt install clamav clamav-daemon

Arch Linux:
bash

sudo pacman -S clamav

Fedora:
bash

sudo dnf install clamav clamav-update

🚀 Installation
Option 1: Build from Source
bash

# Clone the repository
git clone https://github.com/yourusername/clam-tui.git
cd clam-tui

# Build the release version
cargo build --release
The binary will be at ./target/release/clam-tui
