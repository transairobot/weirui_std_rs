use weirui_std_rs::*;
use std::borrow::Cow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Weirui Robot Control Example");
    
    // Example 1: Get servo information
    println!("\n=== Servo Information ===");
    let servo_ids = [1u8, 2u8, 3u8];
    match servo_info(&servo_ids) {
        Ok(info_list) => {
            for info in &info_list.infos {
                if let Some(servo_id) = info.servo_id {
                    println!("Servo {}: {:?}", servo_id, info.name);
                    if let (Some(min), Some(max)) = (info.min_rad, info.max_rad) {
                        println!("  Range: {:.3} to {:.3} radians", min, max);
                        println!("  Range: {:.1} to {:.1} degrees", 
                            radians_to_degrees(min), radians_to_degrees(max));
                    }
                }
            }
        }
        Err(e) => println!("Failed to get servo info: {}", e),
    }
    
    // Example 2: Move servo to specific angle
    println!("\n=== Move Servo to Angle ===");
    let target_angle = degrees_to_radians(90.0); // 90 degrees
    match move_servo_to_angle(1, target_angle, 100, 50) {
        Ok(result) => {
            if let Some(finished) = result.finish_vec.first() {
                println!("Movement finished: {}", finished);
            }
            if let Some(current_pos) = result.current_radian_vec.first() {
                println!("Current position: {:.3} radians ({:.1} degrees)", 
                    current_pos, radians_to_degrees(*current_pos));
            }
        }
        Err(e) => println!("Failed to move servo: {}", e),
    }
    
    // Example 3: Move multiple servos
    println!("\n=== Move Multiple Servos ===");
    let servo_ids = [1u8, 2u8];
    let angles = [degrees_to_radians(45.0), degrees_to_radians(-30.0)];
    match move_servos_to_angles(&servo_ids, &angles) {
        Ok(result) => {
            println!("Movement results:");
            for (i, finished) in result.finish_vec.iter().enumerate() {
                if let Some(current_pos) = result.current_radian_vec.get(i) {
                    println!("  Servo {}: finished={}, pos={:.1}°", 
                        servo_ids[i], finished, radians_to_degrees(*current_pos));
                }
            }
        }
        Err(e) => println!("Failed to move servos: {}", e),
    }
    
    // Example 4: Delta movement
    println!("\n=== Delta Movement ===");
    let delta = degrees_to_radians(15.0); // Move 15 degrees from current position
    match move_servo_by_delta(1, delta) {
        Ok(result) => {
            if let Some(current_pos) = result.current_radian_vec.first() {
                println!("New position after delta: {:.1} degrees", 
                    radians_to_degrees(*current_pos));
            }
        }
        Err(e) => println!("Failed to perform delta movement: {}", e),
    }
    
    // Example 5: End effector movement
    println!("\n=== End Effector Movement ===");
    let end_effector_action = EndEffectorAction {
        delta_x: Some(0.05), // Move 5cm in X direction
        delta_y: Some(0.0),
        delta_z: Some(0.02), // Move 2cm up in Z direction
        urdf_file_path: None, // Use default URDF
        target_link_name: None, // Use default target link
    };
    
    match run_end_effector_action(&end_effector_action) {
        Ok(result) => {
            println!("End effector movement completed");
            for (i, pos) in result.current_radian_vec.iter().enumerate() {
                println!("  Joint {}: {:.1} degrees", i+1, radians_to_degrees(*pos));
            }
        }
        Err(e) => println!("Failed to move end effector: {}", e),
    }
    
    // Example 6: HTTP request
    println!("\n=== HTTP Request ===");
    let http_request = HttpRequest {
        url: Some(Cow::Borrowed("https://api.github.com/repos/rust-lang/rust")),
        method: Some(Cow::Borrowed("GET")),
        headers: vec![
            Pair {
                key: Some(Cow::Borrowed("User-Agent")),
                value: Some(Cow::Borrowed("weirui-robot/1.0")),
            }
        ],
        body: None,
    };
    
    match fetch("https://api.github.com/repos/rust-lang/rust", &http_request) {
        Ok(response) => {
            if let Some(status) = response.status_code {
                println!("HTTP Status: {}", status);
            }
            if let Some(body) = &response.body {
                println!("Response length: {} bytes", body.len());
            }
        }
        Err(e) => println!("HTTP request failed: {}", e),
    }
    
    // Example 7: Interactive input
    println!("\n=== Interactive Input ===");
    match readline("Enter a command: ") {
        Ok(input) => {
            println!("You entered: '{}'", input);
            
            // Parse simple commands
            match input.trim() {
                "home" => {
                    println!("Moving all servos to home position (0 degrees)");
                    let servo_ids = [1u8, 2u8, 3u8];
                    let home_angles = [0.0f32; 3];
                    if let Err(e) = move_servos_to_angles(&servo_ids, &home_angles) {
                        println!("Failed to move to home: {}", e);
                    }
                }
                "disable" => {
                    println!("Disabling torque for all servos");
                    disable_torque(&[1u8, 2u8, 3u8]);
                }
                "enable" => {
                    println!("Enabling torque for all servos");
                    enable_torque(&[1u8, 2u8, 3u8]);
                }
                _ => println!("Unknown command. Try: home, disable, enable"),
            }
        }
        Err(e) => println!("Failed to read input: {}", e),
    }
    
    // Example 8: Angle utilities
    println!("\n=== Angle Utilities ===");
    let angle_deg = 45.0;
    let angle_rad = degrees_to_radians(angle_deg);
    let normalized = normalize_radians(angle_rad + 10.0 * std::f32::consts::PI);
    
    println!("{} degrees = {:.3} radians", angle_deg, angle_rad);
    println!("Normalized angle: {:.3} radians ({:.1} degrees)", 
        normalized, radians_to_degrees(normalized));
    
    Ok(())
}
