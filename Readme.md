# Fan Control for Raspberry Pi 5 (Rust Version)

## Description
With Ubuntu on Raspberry Pi 5 there is no service to monitor cpu temperature and adjust the fan accordingly. This application checks the temperature every two seconds and adjusts the fan accordingly.

This is a Rust rewrite of the original Python script, offering better performance and lower resource usage.
This has been tested on Raspberry Pi 5 Model B.

## Fan control parameters

Every two seconds the service checks the cpu temperature, and compares it 
to a set of constants to determine the fan speed to set. The fan speed is represented by the FanSpeed 
enum. The values correlate to the table below. 

|Value|Fan Speed Constant | Temp |
|-----|-------------------|------|
|  4  | FanSpeed.Full     | >70°C|
|  3  | FanSpeed.High     | >65°C|
|  2  | FanSpeed.Medium   | >60°C|
|  1  | FanSpeed.Low      | >55°C|
|  0  | FanSpeed.Off      | <55°C|

### This is a chart showing a few days of statistics on my pi5. 
![fanstats-dark.png](fanstats-dark.png#gh-dark-mode-only)
![fanstats-light.png](fanstats-light.png#gh-light-mode-only)

## Requirements
* A Raspberry Pi 5
* A suitable Linux distribution
* Rust (installed automatically by setup script if missing)

## Installation 

### Notes about the setup script.
The setup script ***must*** be run on the same path as the fan-control script and fan-control.service unit file as that is the path written to the service unit file. If you wish to move these files, simply run setup.sh again. 

Clone this repository: 

```
git clone <your-repo-url>
```

Then cd to pi5-fan-control and run ./setup.sh and follow the prompts. 


```
cd pi5-fan-control
sudo ./setup.sh
```

# Inspiration and Credit where Due
This is a fork and rewrite of [Nicole Stevens' pi5-fan-control](https://github.com/nicciniamh/pi5-fan-control), which was inspired by [James Ashley's Pi 5 Fan Controller](https://gist.github.com/James-Ansley/32f72729487c8f287a801abcc7a54f38).

# Why I wrote a new version
I wanted to rewrite the tool in Rust for better efficiency and to learn Rust on embedded devices.

# License 
This code is covered by the MIT license. Please when attributing, also attribute James Ashley and Nicole Stevens. 
