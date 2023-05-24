use chrono::Local;
use raw_cpuid::CpuId;
use sysinfo::SystemExt;

const MHZ_TO_HZ: u64 = 1000000;

#[derive(Debug)]
pub struct SystemInfo {
    pub memory_usage: u64,
    pub max_memory: u64,
    pub username: String,
    pub hostname: String,
    pub current_time: String,
    pub os: String,
    pub cpu: String,
    pub cpu_frequency: String,
}

pub fn get_system_info() -> SystemInfo {
    let cpuid = CpuId::new();
    let mut sys: sysinfo::System = <sysinfo::System as sysinfo::SystemExt>::new_all();
    sys.refresh_all();

    let memory_usage: u64 = sys.used_memory();
    let max_memory: u64 = sys.total_memory();
    let username: String = whoami::username();
    let hostname = whoami::hostname();
    let current_time: String = Local::now().time().format("%H:%M:%S").to_string();
    let os: String = std::env::consts::OS.to_string();
    let cpu: String = cpuid
        .get_processor_brand_string()
        .as_ref()
        .map_or_else(|| "n/a", |pbs| pbs.as_str())
        .to_string();
    let tsc_frequency_hz: Option<_> = cpuid.get_tsc_info().map(|tinfo| {
        if tinfo.nominal_frequency() != 0 {
            tinfo.tsc_frequency()
        } else if tinfo.numerator() != 0 && tinfo.denominator() != 0 {
            // Skylake and Kabylake don't report the crystal clock, approximate with base frequency:
            cpuid
                .get_processor_frequency_info()
                .map(|pinfo| pinfo.processor_base_frequency() as u64 * MHZ_TO_HZ)
                .map(|cpu_base_freq_hz| {
                    let crystal_hz =
                        cpu_base_freq_hz * tinfo.denominator() as u64 / tinfo.numerator() as u64;
                    crystal_hz * tinfo.numerator() as u64 / tinfo.denominator() as u64
                })
        } else {
            None
        }
    });
    let cpu_frequency: String = match tsc_frequency_hz {
        Some(x) => format!("{} Hz", x.unwrap_or(0)),
        None => String::from("unknown"),
    };

    SystemInfo {
        hostname,
        memory_usage,
        max_memory,
        username,
        current_time,
        os,
        cpu,
        cpu_frequency,
    }
}
