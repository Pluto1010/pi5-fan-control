use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use clap::Parser;
use colored::Colorize;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "pi5-fan-control")]
#[command(about = "Raspberry Pi 5 fan control utility", long_about = None)]
struct Args {
    /// Run as daemon, continuously controlling the fan based on temperature
    #[arg(long)]
    daemon: bool,

    /// Output status in JSON format
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum FanSpeed {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Full = 4,
}

impl FanSpeed {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(FanSpeed::Off),
            1 => Some(FanSpeed::Low),
            2 => Some(FanSpeed::Medium),
            3 => Some(FanSpeed::High),
            4 => Some(FanSpeed::Full),
            _ => None,
        }
    }
}

impl fmt::Display for FanSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FanSpeed::Off => write!(f, "Off"),
            FanSpeed::Low => write!(f, "Low"),
            FanSpeed::Medium => write!(f, "Medium"),
            FanSpeed::High => write!(f, "High"),
            FanSpeed::Full => write!(f, "Full"),
        }
    }
}

#[derive(Serialize)]
struct FanStatus {
    temperature_celsius: f32,
    speed_setting: FanSpeed,
    speed_level: u8,
    rpm: Option<u32>,
}

struct Fan {
    fan_path: &'static str,
    temp_path: &'static str,
    rpm_path: &'static str,
}

impl Fan {
    fn new() -> Self {
        Fan {
            fan_path: "/sys/class/thermal/cooling_device0/cur_state",
            temp_path: "/sys/devices/virtual/thermal/thermal_zone0/temp",
            rpm_path: "/sys/devices/platform/cooling_fan/hwmon/hwmon2/fan1_input",
        }
    }

    fn get_speed(&self) -> io::Result<FanSpeed> {
        let mut content = String::new();
        File::open(self.fan_path)?.read_to_string(&mut content)?;
        let val = content.trim().parse::<u8>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        FanSpeed::from_u8(val).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid fan speed value"))
    }

    fn set_speed(&self, speed: FanSpeed) -> io::Result<()> {
        let mut file = File::create(self.fan_path)?;
        write!(file, "{}", speed as u8)?;
        Ok(())
    }

    fn get_temp(&self) -> io::Result<f32> {
        let mut content = String::new();
        File::open(self.temp_path)?.read_to_string(&mut content)?;
        let val = content.trim().parse::<f32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(val / 1000.0)
    }

    fn get_rpm(&self) -> Option<u32> {
        // Try multiple possible hwmon paths since the number can vary
        let possible_paths = [
            self.rpm_path,
            "/sys/devices/platform/cooling_fan/hwmon/hwmon1/fan1_input",
            "/sys/devices/platform/cooling_fan/hwmon/hwmon3/fan1_input",
        ];

        for path in &possible_paths {
            if let Ok(mut file) = File::open(path) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    if let Ok(rpm) = content.trim().parse::<u32>() {
                        return Some(rpm);
                    }
                }
            }
        }
        None
    }

    fn get_status(&self) -> io::Result<FanStatus> {
        let temp = self.get_temp()?;
        let speed = self.get_speed()?;
        let rpm = self.get_rpm();

        Ok(FanStatus {
            temperature_celsius: temp,
            speed_setting: speed,
            speed_level: speed as u8,
            rpm,
        })
    }

    fn adjust_for_temp(&self) -> io::Result<()> {
        let temp = self.get_temp()?;
        
        let target_speed = if temp >= 70.0 {
            FanSpeed::Full
        } else if temp >= 65.0 {
            FanSpeed::High
        } else if temp >= 60.0 {
            FanSpeed::Medium
        } else {
            FanSpeed::Low
        };

        self.set_speed(target_speed)?;
        Ok(())
    }
}

fn check_prerequisites() {
    let paths = [
        "/sys/class/thermal/cooling_device0/cur_state",
        "/sys/devices/virtual/thermal/thermal_zone0/temp",
    ];
    
    let mut fail = false;
    for p in &paths {
        if !Path::new(p).exists() {
            eprintln!("Cannot find control interface at {}", p);
            fail = true;
        }
    }
    
    if fail {
        eprintln!("Required control interfaces are not present. Please ensure any required kernel modules are loaded.");
        std::process::exit(-1);
    }
}

fn check_root() {
    let uid = unsafe { libc::getuid() };
    if uid != 0 {
        eprintln!("fan control must be run as root");
        std::process::exit(-13);
    }
}

fn temp_color(temp: f32) -> colored::ColoredString {
    let temp_str = format!("{:.1}°C", temp);
    if temp >= 70.0 {
        temp_str.red().bold()
    } else if temp >= 60.0 {
        temp_str.yellow()
    } else if temp >= 50.0 {
        temp_str.green()
    } else {
        temp_str.cyan()
    }
}

fn speed_emoji(speed: FanSpeed) -> &'static str {
    match speed {
        FanSpeed::Off => "😴",
        FanSpeed::Low => "🌀",
        FanSpeed::Medium => "💨",
        FanSpeed::High => "🌪️",
        FanSpeed::Full => "🚀",
    }
}

fn show_status(fan: &Fan, json: bool) {
    match fan.get_status() {
        Ok(status) => {
            if json {
                println!("{}", serde_json::to_string(&status).unwrap());
            } else {
                println!();
                println!("  {} {}", "🍓".to_string(), "Pi 5 Fan Status".bold().magenta());
                println!("  {}", "─".repeat(30).dimmed());
                println!();
                println!("  {}  {}  {}", 
                    "🌡️".to_string(),
                    "Temperature:".bold(),
                    temp_color(status.temperature_celsius)
                );
                println!();
                println!("  {}  {}  {} {}",
                    speed_emoji(status.speed_setting),
                    "Fan Speed:".bold(),
                    status.speed_setting.to_string().cyan(),
                    format!("(level {})", status.speed_level).dimmed()
                );
                println!();
                match status.rpm {
                    Some(rpm) => {
                        let rpm_str = format!("{} RPM", rpm);
                        let colored_rpm = if rpm == 0 {
                            rpm_str.dimmed()
                        } else if rpm > 5000 {
                            rpm_str.yellow()
                        } else {
                            rpm_str.green()
                        };
                        println!("  {}  {}  {}", "⚡".to_string(), "RPM:".bold(), colored_rpm);
                    }
                    None => {
                        println!("  {}  {}  {}", "⚡".to_string(), "RPM:".bold(), "unavailable".dimmed());
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("{} Failed to read fan status: {}", "❌".to_string(), e.to_string().red());
            std::process::exit(1);
        }
    }
}

fn run_daemon(fan: &Fan) {
    println!("Fan control daemon started");

    loop {
        if let Err(e) = fan.adjust_for_temp() {
            eprintln!("adjust-for-temp: {}", e);
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn main() {
    let args = Args::parse();

    check_prerequisites();

    let fan = Fan::new();

    if args.daemon {
        check_root();
        run_daemon(&fan);
    } else {
        show_status(&fan, args.json);
    }
}
