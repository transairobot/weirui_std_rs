//! # Weirui Std RS
//!
//! A Rust standard library for robot control using WebAssembly host functions.
//! This library provides a unified interface for controlling servo motors,
//! performing network requests, and interacting with Model Context Protocol (MCP) services.
//!
//! ## Features
//!
//! - **Servo Control**: Control servo motors with radian-based positioning
//! - **Action System**: Execute coordinated movements across multiple servos
//! - **End Effector Control**: Move robot end effector using inverse kinematics
//! - **Network Functions**: Perform HTTP requests
//! - **MCP Integration**: Connect to and interact with MCP services
//! - **I/O Functions**: Read user input with prompts
//! - **Unified Error Handling**: Consistent error reporting across all functions
//! - **Angle Utilities**: Convert between degrees and radians with normalization
//!
//! ## Quick Start
//!
//! ```rust
//! use weirui_std_rs::*;
//!
//! // Get servo information
//! let servo_ids = [1u8, 2u8, 3u8];
//! let info_list = servo_info(&servo_ids)?;
//!
//! // Move servo to 90 degrees
//! let angle_radians = degrees_to_radians(90.0);
//! let result = move_servo_to_angle(1, angle_radians, 100, 50)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod error;
pub mod host_functions;
pub mod protobuf;
pub mod utils;

// Re-export all public items for easy access
pub use error::*;
pub use host_functions::*;
pub use protobuf::wasm_host::*;
pub use utils::{degrees_to_radians, normalize_radians, radians_to_degrees};

#[no_mangle]
pub extern "C" fn wasm_new_bytes(len: u32) -> i32 {
    // Allocate a new vector with the specified length plus 4 bytes for the length prefix
    let mut buffer = vec![0u8; (len + 4) as usize];
    
    // Write the length to the first 4 bytes (as little-endian u32)
    let len_bytes = len.to_le_bytes();
    buffer[0] = len_bytes[0];
    buffer[1] = len_bytes[1];
    buffer[2] = len_bytes[2];
    buffer[3] = len_bytes[3];

    // Get the pointer to the allocated memory
    let ptr = buffer.as_mut_ptr();

    // Prevent Rust from deallocating the memory
    std::mem::forget(buffer);

    // Return the pointer as an i32
    ptr as i32
}
