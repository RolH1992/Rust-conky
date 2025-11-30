use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, Networks, ProcessRefreshKind, RefreshKind, System,
};

pub struct SystemInfo {
    system: System,
    networks: Networks,
    disks: Disks,
}

impl SystemInfo {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::everything()),
        );

        Self {
            system,
            networks: Networks::new_with_refreshed_list(),
            disks: Disks::new_with_refreshed_list(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.system.refresh_processes();
        self.networks.refresh();
        self.disks.refresh();
    }

    // CPU
    pub fn cpu_usage(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage()
    }

    pub fn cpu_count(&self) -> usize {
        self.system.cpus().len()
    }

    // Memory
    pub fn memory_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }

    pub fn swap_usage(&self) -> (u64, u64) {
        (self.system.used_swap(), self.system.total_swap())
    }

    // Network
    pub fn network_stats(&self) -> Vec<(String, u64, u64)> {
        self.networks
            .iter()
            .map(|(interface, data)| (interface.clone(), data.received(), data.transmitted()))
            .collect()
    }

    // Disk
    pub fn disk_stats(&self) -> Vec<(String, u64, u64, String)> {
        self.disks
            .iter()
            .map(|disk| {
                (
                    disk.name().to_string_lossy().to_string(),
                    disk.total_space(),
                    disk.available_space(),
                    disk.mount_point().to_string_lossy().to_string(),
                )
            })
            .collect()
    }

    // Processes
    pub fn top_processes(&self, count: usize) -> Vec<(String, u32, f32, u64)> {
        let mut processes: Vec<_> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                (
                    process.name().to_string(),
                    pid.as_u32(),
                    process.cpu_usage(),
                    process.memory(),
                )
            })
            .collect();

        // Sort by CPU usage descending
        processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        processes.into_iter().take(count).collect()
    }

    // System info
    pub fn uptime(&self) -> u64 {
        System::uptime()
    }

    pub fn load_average(&self) -> (f64, f64, f64) {
        let load_avg = System::load_average();
        (load_avg.one, load_avg.five, load_avg.fifteen)
    }
}
