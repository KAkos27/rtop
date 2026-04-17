use sysinfo::{Disks, Pid, System};

pub struct ProcessInformation {
    pub pid: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

pub struct CpuInfromation {
    pub percentage: u16,
    pub cores: Vec<f32>,
}

pub struct DiskInformation {
    pub name: String,
    pub percent: f64,
}

pub struct SystemInformation {
    pub cpu_information: CpuInfromation,
    pub memory: f64,
    pub disk: Vec<DiskInformation>,
    pub processes: Vec<ProcessInformation>,
}

fn get_cpu_info(sys: &System) -> CpuInfromation {
    CpuInfromation {
        percentage: sys.global_cpu_usage() as u16,
        cores: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    }
}

fn get_memory_percentage(sys: &System) -> f64 {
    let total_memory: u64 = sys.total_memory();
    let used_memory: u64 = sys.used_memory();
    return used_memory as f64 / total_memory as f64 * 100.0;
}

fn get_disks_info(disks: &Disks) -> Vec<DiskInformation> {
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

fn get_process_info(sys: &System) -> Vec<ProcessInformation> {
    sys.processes()
        .iter()
        .map(|(pid, process)| ProcessInformation {
            pid: *pid,
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
        })
        .collect()
}

impl SystemInformation {
    pub fn get_system_info(sys: &System, disks: &Disks) -> Self {
        SystemInformation {
            cpu_information: get_cpu_info(sys),
            memory: get_memory_percentage(sys),
            disk: get_disks_info(disks),
            processes: get_process_info(sys),
        }
    }
}
