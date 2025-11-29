use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FanSpeed {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Full = 4,
}

impl FanSpeed {
    #[allow(dead_code)]
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

struct Fan {
    fan_path: &'static str,
    temp_path: &'static str,
}

impl Fan {
    fn new() -> Self {
        Fan {
            fan_path: "/sys/class/thermal/cooling_device0/cur_state",
            temp_path: "/sys/devices/virtual/thermal/thermal_zone0/temp",
        }
    }

    #[allow(dead_code)]
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

    fn adjust_for_temp(&self) -> io::Result<()> {
        let temp = self.get_temp()?;
        
        let target_speed = if temp >= 70.0 {
            FanSpeed::Full
        } else if temp >= 65.0 {
            FanSpeed::High
        } else if temp >= 60.0 {
            FanSpeed::Medium
        } else if temp >= 55.0 {
            FanSpeed::Low
        } else {
            FanSpeed::Off
        };

        self.set_speed(target_speed)?;
        Ok(())
    }
}

fn main() {
    // Check paths
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

    // Check root
    let uid = unsafe { libc::getuid() };
    if uid != 0 {
        eprintln!("fan control must be run as root");
        std::process::exit(-13);
    }

    let fan = Fan::new();
    println!("Fan control started");

    loop {
        if let Err(e) = fan.adjust_for_temp() {
            eprintln!("adjust-for-temp: {}", e);
        }
        thread::sleep(Duration::from_secs(2));
    }
}
