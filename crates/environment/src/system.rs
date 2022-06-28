use std::sync::Mutex;

use lazy_static::lazy_static;
use remote::system_statistics::SystemStatsReponse;
use sysinfo::{CpuExt, System, SystemExt};

lazy_static! {
    static ref SYSTEM: Mutex<System> = Mutex::new(System::new_all());
}

pub fn get_system_stats() -> SystemStatsReponse {
    let system = SYSTEM.lock().expect("Could not lock SYSTEM");

    SystemStatsReponse {
        version: system.os_version().unwrap_or("unknown".to_string()),
        kernel_version: system.kernel_version().unwrap_or("unknown".to_string()),
        architecture: std::env::consts::ARCH.to_string(),
        os: system.long_os_version().unwrap_or("unknown".to_string()),
        cpu: system.global_cpu_info().brand().to_string(),
    }
}
