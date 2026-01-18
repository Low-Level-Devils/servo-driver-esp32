# Servo Driver

## Project Description
This Rust based driver is for controlling servo motors using an ESP32. 

## Required Hardware
- ESP32
- Servo Motor
- Jumper Wires
- 5V DC power Supply
- Breadboard
- Jumper Wires

## Connections

### ESP32
| ESP32 Pin | Connection |
| :------: | :------: |
| GND | 5V Power Supply GND |
| GPIO 2 | Servo Motor Signal Pin |

### Servo Motor
| Servo Pin | Connection |
| :------: | :------: |
| +5V | +5V from external power supply |
| GND | External Power supply GND |
| Signal | ESP32 GPIO 2 |

## Build and run
Connect the ESP32 to the host then run the following command to build, run, and monitor
```bash
cargo run --release
```