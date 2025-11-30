use crate::widgets::Widget;
use crate::data::SystemInfo;

impl Widget for super::CpuWidget {
    fn render(&self, system_info: &SystemInfo) -> String {
        format!("CPU: {:.1}%", system_info.cpu_usage())
    }
}

impl Widget for super::MemoryWidget {
    fn render(&self, system_info: &SystemInfo) -> String {
        let (used, total) = system_info.memory_usage();
        let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_gb = total as f64 / 1024.0 / 1024.0 / 1024.0;
        
        format!("Memory: {:.2}GB / {:.2}GB", used_gb, total_gb)
    }
}
