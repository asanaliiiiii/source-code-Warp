// src/big_functions/sysinfo.rs
use sysinfo::System;

pub fn fetch_system_specs() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Получаем количество ядер без импорта CpuExt
    let cores = sys.cpus().len();
    
    format!(
        "📊 [Warp OS Tracker Profile]\n  - CPU Cores  : {}\n  - RAM Total  : {} MB",
        cores,
        sys.total_memory() / 1024 / 1024
    )
}