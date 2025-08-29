# Host Functions Documentation

This document describes all the host functions that the WebAssembly module expects from the host environment. These functions provide the interface between the WASM module and the host system for robot control, networking, and system operations.

## Overview

The WASM module communicates with the host environment through external function declarations marked with `@external("env", "function_name")`. All data exchange uses binary serialization formats (Protocol Buffers) for efficiency and type safety.

## Unified Result System

**All host functions now return a unified `HostResult` wrapper** that provides consistent error handling and response formatting. The HostResult structure contains:

```protobuf
message HostResult {
    optional int32 error_code = 1;      // 0 = success, non-zero = error
    optional string error_message = 2;  // Human-readable error description
    optional bytes data = 3;            // Serialized response data (specific message type)
}
```

### Error Codes
- `0` - Success
- `1` - Invalid Parameter
- `2` - Memory Error
- `3` - Servo Error
- `4` - Network Error
- `5` - MCP Error
- `6` - Serialization Error
- `7` - Internal Error

### Usage Pattern

Functions using the unified result system should be called using this pattern:

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
```

### AssemblyScript Usage

For AssemblyScript WASM modules:

```typescript
// AssemblyScript helper function
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
```

### Migration Status

The following functions have been migrated to use the unified result system:
- ✅ `servo_info` - Returns `HostResult` containing `ServoInfoList`
- ✅ `servo_raw_param` - Returns `HostResult` containing `ServoRawParamList`  
- ✅ `run_target_action` - Returns `HostResult` containing `ActionResult`
- ✅ `run_delta_action` - Returns `HostResult` containing `ActionResult`
- ✅ `run_end_effector_action` - Returns `HostResult` containing `ActionResult`
- ✅ `fetch` - Returns `HostResult` containing `HttpResponse`
- ✅ `readline` - Returns `HostResult` containing `StringResponse`

The following functions still use legacy return formats:
- ⏳ `connect_mcp_service` - Returns simple integer handle
- ⏳ `list_all_tools` - Returns direct `McpToolList` protobuf
- ⏳ `call_tool` - Returns direct `McpCallToolResponse` protobuf
- ⏳ `register_tool` - Returns simple integer result
- ⏳ `set_target_radians` - Returns void
- ⏳ `disable_torque` - Returns void  
- ⏳ `enable_torque` - Returns void

## Servo Control Functions

### Radian-Based Coordinate System

All servo control functions now use **radian-based positioning** for consistent mathematical operations and improved precision. Key characteristics:

- **Range:** All servo positions are expressed in radians within the range **-π to π** (-3.14159 to 3.14159)
- **Zero Position:** 0 radians represents the servo's center position
- **Positive Direction:** Positive values represent clockwise rotation (servo-dependent)
- **Negative Direction:** Negative values represent counter-clockwise rotation
- **Conversion:** To convert to degrees: `degrees = radians * 180 / Math.PI`
- **Precision:** Floating-point precision allows for smooth, accurate positioning

**Benefits of Radian System:**
- Mathematical consistency with trigonometric functions
- Direct compatibility with inverse kinematics calculations
- Eliminates servo-specific position value conversions
- Improved precision for fine motor control
- Standardized coordinate system across all servos

**Common Radian Values:**
```typescript
// Utility functions for common angles
const ANGLES = {
    ZERO: 0,                    // Center position
    QUARTER_TURN: Math.PI/2,    // 90 degrees
    HALF_TURN: Math.PI,         // 180 degrees
    THREE_QUARTER: 3*Math.PI/2, // 270 degrees (equivalent to -π/2)
    FULL_TURN: 2*Math.PI        // 360 degrees (wraps to 0)
};

// Conversion utilities
function degreesToRadians(degrees: number): number {
    return degrees * Math.PI / 180;
}

function radiansToDegrees(radians: number): number {
    return radians * 180 / Math.PI;
}

// Normalize radians to [-π, π] range
function normalizeRadians(radians: number): number {
    while (radians > Math.PI) radians -= 2 * Math.PI;
    while (radians < -Math.PI) radians += 2 * Math.PI;
    return radians;
}
```

### `servo_info`

**Declaration:** `host_servo_info(ids: Uint8Array): Uint8Array`

**Purpose:** Retrieves detailed information about one or more servo motors.

**Parameters:**
- `ids`: Uint8Array containing servo IDs to query

**Returns:** `HostResult` containing Protocol Buffer encoded `ServoInfoList` message

**ServoInfoList Structure:**
```protobuf
message ServoInfoList {
    repeated ServoInfo infos = 1;
}

message ServoInfo {
    optional uint32 servo_id = 1;        // Unique servo identifier
    optional string name = 3;            // Human-readable servo name
    optional float min_rad = 4;          // Minimum allowed position in radians (-π to π)
    optional float max_rad = 5;          // Maximum allowed position in radians (-π to π)
    optional uint32 resolution = 6;      // Servo resolution (e.g., 4096 for 12-bit servos)
}
```

**Usage Example:**
```typescript
try {
    // Get info for servos 1, 2, and 3
    let servoIds = new Uint8Array([1, 2, 3]);
    let resultBytes = host_servo_info(servoIds);
    
    // Handle the unified result
    let servoInfoList = handleHostResult(resultBytes, (data) => 
        ServoInfoList.deserializeBinary(data)
    );

    for (let servoInfo of servoInfoList.getInfosList()) {
        console.log(`Servo ${servoInfo.getServoId()}: ${servoInfo.getName()}`);
        console.log(`Range: ${servoInfo.getMinRad()} to ${servoInfo.getMaxRad()} radians`);
        console.log(`Resolution: ${servoInfo.getResolution()}`);
    }
} catch (error) {
    console.error("Servo info request failed:", error.message);
}
```

### `servo_raw_param`

**Declaration:** `host_servo_raw_param(ids: Uint8Array): Uint8Array`

**Purpose:** Retrieves raw parameter values from servo motors for advanced configuration and diagnostics.

**Parameters:**
- `ids`: Uint8Array containing servo IDs to query

**Returns:** `HostResult` containing Protocol Buffer encoded `ServoRawParamList` message

**ServoRawParamList Structure:**
```protobuf
message ServoRawParamList {
    repeated ServoRawParam params = 1;
}

message ServoRawParam {
    optional uint32 servo_id = 1;
    repeated ParamEntry params = 2;
}

message ParamEntry {
    optional string key = 1;
    optional int32 value = 2;
}
```

**Usage Example:**
```typescript
try {
    // Get raw parameters for servo 1
    let servoIds = new Uint8Array([1]);
    let resultBytes = host_servo_raw_param(servoIds);
    
    // Handle the unified result
    let paramList = handleHostResult(resultBytes, (data) => 
        ServoRawParamList.deserializeBinary(data)
    );

    for (let servoParam of paramList.getParamsList()) {
        console.log(`Servo ${servoParam.getServoId()} parameters:`);
        for (let param of servoParam.getParamsList()) {
            console.log(`  ${param.getKey()}: ${param.getValue()}`);
        }
    }
} catch (error) {
    console.error("Servo raw param request failed:", error.message);
}
```

### `set_target_radians`

**Declaration:** `host_set_target_radians(ids: Uint8Array, target_radians_ptr: Float32Array, speeds_ptr: Int32Array, max_accelerations_ptr: Int32Array): void`

**Purpose:** Commands multiple servos to move to target positions specified in radians with motion parameters.

**Parameters:**
- `ids`: Uint8Array containing servo IDs
- `target_radians_ptr`: Float32Array of target positions in radians (one per servo, range -π to π)
- `speeds_ptr`: Int32Array of movement speeds (one per servo)
- `max_accelerations_ptr`: Int32Array of maximum accelerations (one per servo)

**Returns:** void

**Notes:**
- All arrays must have the same length
- Radian values must be in the range -π to π (-3.14159 to 3.14159)
- Speed and acceleration values depend on servo specifications
- Radians are automatically converted to servo-specific position values internally

**Usage Example:**
```typescript
// Move servos 1 and 2 to π/4 and -π/2 radians respectively
setTargetRadians([1, 2], [Math.PI/4, -Math.PI/2], [100, 150], [50, 75]);
```

## Action Control Functions

### `run_target_action`

**Declaration:** `host_run_target_action(action: Uint8Array): Uint8Array`

**Purpose:** Executes a coordinated action across multiple servos using radian-based positioning and returns the execution result.

**Parameters:**
- `action`: Protocol Buffer encoded `TargetRadianAction` message

**Returns:** `HostResult` containing Protocol Buffer encoded `ActionResult` message

**TargetRadianAction Structure:**
```protobuf
message TargetRadianAction {
    repeated uint32 servo_id_vec = 1;
    repeated float target_rad_vec = 2;  // Target positions in radians (-π to π)
}
```

**ActionResult Structure:**
```protobuf
message ActionResult {
    repeated bool finish_vec = 1;
    repeated float current_radian_vec = 2;  // Current positions in radians (-π to π)
}
```

**Usage Example:**
```typescript
try {
    // Create action to move servo 1 to π/2 radians (90 degrees)
    let action = new TargetRadianAction();
    action.setServoIdVec([1]);
    action.setTargetRadVec([Math.PI/2]);
    
    let actionBytes = action.serializeBinary();
    let resultBytes = host_run_target_action(actionBytes);
    
    // Handle the unified result
    let result = handleHostResult(resultBytes, (data) => 
        ActionResult.deserializeBinary(data)
    );
    
    console.log(`Action finished: ${result.getFinishVec()[0]}`);
    console.log(`Current position: ${result.getCurrentRadianVec()[0]} radians`);
    console.log(`Current position: ${result.getCurrentRadianVec()[0] * 180 / Math.PI} degrees`);
} catch (error) {
    console.error("Action execution failed:", error.message);
}
```

### `run_delta_action`

**Declaration:** `host_run_delta_action(action: Uint8Array): Uint8Array`

**Purpose:** Executes a coordinated delta movement action across multiple servos, moving them by relative amounts in radians from their current positions.

**Parameters:**
- `action`: Protocol Buffer encoded `DeltaRadianAction` message

**Returns:** `HostResult` containing Protocol Buffer encoded `ActionResult` message

**DeltaRadianAction Structure:**
```protobuf
message DeltaRadianAction {
    repeated uint32 servo_id_vec = 1;
    repeated float delta_rad_vec = 2;  // Delta movements in radians
}
```

**Usage Example:**
```typescript
try {
    // Create action to move servo 1 by +π/6 radians (30 degrees) from current position
    let action = new DeltaRadianAction();
    action.setServoIdVec([1]);
    action.setDeltaRadVec([Math.PI/6]);
    
    let actionBytes = action.serializeBinary();
    let resultBytes = host_run_delta_action(actionBytes);
    
    // Handle the unified result
    let result = handleHostResult(resultBytes, (data) => 
        ActionResult.deserializeBinary(data)
    );
    
    console.log(`Action finished: ${result.getFinishVec()[0]}`);
    console.log(`Final position: ${result.getCurrentRadianVec()[0]} radians`);
    console.log(`Final position: ${result.getCurrentRadianVec()[0] * 180 / Math.PI} degrees`);
} catch (error) {
    console.error("Delta action execution failed:", error.message);
}
```

### `run_end_effector_action`

**Declaration:** `host_run_end_effector_action(action: Uint8Array): Uint8Array`

**Purpose:** Executes an end effector movement using inverse kinematics to move the robot's end effector to a target position in 3D space.

**Parameters:**
- `action`: Protocol Buffer encoded `EndEffectorAction` message

**Returns:** `HostResult` containing Protocol Buffer encoded `ActionResult` message

**EndEffectorAction Structure:**
```protobuf
message EndEffectorAction {
    optional float delta_x = 1;
    optional float delta_y = 2;
    optional float delta_z = 3;
    optional string urdf_file_path = 4;  // Path to URDF file, defaults to "./SO101/so101_new_calib.urdf"
    optional string target_link_name = 5;  // Target link name, defaults to "gripper"
}
```

**Usage Example:**
```typescript
try {
    // Create action to move end effector by +0.1 in Z direction
    let action = new EndEffectorAction();
    action.setDeltaX(0.0);
    action.setDeltaY(0.0);
    action.setDeltaZ(0.1);
    // Optional: specify custom URDF file and target link
    // action.setUrdfFilePath("./custom_robot.urdf");
    // action.setTargetLinkName("end_effector");
    
    let actionBytes = action.serializeBinary();
    let resultBytes = host_run_end_effector_action(actionBytes);
    
    // Handle the unified result
    let result = handleHostResult(resultBytes, (data) => 
        ActionResult.deserializeBinary(data)
    );
    
    console.log(`Action finished: ${result.getFinishVec()}`);
    console.log(`Final positions: ${result.getCurrentRadianVec()} radians`);
    
    // Convert to degrees for display
    let degrees = result.getCurrentRadianVec().map(rad => rad * 180 / Math.PI);
    console.log(`Final positions: ${degrees} degrees`);
} catch (error) {
    console.error("End effector action execution failed:", error.message);
}
```

**Features:**
- Uses inverse kinematics to calculate joint angles for target end effector position
- Supports custom URDF files and target link names
- Automatically monitors servo movement completion
- Provides feedback on final servo positions in radians and completion status
- All servo positions are handled in radians (-π to π) for consistent mathematical operations

## Network Functions

### `connect_mcp_service`

**Declaration:** `host_connect_mcp_service(url: string): i32`

**Purpose:** Establishes a connection to a Model Context Protocol (MCP) service via Server-Sent Events (SSE) transport.

**Parameters:**
- `url`: The SSE endpoint URL for the MCP service (e.g., "http://transairobot.com:3001/sse")

**Returns:** 
- Service handle ID (positive integer) on successful connection establishment
- Throws error on connection failure

**Note:** This function returns a simple integer handle and does not use the unified result system.

**Behavior:**
- Creates an SSE client transport to the specified URL
- Establishes an MCP service connection with client role
- Returns immediately after successful connection setup
- The connection runs asynchronously in the background

**Error Handling:**
- Invalid URLs will throw an error
- Network connectivity issues will result in connection errors
- SSE transport failures are propagated as errors
- Connection timeouts are handled by the underlying transport

**Usage Example:**
```typescript
try {
    // Connect to MCP service
    let result = host_connect_mcp_service("http://transairobot.com:3001/sse");
    if (result === 1) {
        console.log("Successfully connected to MCP service");
        
        // Service is now available for MCP protocol operations
        // Additional MCP operations would be handled through separate functions
    }
} catch (error) {
    console.error("Failed to connect to MCP service:", error);
}

// Example with different MCP endpoints
function connectToMcpServices(): void {
    const mcpEndpoints = [
        "http://localhost:3001/sse",
        "https://api.example.com/mcp/sse",
        "http://transairobot.com:3001/sse"
    ];
    
    for (let endpoint of mcpEndpoints) {
        try {
            let result = host_connect_mcp_service(endpoint);
            console.log(`Connected to ${endpoint}: ${result === 1 ? 'Success' : 'Failed'}`);
            break; // Use first successful connection
        } catch (error) {
            console.warn(`Failed to connect to ${endpoint}:`, error);
        }
    }
}
```

**Technical Details:**
- Uses Server-Sent Events (SSE) as the transport layer
- Implements MCP client role for bidirectional communication
- Connection is established asynchronously but function returns synchronously
- The underlying service handles MCP protocol message routing

**Security Considerations:**
- Validate MCP service URLs to prevent SSRF attacks
- Ensure proper authentication if required by the MCP service
- Consider rate limiting for connection attempts

### `list_all_tools`

**Declaration:** `host_list_all_tools(handle_id: i32): Uint8Array`

**Purpose:** Retrieves all available tools from a connected MCP service.

**Parameters:**
- `handle_id`: The service handle ID returned from `connect_mcp_service`

**Returns:** Protocol Buffer encoded `McpToolList` message (Note: This function may not yet use the unified result system)

**McpToolList Structure:**
```protobuf
message McpToolList {
    repeated McpTool tools = 1;
}

message McpTool {
    optional string name = 1;
    optional string description = 2;
    optional string input_schema = 3;  // JSON schema as string
}
```

**Behavior:**
- Calls the MCP service's `list_all_tools()` method asynchronously
- Converts MCP tool definitions to protobuf format
- Serializes input schemas as JSON strings
- Returns all available tools in a single response

**Error Handling:**
- Invalid handle IDs will result in an error
- MCP service communication errors are propagated
- JSON serialization errors for schemas are handled gracefully

**Usage Example:**
```typescript
try {
    // First connect to MCP service
    let serviceHandle = host_connect_mcp_service("http://transairobot.com:3001/sse");
    
    // List all available tools
    let toolsBytes = host_list_all_tools(serviceHandle);
    let toolsList = McpToolList.deserializeBinary(toolsBytes);
    
    console.log(`Found ${toolsList.getToolsList().length} tools:`);
    
    for (let tool of toolsList.getToolsList()) {
        console.log(`Tool: ${tool.getName()}`);
        console.log(`Description: ${tool.getDescription()}`);
        
        // Parse input schema if available
        if (tool.getInputSchema()) {
            let schema = JSON.parse(tool.getInputSchema());
            console.log(`Input Schema:`, schema);
        }
    }
} catch (error) {
    console.error("Failed to list MCP tools:", error);
}

// Example with tool filtering
function findToolByName(serviceHandle: i32, toolName: string): McpTool | null {
    try {
        let toolsBytes = host_list_all_tools(serviceHandle);
        let toolsList = McpToolList.deserializeBinary(toolsBytes);
        
        for (let tool of toolsList.getToolsList()) {
            if (tool.getName() === toolName) {
                return tool;
            }
        }
        return null;
    } catch (error) {
        console.error(`Failed to find tool ${toolName}:`, error);
        return null;
    }
}

// Example with schema validation
function validateToolInput(tool: McpTool, input: any): boolean {
    if (!tool.getInputSchema()) {
        return true; // No schema means any input is valid
    }
    
    try {
        let schema = JSON.parse(tool.getInputSchema());
        // Use a JSON schema validator library here
        return validateJsonSchema(input, schema);
    } catch (error) {
        console.error("Failed to validate tool input:", error);
        return false;
    }
}
```

**Technical Details:**
- Uses async/await internally but returns synchronously to WASM
- Tool schemas are serialized as JSON strings for cross-language compatibility
- Handle validation ensures only valid MCP services are accessed
- Memory management is handled automatically by the host

### `call_tool`

**Declaration:** `host_call_tool(handle_id: i32, request: Uint8Array): Uint8Array`

**Purpose:** Executes a specific tool from a connected MCP service with provided arguments.

**Parameters:**
- `handle_id`: The service handle ID returned from `connect_mcp_service`
- `request`: Protocol Buffer encoded `McpCallToolRequest` message

**Returns:** Protocol Buffer encoded `McpCallToolResponse` message (Note: This function may not yet use the unified result system)

**McpCallToolRequest Structure:**
```protobuf
message McpCallToolRequest {
    optional string tool_name = 1;
    optional string arguments = 2;  // JSON string
}
```

**McpCallToolResponse Structure:**
```protobuf
message McpCallToolResponse {
    optional string content = 1;  // JSON string of the result
    optional bool is_error = 2;
}
```

**Behavior:**
- Calls the MCP service's `call_tool()` method asynchronously
- Parses JSON arguments and passes them to the tool
- Returns the tool execution result as JSON
- Handles both successful results and error conditions

**Error Handling:**
- Invalid handle IDs will result in an error
- JSON parsing errors for arguments are handled gracefully
- Tool execution errors are returned in the response with `is_error = true`
- MCP service communication errors are propagated

**Usage Example:**
```typescript
try {
    // First connect to MCP service and get tools
    let serviceHandle = host_connect_mcp_service("http://transairobot.com:3001/sse");
    let toolsBytes = host_list_all_tools(serviceHandle);
    let toolsList = McpToolList.deserializeBinary(toolsBytes);
    
    // Find a specific tool
    let weatherTool = toolsList.getToolsList().find(tool => 
        tool.getName() === "get_weather"
    );
    
    if (weatherTool) {
        // Create tool call request
        let request = new McpCallToolRequest();
        request.setToolName("get_weather");
        
        // Set arguments as JSON string
        let args = {
            location: "New York",
            units: "celsius"
        };
        request.setArguments(JSON.stringify(args));
        
        // Serialize request
        let requestBytes = request.serializeBinary();
        
        // Call the tool
        let responseBytes = host_call_tool(serviceHandle, requestBytes);
        let response = McpCallToolResponse.deserializeBinary(responseBytes);
        
        if (response.getIsError()) {
            console.error("Tool execution failed:", response.getContent());
        } else {
            // Parse the result
            let result = JSON.parse(response.getContent());
            console.log("Weather data:", result);
        }
    }
} catch (error) {
    console.error("Failed to call MCP tool:", error);
}

// Example with error handling and validation
function callMcpTool(serviceHandle: i32, toolName: string, args: any): any {
    try {
        // Create and populate request
        let request = new McpCallToolRequest();
        request.setToolName(toolName);
        request.setArguments(JSON.stringify(args));
        
        // Make the call
        let responseBytes = host_call_tool(serviceHandle, request.serializeBinary());
        let response = McpCallToolResponse.deserializeBinary(responseBytes);
        
        if (response.getIsError()) {
            throw new Error(`Tool '${toolName}' failed: ${response.getContent()}`);
        }
        
        return JSON.parse(response.getContent());
    } catch (error) {
        console.error(`Error calling tool '${toolName}':`, error);
        throw error;
    }
}

// Example with multiple tool calls
async function processWithTools(serviceHandle: i32, userQuery: string) {
    try {
        // Get available tools
        let toolsBytes = host_list_all_tools(serviceHandle);
        let toolsList = McpToolList.deserializeBinary(toolsBytes);
        
        // Call multiple tools based on query
        for (let tool of toolsList.getToolsList()) {
            if (shouldUseTool(tool.getName(), userQuery)) {
                let args = extractArgsForTool(tool, userQuery);
                let result = callMcpTool(serviceHandle, tool.getName(), args);
                console.log(`${tool.getName()} result:`, result);
            }
        }
    } catch (error) {
        console.error("Error processing with tools:", error);
    }
}
```

**Technical Details:**
- Uses async/await internally but returns synchronously to WASM
- Arguments are serialized as JSON strings for flexibility
- Results are returned as JSON strings for cross-language compatibility
- Error conditions are clearly indicated in the response structure
- Memory management is handled automatically by the host

### `register_tool`

**Declaration:** `host_register_tool(func_ptr: i32, tool: Uint8Array): i32`

**Purpose:** Registers a new tool that can be called from the MCP service.

**Parameters:**
- `func_ptr`: Function pointer to the WASM function that implements the tool
- `tool`: Protocol Buffer encoded McpTool message describing the tool

**Returns:** 
- `0` on successful registration
- Non-zero value on failure

**McpTool Structure:**
```protobuf
message McpTool {
    optional string name = 1;
    optional string description = 2;
    optional string input_schema = 3;  // JSON schema as string
}
```

**Behavior:**
- Registers a tool implementation with the MCP service
- Associates the tool name with a WASM function pointer
- Stores tool metadata for listing and calling
- Returns immediately after registration

**Error Handling:**
- Invalid function pointers will result in an error
- Malformed tool definitions will result in an error
- Duplicate tool names may overwrite previous registrations

**Usage Example:**
```typescript
// Define a tool implementation
function myToolImplementation(args: string): string {
    // Parse arguments
    let parsedArgs = JSON.parse(args);
    
    // Implement tool logic
    let result = {
        message: "Hello from my tool!",
        input: parsedArgs
    };
    
    // Return result as JSON string
    return JSON.stringify(result);
}

// Get function pointer to the implementation
let funcPtr = getFunctionPointer(myToolImplementation);

// Create tool definition
let tool = new McpTool();
tool.setName("my_tool");
tool.setDescription("A sample tool implementation");
tool.setInputSchema(JSON.stringify({
    type: "object",
    properties: {
        name: { type: "string" },
        value: { type: "number" }
    },
    required: ["name"]
}));

// Serialize tool definition
let toolBytes = tool.serializeBinary();

// Register the tool
let result = host_register_tool(funcPtr, toolBytes);
if (result === 0) {
    console.log("Tool registered successfully");
} else {
    console.error("Failed to register tool");
}

// The tool is now available for use with call_tool
```

**Technical Details:**
- Tools are stored in a registry associated with the MCP service
- Function pointers are used to call WASM implementations directly
- Tool schemas are stored as JSON strings for cross-language compatibility
- Memory management is handled automatically by the host

### `fetch`

**Declaration:** `host_fetch(url: string, request: Uint8Array): Uint8Array`

**Purpose:** Performs HTTP requests to external services.

**Parameters:**
- `url`: Target URL for the HTTP request
- `request`: Protocol Buffer encoded `HttpRequest` message

**Returns:** `HostResult` containing Protocol Buffer encoded `HttpResponse` message

**Error Handling:**
- Network errors will result in `NetworkError` (code 4) with descriptive messages
- Invalid URLs will result in `MemoryError` (code 2) or `NetworkError` (code 4)
- Serialization errors will result in `SerializationError` (code 6)
- The function blocks until the HTTP request completes

**HttpRequest Structure:**
```protobuf
message HttpRequest {
    optional string url     = 1;
    optional string method  = 2;
    repeated Pair headers   = 3;
    optional string body    = 4;
}
```

**HttpResponse Structure:**
```protobuf
message HttpResponse {
    optional int32 status_code = 1;
    repeated Pair headers      = 2;
    optional string body       = 3;
}
```

**Pair Structure:**
```protobuf
message Pair {
    optional string key     = 1;
    optional string value   = 2;
}
```

**Usage Example:**
```typescript
try {
    // Create HTTP request
    let request = new HttpRequest();
    request.setMethod("GET");
    request.setUrl("https://api.example.com/data");

    // Add headers
    let authHeader = new Pair();
    authHeader.setKey("Authorization");
    authHeader.setValue("Bearer token123");
    request.getHeadersList().push(authHeader);

    let contentTypeHeader = new Pair();
    contentTypeHeader.setKey("Content-Type");
    contentTypeHeader.setValue("application/json");
    request.getHeadersList().push(contentTypeHeader);

    // For POST requests, add body
    // request.setBody(JSON.stringify({key: "value"}));

    // Serialize request to protobuf
    let requestBytes = request.serializeBinary();

    // Make the HTTP request
    let resultBytes = host_fetch("https://api.example.com/data", requestBytes);

    // Handle the unified result
    let response = handleHostResult(resultBytes, (data) => 
        HttpResponse.deserializeBinary(data)
    );

    if (response.getStatusCode() == 200) {
        console.log("Response body:", response.getBody());
        
        // Access response headers
        for (let header of response.getHeadersList()) {
            console.log(`${header.getKey()}: ${header.getValue()}`);
        }
    }
} catch (error) {
    console.error("HTTP request failed:", error.message);
}
```

## Standard I/O Functions

### `readline`

**Declaration:** `host_readline(prompt: string): Uint8Array`

**Purpose:** Reads a line of input from standard input (stdin) with an optional prompt.

**Parameters:**
- `prompt`: Optional prompt string to display before reading input. Pass empty string or null for no prompt.

**Returns:** `HostResult` containing `StringResponse` with the user input (without trailing newline)

**Behavior:**
- Displays the prompt (if provided) and waits for user input
- Reads until a newline character is encountered
- Automatically removes trailing newline and carriage return characters
- Returns empty string on read error or EOF
- Blocks execution until input is received

**StringResponse Structure:**
```protobuf
message StringResponse {
    optional string value = 1;  // String value
}
```

**Usage Example:**
```typescript
try {
    // Read input with a prompt
    let resultBytes = host_readline("Enter your name: ");
    let response = handleHostResult(resultBytes, (data) => 
        StringResponse.deserializeBinary(data)
    );
    let name = response.getValue();
    console.log(`Hello, ${name}!`);

    // Read input without a prompt
    let inputBytes = host_readline("");
    let inputResponse = handleHostResult(inputBytes, (data) => 
        StringResponse.deserializeBinary(data)
    );
    console.log(`You entered: ${inputResponse.getValue()}`);

    // Interactive menu example
    function showMenu(): string {
        console.log("=== Robot Control Menu ===");
        console.log("1. Move to position");
        console.log("2. Get servo info");
        console.log("3. Exit");
        
        let resultBytes = host_readline("Select option (1-3): ");
        let response = handleHostResult(resultBytes, (data) => 
            StringResponse.deserializeBinary(data)
        );
        return response.getValue();
    }

    // Simple calculator example
    function calculator(): void {
        while (true) {
            let resultBytes = host_readline("Enter expression (or 'quit' to exit): ");
            let response = handleHostResult(resultBytes, (data) => 
                StringResponse.deserializeBinary(data)
            );
            let expression = response.getValue();
            
            if (expression === "quit") {
                break;
            }
            
            // Parse and evaluate expression
            console.log(`Result: ${evaluateExpression(expression)}`);
        }
    }
} catch (error) {
    console.error("Readline operation failed:", error.message);
}
```

**Error Handling:**
- I/O errors result in an empty string return value
- The function logs errors but does not throw exceptions
- EOF conditions are handled gracefully

## Utility Functions

### `__new_bytes` (Exported to Host)

**Declaration:** `__new_bytes(len: i32): Uint8Array`

**Purpose:** Allocates a new byte array in WASM memory for host-to-WASM data transfer.

**Parameters:**
- `len`: Length of the byte array to allocate

**Returns:** Newly allocated Uint8Array

**Note:** This function is exported FROM the WASM module TO the host, not the other way around.

## Data Serialization Formats



### Protocol Buffers
Used for:
- Servo information exchange (ServoInfoList, ServoRawParamList) with radian-based positioning
- Raw parameter data for advanced servo configuration
- Action execution (TargetRadianAction, DeltaRadianAction, ActionResult) with radian coordinates
- HTTP request/response messages
- Structured host communication with type safety and schema evolution

### Model Context Protocol (MCP)
Used for:
- MCP service connections via Server-Sent Events transport
- Client-server communication following MCP specification
- Asynchronous message handling for AI model context sharing

## Error Handling

All host functions use the unified `HostResult` system for consistent error handling:

### Error Categories

1. **Success (0):** Operation completed successfully
2. **Invalid Parameter (1):** Input parameters are invalid or malformed
3. **Memory Error (2):** WASM memory access or allocation failures
4. **Servo Error (3):** Hardware communication or servo operation failures
5. **Network Error (4):** HTTP requests, MCP connections, or network communication failures
6. **MCP Error (5):** Model Context Protocol specific errors
7. **Serialization Error (6):** Protocol buffer serialization/deserialization failures
8. **Internal Error (7):** Unexpected internal system errors

### Error Handling Best Practices

```typescript
// Always wrap host function calls in try-catch blocks
try {
    let result = handleHostResult(hostFunctionCall(), parseFunction);
    // Process successful result
} catch (error) {
    // Handle specific error types based on error codes
    if (error.message.includes("[3]")) {
        console.error("Servo hardware error:", error.message);
        // Implement servo-specific error recovery
    } else if (error.message.includes("[4]")) {
        console.error("Network error:", error.message);
        // Implement network retry logic
    } else {
        console.error("Unexpected error:", error.message);
        // General error handling
    }
}
```

### Migration from Legacy Functions

For functions not yet migrated to the unified system, they may still return direct protobuf messages or simple values. Check the function documentation to determine the return format.

### Migration from Position-Based to Radian-Based System

**Breaking Changes:**
- `TargetPositionAction` → `TargetRadianAction` (position values now in radians)
- `DeltaPositionAction` → `DeltaRadianAction` (delta values now in radians)
- `ActionResult.current_position_vec` → `ActionResult.current_radian_vec`
- `ServoInfo.position` → removed (use action functions to get current position)
- `ServoInfo.min_position/max_position` → `ServoInfo.min_rad/max_rad` (now in radians)
- `set_target_posistions` → `set_target_radians` (positions now in radians)

**Migration Example:**
```typescript
// OLD: Position-based system
let action = new TargetPositionAction();
action.setServoIdVec([1]);
action.setTargetPositionVec([2048]); // Raw position value

// NEW: Radian-based system
let action = new TargetRadianAction();
action.setServoIdVec([1]);
action.setTargetRadVec([Math.PI/2]); // π/2 radians (90 degrees)

// OLD: Reading position results
console.log(`Position: ${result.getCurrentPositionVec()[0]}`);

// NEW: Reading radian results
console.log(`Position: ${result.getCurrentRadianVec()[0]} radians`);
console.log(`Position: ${result.getCurrentRadianVec()[0] * 180 / Math.PI} degrees`);
```