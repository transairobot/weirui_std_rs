# Weirui Std RS

A Rust standard library for robot control using WebAssembly host functions. This library provides a unified interface for controlling servo motors, performing network requests, and interacting with Model Context Protocol (MCP) services.

## Features

- **Servo Control**: Control servo motors with radian-based positioning
- **Action System**: Execute coordinated movements across multiple servos
- **End Effector Control**: Move robot end effector using inverse kinematics
- **Network Functions**: Perform HTTP requests
- **MCP Integration**: Connect to and interact with MCP services
- **I/O Functions**: Read user input with prompts
- **Unified Error Handling**: Consistent error reporting across all functions
- **Angle Utilities**: Convert between degrees and radians with normalization

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
weirui_std_rs = "0.1.0"
```

## Basic Usage

### Servo Control

```rust
use weirui_std_rs::*;

// Get servo information
let servo_ids = [1u8, 2u8, 3u8];
let info_list = servo_info(&servo_ids)?;

// Move servo to 90 degrees
let angle_radians = degrees_to_radians(90.0);
let result = move_servo_to_angle(1, angle_radians, 100, 50)?;

// Move multiple servos
let servo_ids = [1u8, 2u8];
let angles = [degrees_to_radians(45.0), degrees_to_radians(-30.0)];
let result = move_servos_to_angles(&servo_ids, &angles)?;
```

### End Effector Control

```rust
use std::borrow::Cow;

// Move end effector by delta amounts
let action = EndEffectorAction {
    delta_x: Some(0.05),  // 5cm in X
    delta_y: Some(0.0),   // No Y movement
    delta_z: Some(0.02),  // 2cm up in Z
    urdf_file_path: None, // Use default URDF
    target_link_name: None, // Use default target link
};

let result = run_end_effector_action(&action)?;
```

### Network Requests

```rust
use std::borrow::Cow;

let request = HttpRequest {
    url: Some(Cow::Borrowed("https://api.example.com/data")),
    method: Some(Cow::Borrowed("GET")),
    headers: vec![
        Pair {
            key: Some(Cow::Borrowed("Authorization")),
            value: Some(Cow::Borrowed("Bearer token123")),
        }
    ],
    body: None,
};

let response = fetch("https://api.example.com/data", &request)?;
```

### Interactive Input

```rust
let user_input = readline("Enter command: ")?;
println!("You entered: {}", user_input);
```

## Coordinate System

All servo positions use **radian-based coordinates** in the range **-π to π**:

- **0 radians**: Center position
- **π/2 radians**: 90 degrees clockwise
- **-π/2 radians**: 90 degrees counter-clockwise
- **π radians**: 180 degrees

### Angle Conversion Utilities

```rust
// Convert degrees to radians
let radians = degrees_to_radians(90.0); // π/2

// Convert radians to degrees  
let degrees = radians_to_degrees(std::f32::consts::PI); // 180.0

// Normalize angle to [-π, π] range
let normalized = normalize_radians(3.0 * std::f32::consts::PI); // π
```

## Error Handling

All functions use the unified `HostResult` system with these error codes:

- `0` - Success
- `1` - Invalid Parameter
- `2` - Memory Error
- `3` - Servo Error
- `4` - Network Error
- `5` - MCP Error
- `6` - Serialization Error
- `7` - Internal Error

```rust
match servo_info(&[1u8]) {
    Ok(info_list) => {
        // Handle successful result
        println!("Got servo info: {:?}", info_list);
    }
    Err(e) => {
        // Handle error with descriptive message
        eprintln!("Servo error: {}", e);
    }
}
```

## MCP Integration

Connect to Model Context Protocol services:

```rust
// Connect to MCP service
let handle = connect_mcp_service("http://localhost:3001/sse");

// List available tools
let tools = list_all_tools(handle)?;

// Call a tool
let request = McpCallToolRequest {
    tool_name: Some(Cow::Borrowed("get_weather")),
    arguments: Some(Cow::Borrowed(r#"{"location": "New York"}"#)),
};
let response = call_tool(handle, &request)?;
```

## Examples

Run the example to see the library in action:

```bash
cargo run --example robot_control
```

## Architecture

This library acts as a bridge between Rust WebAssembly modules and host environment functions. It provides:

1. **Type-safe protobuf message handling** using `quick-protobuf`
2. **Memory-safe FFI** with proper error handling
3. **Consistent API** across all robot control functions
4. **Lifetime management** for borrowed data from host functions

## Host Function Requirements

The library expects the host environment to provide these external functions:

- `host_servo_info` - Get servo information
- `host_run_target_action` - Execute target position actions
- `host_run_delta_action` - Execute delta movement actions
- `host_run_end_effector_action` - Execute end effector movements
- `host_fetch` - Perform HTTP requests
- `host_readline` - Read user input
- `host_connect_mcp_service` - Connect to MCP services
- And more...

See the [Host Functions Documentation](docs/host_func.md) for complete details.

## License

This project is licensed under the MIT License.
