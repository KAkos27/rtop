use sysinfo::{Disks, System};

pub struct DiskInformation {
    pub name: String,
    pub percent: f64,
}

pub struct SystemInformation {
    pub cpu: f64,
    pub memory: f64,
    pub disk: Vec<DiskInformation>,
}

fn get_cpu_percentage(sys: &System) -> f64 {
    return sys.global_cpu_usage() as f64;
}

fn get_memory_percentage(sys: &System) -> f64 {
    let total_memory: u64 = sys.total_memory();
    let used_memory: u64 = sys.used_memory();
    return used_memory as f64 / total_memory as f64 * 100.0;
}

fn get_disks_info() -> Vec<DiskInformation> {
    let disks: Disks = Disks::new_with_refreshed_list();
    let mut result: Vec<DiskInformation> = Vec::new();

    for current_disk in disks.iter() {
        let total_disk: u64 = current_disk.total_space();
        let used_disk: u64 = total_disk - current_disk.available_space();
        let percent: f64 = used_disk as f64 / total_disk as f64 * 100.0;

        result.push(DiskInformation {
            name: format!(
                "{} ({})",
                current_disk.name().to_string_lossy(),
                current_disk.mount_point().to_string_lossy()
            ),
            percent,
        });
    }

    result
}

fn get_system_info() -> SystemInformation {
    let mut sys: System = System::new_all();
    sys.refresh_all();

    return SystemInformation {
        cpu: get_cpu_percentage(&sys),
        memory: get_memory_percentage(&sys),
        disk: get_disks_info(),
    };
}

impl SystemInformation {
    pub fn init() -> Self {
        get_system_info()
    }
}
