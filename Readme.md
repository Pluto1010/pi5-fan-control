# Fan Control for Raspberry Pi 5 (Rust Version)

## Description
With Ubuntu on Raspberry Pi 5 there is no service to monitor cpu temperature and adjust the fan accordingly. This application checks the temperature every two seconds and adjusts the fan accordingly.

This is a Rust rewrite of the original Python script, offering better performance and lower resource usage.
This has been tested on Raspberry Pi 5 Model B.

## Usage

### Status Display (Default)
Running the binary without arguments displays the current fan status with a colorful, modern output:

```
$ rpi-fan-control

  🍓 Pi 5 Fan Status
  ──────────────────────────────────

  🌡️  Temperature:  45.2°C

  🌀  Fan Speed:  Low (level 1)

  ⚡  RPM:  1400 RPM
```

### JSON Output
For scripting or monitoring integration, use the `--json` flag:

```
$ rpi-fan-control --json
{"temperature_celsius":45.2,"speed_setting":"off","speed_level":0,"rpm":0}
```

### Daemon Mode
Run as a background service that continuously monitors temperature and adjusts fan speed:

```
$ sudo rpi-fan-control --daemon
Fan control daemon started
```

This is the mode used by the systemd service.

## Fan control parameters

Every two seconds the daemon checks the cpu temperature, and compares it 
to a set of constants to determine the fan speed to set. The fan speed is represented by the FanSpeed 
enum. The values correlate to the table below. 

|Value|Fan Speed Constant | Temp |Emoji|
|-----|-------------------|------|-----|
|  4  | FanSpeed.Full     | ≥70°C| 🚀  |
|  3  | FanSpeed.High     | ≥65°C| 🌪️  |
|  2  | FanSpeed.Medium   | ≥60°C| 💨  |
|  1  | FanSpeed.Low      | <60°C (minimum daemon speed)| 🌀  |
|  0  | FanSpeed.Off      | Not used by daemon | 😴  |

### This is a chart showing a few days of statistics on my pi5. 
![fanstats-dark.png](fanstats-dark.png#gh-dark-mode-only)
![fanstats-light.png](fanstats-light.png#gh-light-mode-only)

## Requirements
* A Raspberry Pi 5
* A suitable Linux distribution
* Rust (installed automatically by setup script if missing)

## Installation 

Clone this repository: 

```
git clone <your-repo-url>
cd pi5-fan-control
```

### Quick Setup

Run the setup script to install build dependencies:

```
sudo ./setup.sh
```

Then build and install:

```
make build
sudo make install
sudo make install-service
```

### Start the Service

```
sudo systemctl enable --now rpi-fan-control
```

### Check Status

```
# View service status
sudo systemctl status rpi-fan-control

# View current fan status (colorful output)
rpi-fan-control

# View current fan status (JSON)
rpi-fan-control --json
```

# Inspiration and Credit where Due
This is a fork and rewrite of [Nicole Stevens' pi5-fan-control](https://github.com/nicciniamh/pi5-fan-control), which was inspired by [James Ashley's Pi 5 Fan Controller](https://gist.github.com/James-Ansley/32f72729487c8f287a801abcc7a54f38).

# Why I wrote a new version
I wanted to rewrite the tool in Rust for better efficiency and to learn Rust on embedded devices.

# License 
This code is covered by the MIT license. Please when attributing, also attribute James Ashley and Nicole Stevens. 
