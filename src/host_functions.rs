use crate::protobuf::wasm_host::*;
use crate::error::{HostError, HostErrorCode};
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, BytesWriter};
use std::borrow::Cow;

// External host function declarations
unsafe extern "C" {
    fn host_servo_info(ids_ptr: *const u8, ids_len: usize) -> *mut u8;
    fn host_servo_raw_param(ids_ptr: *const u8, ids_len: usize) -> *mut u8;
    fn host_run_target_action(action_ptr: *const u8, action_len: usize) -> *mut u8;
    fn host_run_delta_action(action_ptr: *const u8, action_len: usize) -> *mut u8;
    fn host_run_end_effector_action(action_ptr: *const u8, action_len: usize) -> *mut u8;
    fn host_fetch(url_ptr: *const u8, url_len: usize, request_ptr: *const u8, request_len: usize) -> *mut u8;
    fn host_readline(prompt_ptr: *const u8, prompt_len: usize) -> *mut u8;
    fn host_set_target_radians(ids_ptr: *const u8, ids_len: usize, radians_ptr: *const f32, speeds_ptr: *const i32, accel_ptr: *const i32);
    fn host_connect_mcp_service(url_ptr: *const u8, url_len: usize) -> i32;
    fn host_list_all_tools(handle_id: i32) -> *mut u8;
    fn host_call_tool(handle_id: i32, request_ptr: *const u8, request_len: usize) -> *mut u8;
    fn host_register_tool(func_ptr: i32, tool_ptr: *const u8, tool_len: usize) -> i32;
    fn host_disable_torque(ids_ptr: *const u8, ids_len: usize);
    fn host_enable_torque(ids_ptr: *const u8, ids_len: usize);
}

// Helper function to serialize messages
fn serialize_message<T: MessageWrite>(msg: &T) -> Result<Vec<u8>, HostError> {
    let mut buf = Vec::new();
    {
        let mut writer = Writer::new(BytesWriter::new(&mut buf));
        msg.write_message(&mut writer)
            .map_err(|e| HostError::new(HostErrorCode::SerializationError, format!("Serialization failed: {}", e)))?;
    }
    Ok(buf)
}

// Helper function to deserialize messages
fn deserialize_message<'a, T: MessageRead<'a>>(bytes: &'a [u8]) -> Result<T, HostError> {
    let mut reader = BytesReader::from_bytes(bytes);
    T::from_reader(&mut reader, bytes)
        .map_err(|e| HostError::new(HostErrorCode::SerializationError, format!("Deserialization failed: {}", e)))
}

// Helper function to handle HostResult responses - returns owned data
fn handle_host_result<T>(result_ptr: *mut u8, parse_fn: impl Fn(&[u8]) -> Result<T, HostError>) -> Result<T, HostError> {
    if result_ptr.is_null() {
        return Err(HostError::new(HostErrorCode::MemoryError, "Null pointer returned".to_string()));
    }
    
    unsafe {
        // Read the length first (assuming it's stored as u32 at the beginning)
        let len_ptr = result_ptr as *const u32;
        let len = *len_ptr as usize;
        let data_ptr = result_ptr.offset(4) as *const u8;
        let result_bytes = std::slice::from_raw_parts(data_ptr, len);
        
        let host_result: HostResult = deserialize_message(result_bytes)?;
        
        if let Some(error_code) = host_result.error_code {
            if error_code != 0 {
                let error_msg = host_result.error_message.unwrap_or(Cow::Borrowed("Unknown error"));
                return Err(HostError::new(
                    match error_code {
                        1 => HostErrorCode::InvalidParameter,
                        2 => HostErrorCode::MemoryError,
                        3 => HostErrorCode::ServoError,
                        4 => HostErrorCode::NetworkError,
                        5 => HostErrorCode::McpError,
                        6 => HostErrorCode::SerializationError,
                        _ => HostErrorCode::InternalError,
                    },
                    error_msg.to_string()
                ));
            }
        }
        
        let data = host_result.data.ok_or_else(|| 
            HostError::new(HostErrorCode::InternalError, "No data in successful result".to_string()))?;
        
        parse_fn(&data)
    }
}

// Servo control functions
pub fn servo_info(ids: &[u8]) -> Result<ServoInfoList<'static>, HostError> {
    let result_ptr = unsafe { host_servo_info(ids.as_ptr(), ids.len()) };
    handle_host_result(result_ptr, |bytes| {
        // Convert to owned data
        let owned_bytes = bytes.to_vec();
        let servo_info: ServoInfoList = deserialize_message(&owned_bytes)?;
        Ok(ServoInfoList {
            infos: servo_info.infos.into_iter().map(|info| ServoInfo {
                servo_id: info.servo_id,
                name: info.name.map(|n| Cow::Owned(n.into_owned())),
                min_rad: info.min_rad,
                max_rad: info.max_rad,
                resolution: info.resolution,
            }).collect(),
        })
    })
}

pub fn servo_raw_param(ids: &[u8]) -> Result<ServoRawParamList<'static>, HostError> {
    let result_ptr = unsafe { host_servo_raw_param(ids.as_ptr(), ids.len()) };
    handle_host_result(result_ptr, |bytes| {
        let owned_bytes = bytes.to_vec();
        let param_list: ServoRawParamList = deserialize_message(&owned_bytes)?;
        Ok(ServoRawParamList {
            params: param_list.params.into_iter().map(|param| ServoRawParam {
                servo_id: param.servo_id,
                params: param.params.into_iter().map(|entry| ParamEntry {
                    key: entry.key.map(|k| Cow::Owned(k.into_owned())),
                    value: entry.value,
                }).collect(),
            }).collect(),
        })
    })
}

pub fn run_target_action(action: &TargetRadianAction) -> Result<ActionResult, HostError> {
    let action_bytes = serialize_message(action)?;
    let result_ptr = unsafe { host_run_target_action(action_bytes.as_ptr(), action_bytes.len()) };
    handle_host_result(result_ptr, |bytes| {
        let owned_bytes = bytes.to_vec();
        deserialize_message(&owned_bytes)
    })
}

pub fn run_delta_action(action: &DeltaRadianAction) -> Result<ActionResult, HostError> {
    let action_bytes = serialize_message(action)?;
    let result_ptr = unsafe { host_run_delta_action(action_bytes.as_ptr(), action_bytes.len()) };
    handle_host_result(result_ptr, |bytes| {
        let owned_bytes = bytes.to_vec();
        deserialize_message(&owned_bytes)
    })
}

pub fn run_end_effector_action(action: &EndEffectorAction) -> Result<ActionResult, HostError> {
    let action_bytes = serialize_message(action)?;
    let result_ptr = unsafe { host_run_end_effector_action(action_bytes.as_ptr(), action_bytes.len()) };
    handle_host_result(result_ptr, |bytes| {
        let owned_bytes = bytes.to_vec();
        deserialize_message(&owned_bytes)
    })
}

pub fn set_target_radians(ids: &[u8], radians: &[f32], speeds: &[i32], accelerations: &[i32]) {
    unsafe {
        host_set_target_radians(
            ids.as_ptr(), ids.len(),
            radians.as_ptr(),
            speeds.as_ptr(),
            accelerations.as_ptr()
        );
    }
}

pub fn disable_torque(ids: &[u8]) {
    unsafe { host_disable_torque(ids.as_ptr(), ids.len()); }
}

pub fn enable_torque(ids: &[u8]) {
    unsafe { host_enable_torque(ids.as_ptr(), ids.len()); }
}

// Network functions
pub fn fetch(url: &str, request: &HttpRequest) -> Result<HttpResponse<'static>, HostError> {
    let request_bytes = serialize_message(request)?;
    let result_ptr = unsafe { 
        host_fetch(
            url.as_ptr(), url.len(),
            request_bytes.as_ptr(), request_bytes.len()
        ) 
    };
    handle_host_result(result_ptr, |bytes| {
        let owned_bytes = bytes.to_vec();
        let response: HttpResponse = deserialize_message(&owned_bytes)?;
        Ok(HttpResponse {
            status_code: response.status_code,
            headers: response.headers.into_iter().map(|pair| Pair {
                key: pair.key.map(|k| Cow::Owned(k.into_owned())),
                value: pair.value.map(|v| Cow::Owned(v.into_owned())),
            }).collect(),
            body: response.body.map(|b| Cow::Owned(b.into_owned())),
        })
    })
}

// I/O functions
pub fn readline(prompt: &str) -> Result<String, HostError> {
    let result_ptr = unsafe { host_readline(prompt.as_ptr(), prompt.len()) };
    handle_host_result(result_ptr, |bytes| {
        let owned_bytes = bytes.to_vec();
        let response: StringResponse = deserialize_message(&owned_bytes)?;
        Ok(response.value.unwrap_or(Cow::Borrowed("")).to_string())
    })
}

// MCP functions
pub fn connect_mcp_service(url: &str) -> i32 {
    unsafe { host_connect_mcp_service(url.as_ptr(), url.len()) }
}

pub fn list_all_tools(handle_id: i32) -> Result<McpToolList<'static>, HostError> {
    let result_ptr = unsafe { host_list_all_tools(handle_id) };
    if result_ptr.is_null() {
        return Err(HostError::new(HostErrorCode::McpError, "Failed to list tools".to_string()));
    }
    
    unsafe {
        let len_ptr = result_ptr as *const u32;
        let len = *len_ptr as usize;
        let data_ptr = result_ptr.offset(4) as *const u8;
        let result_bytes = std::slice::from_raw_parts(data_ptr, len);
        let owned_bytes = result_bytes.to_vec();
        let tool_list: McpToolList = deserialize_message(&owned_bytes)?;
        Ok(McpToolList {
            tools: tool_list.tools.into_iter().map(|tool| McpTool {
                name: tool.name.map(|n| Cow::Owned(n.into_owned())),
                description: tool.description.map(|d| Cow::Owned(d.into_owned())),
                input_schema: tool.input_schema.map(|s| Cow::Owned(s.into_owned())),
            }).collect(),
        })
    }
}

pub fn call_tool(handle_id: i32, request: &McpCallToolRequest) -> Result<McpCallToolResponse<'static>, HostError> {
    let request_bytes = serialize_message(request)?;
    let result_ptr = unsafe { host_call_tool(handle_id, request_bytes.as_ptr(), request_bytes.len()) };
    
    if result_ptr.is_null() {
        return Err(HostError::new(HostErrorCode::McpError, "Failed to call tool".to_string()));
    }
    
    unsafe {
        let len_ptr = result_ptr as *const u32;
        let len = *len_ptr as usize;
        let data_ptr = result_ptr.offset(4) as *const u8;
        let result_bytes = std::slice::from_raw_parts(data_ptr, len);
        let owned_bytes = result_bytes.to_vec();
        let response: McpCallToolResponse = deserialize_message(&owned_bytes)?;
        Ok(McpCallToolResponse {
            content: response.content.map(|c| Cow::Owned(c.into_owned())),
            is_error: response.is_error,
        })
    }
}

pub fn register_tool(func_ptr: i32, tool: &McpTool) -> i32 {
    let tool_bytes = serialize_message(tool).unwrap_or_default();
    unsafe { host_register_tool(func_ptr, tool_bytes.as_ptr(), tool_bytes.len()) }
}

// Utility functions for common operations
pub fn move_servo_to_angle(servo_id: u8, angle_radians: f32, _speed: i32, _acceleration: i32) -> Result<ActionResult, HostError> {
    let action = TargetRadianAction {
        servo_id_vec: vec![servo_id as u32],
        target_rad_vec: vec![angle_radians],
    };
    run_target_action(&action)
}

pub fn move_servos_to_angles(servo_ids: &[u8], angles_radians: &[f32]) -> Result<ActionResult, HostError> {
    let action = TargetRadianAction {
        servo_id_vec: servo_ids.iter().map(|&id| id as u32).collect(),
        target_rad_vec: angles_radians.to_vec(),
    };
    run_target_action(&action)
}

pub fn move_servo_by_delta(servo_id: u8, delta_radians: f32) -> Result<ActionResult, HostError> {
    let action = DeltaRadianAction {
        servo_id_vec: vec![servo_id as u32],
        delta_rad_vec: vec![delta_radians],
    };
    run_delta_action(&action)
}

// Angle conversion utilities
pub use crate::utils::{degrees_to_radians, radians_to_degrees, normalize_radians};
