# weirui_std_rs

A Rust standard library for robot control using WebAssembly host functions in the Weirui robot operating system.

## Overview

`weirui_std_rs` is a Rust library that provides a high-level interface for controlling robots in the Weirui operating system. It allows WebAssembly modules to communicate with the host environment to perform operations such as servo control, network requests, and I/O operations.

The library implements a unified result system and uses a radian-based coordinate system as specified in the Weirui documentation.

## Features

- **Servo Control**: High-level functions for controlling robot actuators
- **Network Operations**: Make HTTP requests from WebAssembly modules
- **I/O Operations**: Read input from the console
- **MCP Integration**: Connect and interact with MCP services
- **Error Handling**: Unified error system with proper error propagation
- **Angle Utilities**: Conversion between degrees and radians with normalization
- **Memory Safety**: Safe FFI with proper lifetime management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
weirui_std_rs = { git = "https://github.com/transairobot/weirui_std_rs.git" }
```

## Usage

Here's a simple example of how to use the library:

```rust
use weirui_std_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Write to console
    println!("Hello, Weirui robot!");
    
    // Get information about all actuators
    let actuators = get_actuators()?;
    for actuator in actuators {
        println!("Actuator: {} (ID: {})", actuator.name, actuator.id);
    }
    
    // Control robot actuators
    let servo_ids = &[1, 2, 3];
    let target_positions = &[1.57, 0.0, -1.57]; // 90°, 0°, -90° in radians
    run_actuator_targets(servo_ids, target_positions)?;
    
    Ok(())
}
```

## API Documentation

### Host Functions

The library provides wrappers for these low-level host functions:

1. **Console I/O**
   - `write_console(message: &str)` - Write a message to the console

2. **Actuator Control**
   - `run_actuator_targets(servo_ids: &[u32], target_radians: &[f32])` - Set target positions for actuators
   - `get_actuators()` - Get information about all actuators

3. **Joint Information**
   - `get_joints()` - Get information about robot joints

### Utility Functions

- `degrees_to_radians(degrees: f32) -> f32` - Convert degrees to radians
- `radians_to_degrees(radians: f32) -> f32` - Convert radians to degrees
- `normalize_radians(radians: f32) -> f32` - Normalize angles to [-π, π] range

## Architecture

The library acts as a bridge between Rust WebAssembly modules and host environment functions:

1. **Host Functions** - External C functions provided by the host
2. **Protobuf Messages** - Type-safe message serialization
3. **Error Handling** - Unified error system with proper propagation
4. **Utility Functions** - High-level convenience functions
5. **Angle Conversion** - Mathematical utilities for coordinate systems

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.