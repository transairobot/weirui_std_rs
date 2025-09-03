# Host Functions API Documentation for WASM Clients

This document provides API documentation for WASM clients to use host functions implemented in the Weirui Kernel system. The documentation focuses on how to implement client-side code in Rust to call these host functions.

## Overview

Host functions allow WASM code running in a Web Worker to communicate with the main thread to perform operations that are not possible within the WASM environment itself. This includes operations like console output, robot control, and information retrieval.

## Using Host Functions in Rust

To use host functions in your Rust code, you need to:

1. Declare the host functions as external functions
2. Encode request messages using Protocol Buffers
3. Call the host functions with the encoded data
4. Handle the response appropriately

## Available Host Functions

### 1. console_write - Write to Console

Writes a message to the main thread's console using `console.log`.

#### Function Signature
```rust
extern "C" {
    fn console_write(ptr: i32, len: i32) -> i32;
}
```

#### Request Message
```protobuf
message ConsoleWriteReq {
    optional string message = 1;
}
```

#### Example Usage
```rust
use prost::Message; // You'll need the prost crate for protobuf encoding

// Define the ConsoleWriteReq struct (or generate it from the .proto file)
#[derive(Message)]
struct ConsoleWriteReq {
    #[prost(string, optional, tag="1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}

// Function to write to console
fn write_to_console(message: &str) {
    // Create the request
    let req = ConsoleWriteReq {
        message: Some(message.to_string()),
    };
    
    // Encode the request
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("Failed to encode ConsoleWriteReq");
    
    // Call the host function
    let ptr = buf.as_ptr() as i32;
    let len = buf.len() as i32;
    let result_ptr = unsafe { console_write(ptr, len) };
    
    // Handle the result (result_ptr points to a HostResult message)
    // You would typically decode the HostResult to check for errors
}
```

### 2. run_target_action - Control Robot Actuators

Sets target positions for robot actuators.

#### Function Signature
```rust
extern "C" {
    fn run_target_action(ptr: i32, len: i32) -> i32;
}
```

#### Request Message
```protobuf
message RunTargetActionReq {
    repeated uint32 servo_id_vec = 1;
    repeated float target_rad_vec = 2;
}
```

#### Example Usage
```rust
// Define the RunTargetActionReq struct
#[derive(Message)]
struct RunTargetActionReq {
    #[prost(uint32, repeated, tag="1")]
    pub servo_id_vec: ::prost::alloc::vec::Vec<u32>,
    #[prost(float, repeated, tag="2")]
    pub target_rad_vec: ::prost::alloc::vec::Vec<f32>,
}

// Function to set actuator targets
fn set_actuator_targets(servo_ids: &[u32], target_radians: &[f32]) {
    // Create the request
    let req = RunTargetActionReq {
        servo_id_vec: servo_ids.to_vec(),
        target_rad_vec: target_radians.to_vec(),
    };
    
    // Encode the request
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("Failed to encode RunTargetActionReq");
    
    // Call the host function
    let ptr = buf.as_ptr() as i32;
    let len = buf.len() as i32;
    let result_ptr = unsafe { run_target_action(ptr, len) };
    
    // Handle the result
}
```

### 3. get_actuator_info - Get Actuator Information

Retrieves information about robot actuators.

#### Function Signature
```rust
extern "C" {
    fn get_actuator_info(ptr: i32, len: i32) -> i32;
}
```

#### Request Message
```protobuf
// Empty request message
message GetActuatorInfoReq {
}

message ActuatorInfo {
    optional string name = 1;
    optional int32 id = 2;
    optional ActuatorType type = 3;
    optional string vendor = 4;
    optional string model = 5;
    optional float ctrl = 6;
    optional float ctrl_min = 7;
    optional float ctrl_max = 8;
    optional float force_min = 9;
    optional float force_max = 10;
    optional int32 joint_id = 11;
}

enum ActuatorType {
    DUMMY = 0;
    MOTOR = 1;
    POSITION = 2;
}
```

#### Example Usage
```rust
// Define the request and response structs
#[derive(Message)]
struct GetActuatorInfoReq {}

#[derive(Message)]
struct ActuatorInfo {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub id: ::core::option::Option<i32>,
    #[prost(enumeration="ActuatorType", optional, tag="3")]
    pub r#type: ::core::option::Option<i32>,
    // ... other fields
}

#[derive(Message)]
struct GetActuatorInfoResp {
    #[prost(message, repeated, tag="1")]
    pub actuators: ::prost::alloc::vec::Vec<ActuatorInfo>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActuatorType {
    Dummy = 0,
    Motor = 1,
    Position = 2,
}

// Function to get actuator information
fn get_actuator_info() -> Vec<ActuatorInfo> {
    // Create the request (empty)
    let req = GetActuatorInfoReq {};
    
    // Encode the request
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("Failed to encode GetActuatorInfoReq");
    
    // Call the host function
    let ptr = buf.as_ptr() as i32;
    let len = buf.len() as i32;
    let result_ptr = unsafe { get_actuator_info(ptr, len) };
    
    // Decode the response (simplified - you'd need to handle the HostResult wrapper)
    // This is just an example of how you might process the response
    vec![] // Return actual data in real implementation
}
```

### 4. get_joint_info - Get Joint Information

Retrieves information about robot joints.

#### Function Signature
```rust
extern "C" {
    fn get_joint_info(ptr: i32, len: i32) -> i32;
}
```

#### Request Message
```protobuf
// Empty request message
message GetJointInfoReq {
}

message JointInfo {
    optional string name = 1;
    optional int32 id = 2;
    optional JointType type = 3;
    optional int32 dof_dim = 4;
    repeated float joint_pos = 5;
}

enum JointType {
    HINGE = 0;
    SLIDE = 1;
    BALL = 2;
    FREE = 3;
}
```

## Error Handling

All host functions return a pointer to a `HostResult` message which contains:

```protobuf
message HostResult {
    optional int32 error_code = 1;      // 0 = success, non-zero = error
    optional string error_message = 2;  // Human-readable error description
    optional bytes data = 3;            // Serialized response data (specific message type)
}
```

Always check the `error_code` field in the response:

```rust
#[derive(Message)]
struct HostResult {
    #[prost(int32, optional, tag="1")]
    pub error_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub error_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

fn check_host_result(result_ptr: i32) -> Result<HostResult, String> {
    // In a real implementation, you would:
    // 1. Convert result_ptr to a memory buffer
    // 2. Decode the HostResult message
    // 3. Check error_code and return appropriate result
    
    // This is a simplified example
    Ok(HostResult {
        error_code: Some(0),
        error_message: None,
        data: None,
    })
}
```

## Memory Management

When calling host functions:
1. The WASM module is responsible for allocating memory for request messages
2. The host function returns a pointer to a response message in WASM memory
3. After processing the response, you may need to free the memory (implementation dependent)

## Dependencies

To use these host functions in Rust, you'll need:

1. The `prost` crate for Protocol Buffer encoding/decoding:
```toml
[dependencies]
prost = "0.11"
```

2. Generated Rust code from the `.proto` files (or manually defined structs as shown above)

## Best Practices

1. Always check the error code in the `HostResult` response
2. Handle errors gracefully to prevent crashes
3. Free any allocated memory when appropriate
4. Use meaningful error messages for debugging
5. Validate input parameters before calling host functions