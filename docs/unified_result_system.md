# Unified Host Function Result System

## Overview

The unified result system provides consistent error handling and response formatting across all host functions in the WASM runtime. All host function responses are now wrapped in a standardized `HostResult` message that includes error codes, error messages, and serialized response data.

## Architecture

### Core Components

1. **HostResult Message** (protobuf)
2. **HostError Types** (Rust)
3. **Unified Error Handling** (helper functions)
4. **Host Function Wrappers** (implementation pattern)

## HostResult Message Structure

```protobuf
message HostResult {
    optional int32 error_code = 1;      // 0 = success, non-zero = error
    optional string error_message = 2;  // Human-readable error description
    optional bytes data = 3;            // Serialized response data (specific message type)
}
```

## Error Codes

```rust
pub enum HostErrorCode {
    Success = 0,
    InvalidParameter = 1,
    MemoryError = 2,
    ServoError = 3,
    NetworkError = 4,
    McpError = 5,
    SerializationError = 6,
    InternalError = 7,
}
```

## Implementation Pattern

### 1. Host Function Wrapper

Each host function follows this pattern:

```rust
pub fn host_function(mut caller: Caller<'_, HostState>, params...) -> anyhow::Result<i32> {
    let memory = caller
        .get_export("memory")
        .and_then(|e| e.into_memory())
        .ok_or_else(|| anyhow::anyhow!("Failed to find memory export"))?;

    let result = host_function_internal(&caller, &memory, params...);
    handle_host_result(caller, &memory, result)
}
```

### 2. Internal Implementation

The actual logic is implemented in an internal function:

```rust
fn host_function_internal(
    caller: &Caller<'_, HostState>,
    memory: &wasmtime::Memory,
    params...
) -> Result<ResponseType, HostError> {
    // Implementation with proper error handling
    let data = read_array(caller, memory, ptr)
        .map_err(|e| HostError::new(HostErrorCode::MemoryError, format!("Failed to read data: {}", e)))?;
    
    // Process data and return result
    Ok(response_data)
}
```

## Updated Host Functions

### Robot Control Functions

#### servo_info
- **Input**: Servo IDs array
- **Output**: `HostResult` containing `ServoInfoList`
- **Error Handling**: Memory errors, servo communication errors

#### servo_raw_param
- **Input**: Servo IDs array
- **Output**: `HostResult` containing `ServoRawParamList`
- **Error Handling**: Memory errors, servo communication errors

#### run_target_action
- **Input**: `TargetPositionAction` message
- **Output**: `HostResult` containing `ActionResult`
- **Error Handling**: Parameter validation, servo communication errors

#### run_delta_action
- **Input**: `DeltaPositionAction` message
- **Output**: `HostResult` containing `ActionResult`
- **Error Handling**: Parameter validation, servo communication errors, current position reading

#### run_end_effector_action
- **Input**: `EndEffectorAction` message
- **Output**: `HostResult` containing `ActionResult`
- **Error Handling**: URDF file loading, inverse kinematics solving, servo communication errors

### Network Functions

#### fetch
- **Input**: URL string and `HttpRequest` message
- **Output**: `HostResult` containing `HttpResponse`
- **Error Handling**: Network errors, serialization errors, memory errors

## WASM Client Usage

### JavaScript/TypeScript Example

```typescript
// Helper function to handle HostResult
function handleHostResult<T>(resultBytes: Uint8Array, parseResponse: (data: Uint8Array) => T): T {
    const hostResult = HostResult.deserializeBinary(resultBytes);
    
    if (hostResult.getErrorCode() !== 0) {
        throw new Error(`Host function error [${hostResult.getErrorCode()}]: ${hostResult.getErrorMessage()}`);
    }
    
    const responseData = hostResult.getData();
    if (!responseData) {
        throw new Error("No response data in successful result");
    }
    
    return parseResponse(responseData);
}

// Usage example
try {
    // Call servo_info host function
    let servoIds = new Uint8Array([1, 2, 3]);
    let resultBytes = host_servo_info(servoIds);
    
    // Handle the unified result
    let servoInfoList = handleHostResult(resultBytes, (data) => 
        ServoInfoList.deserializeBinary(data)
    );
    
    // Process the servo info
    for (let servoInfo of servoInfoList.getInfosList()) {
        console.log(`Servo ${servoInfo.getServoId()}: position=${servoInfo.getPosition()}`);
    }
} catch (error) {
    console.error("Servo info request failed:", error.message);
}
```

### AssemblyScript Example

```typescript
// Helper function for AssemblyScript
function handleHostResult<T>(resultBytes: Uint8Array, parseResponse: (data: Uint8Array) => T): T {
    const hostResult = HostResult.decode(resultBytes);
    
    if (hostResult.error_code != 0) {
        throw new Error(`Host function error [${hostResult.error_code}]: ${hostResult.error_message}`);
    }
    
    if (!hostResult.data) {
        throw new Error("No response data in successful result");
    }
    
    return parseResponse(hostResult.data);
}

// Usage
let servoIds = new Uint8Array(3);
servoIds[0] = 1;
servoIds[1] = 2;
servoIds[2] = 3;

let resultBytes = host_servo_info(servoIds);
let servoInfoList = handleHostResult(resultBytes, (data: Uint8Array) => 
    ServoInfoList.decode(data)
);
```

## Benefits

1. **Consistent Error Handling**: All host functions return errors in the same format
2. **Structured Error Information**: Error codes and messages provide clear debugging information
3. **Type Safety**: Protobuf serialization ensures data integrity
4. **Extensibility**: Easy to add new error codes and response types
5. **Debugging**: Clear error messages help identify issues quickly

## Migration Guide

### For Existing Host Functions

1. Add unified result imports:
```rust
use crate::wasm_exec::host_func::result::{HostError, HostErrorCode, handle_host_result};
```

2. Split function into wrapper and internal implementation
3. Convert `anyhow::Error` to `HostError` with appropriate error codes
4. Use `handle_host_result` to serialize and return the response

### For WASM Clients

1. Update host function calls to handle `HostResult` wrapper
2. Add error checking for error codes
3. Extract response data from the `data` field
4. Handle errors appropriately in application logic

## Future Enhancements

1. **Async Support**: Extend pattern for async host functions
2. **Metrics**: Add performance metrics to error handling
3. **Logging**: Enhanced logging with error codes and context
4. **Validation**: Input parameter validation helpers
5. **Testing**: Unit test helpers for host function testing