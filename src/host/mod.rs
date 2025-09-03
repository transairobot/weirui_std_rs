// @generated

mod pb;
pub mod host_func;

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
    ptr as i32 + 4
}

#[no_mangle]
pub static WEIRUI_CLIENT_LANGUAGE: i32 = 1;