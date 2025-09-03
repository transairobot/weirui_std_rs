use super::pb::host_pb::*;
use protobuf::Message;
// External host function declarations
extern "C" {
    fn console_write(ptr: i32, len: i32) -> i32;
    fn run_target_action(ptr: i32, len: i32) -> i32;
    fn get_actuator_info(ptr: i32, len: i32) -> i32;
    fn get_joint_info(ptr: i32, len: i32) -> i32;
}

pub fn handle_response<T>(resp_ptr: *mut u8) -> anyhow::Result<T>
where
    T: Message,
{
    if resp_ptr.is_null() {
        anyhow::bail!("Received null pointer from host")
    }

    unsafe {
        // Read the length prefix (first 4 bytes)
        let len_ptr = resp_ptr.offset(-4) as *const u32;
        let len = *len_ptr as usize;

        // Create a slice from the response pointer
        let resp_slice = std::slice::from_raw_parts(resp_ptr, len);

        // Deserialize the response
        let result= HostResult::parse_from_bytes(resp_slice)?;
        if let Some(err_code) = result.error_code {
            if err_code != 0 {
                anyhow::bail!("Host error: {:?}", result.error_message);
            }
        }
        if let Some(data) = result.data {
            let message: T = T::parse_from_bytes(&data.to_owned())?;
            // Free the allocated memory
            let _ = Vec::from_raw_parts(len_ptr as *mut u8, len + 4, len + 4);
            return Ok(message);
        }
        let _ = Vec::from_raw_parts(len_ptr as *mut u8, len + 4, len + 4);
        Ok(T::default())
    }
}

// Public API functions
pub fn write_console(message: &str) -> anyhow::Result<()> {
    let mut req = ConsoleWriteReq::new();
    req.set_message(message.to_string());

    let mut buf = Vec::new();
    req.write_to_vec(&mut buf)?;
    let result_ptr = unsafe { console_write(buf.as_ptr() as i32, buf.len() as i32) };
    let _ = handle_response::<EmptyDummy>(result_ptr as *mut u8)?;
    Ok(())
}

pub fn run_actuator_targets(servo_ids: &[u32], target_radians: &[f32]) -> anyhow::Result<()> {
    let mut req = RunTargetActionReq::new();
    req.servo_id_vec = servo_ids.to_vec();
    req.target_rad_vec = target_radians.to_vec();

    let mut buf = Vec::new();
    req.write_to_vec(&mut buf)?;
    let result_ptr = unsafe { run_target_action(buf.as_ptr() as i32, buf.len() as i32) };
    let _ = handle_response::<RunTargetActionResp>(result_ptr as *mut u8)?;
    Ok(())
}

pub fn get_actuators() -> anyhow::Result<Vec<ActuatorInfo>> {
    let req = GetActuatorInfoReq::new();

    let mut buf = Vec::new();
    req.write_to_vec(&mut buf)?;
    let result_ptr = unsafe { get_actuator_info(buf.as_ptr() as i32, buf.len() as i32) };
    let resp = handle_response::<GetActuatorInfoResp>(result_ptr as *mut u8)?;
    Ok(resp.actuators)
}

pub fn get_joints() -> anyhow::Result<Vec<JointInfo>> {
    let req = GetJointInfoReq::new();

    let mut buf = Vec::new();
    req.write_to_vec(&mut buf)?;
    let result_ptr = unsafe { get_joint_info(buf.as_ptr() as i32, buf.len() as i32) };
    let resp = handle_response::<GetJointInfoResp>(result_ptr as *mut u8)?;
    Ok(resp.joints)
}
