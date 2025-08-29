// Angle conversion utilities
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

pub fn normalize_radians(radians: f32) -> f32 {
    let two_pi = 2.0 * std::f32::consts::PI;
    let mut normalized = radians % two_pi;
    
    if normalized > std::f32::consts::PI {
        normalized -= two_pi;
    } else if normalized < -std::f32::consts::PI {
        normalized += two_pi;
    }
    
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        assert!((degrees_to_radians(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((degrees_to_radians(90.0) - std::f32::consts::PI / 2.0).abs() < f32::EPSILON);
        assert!((degrees_to_radians(180.0) - std::f32::consts::PI).abs() < f32::EPSILON);
        assert!((degrees_to_radians(-90.0) + std::f32::consts::PI / 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_radians_to_degrees() {
        assert!((radians_to_degrees(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((radians_to_degrees(std::f32::consts::PI / 2.0) - 90.0).abs() < f32::EPSILON);
        assert!((radians_to_degrees(std::f32::consts::PI) - 180.0).abs() < f32::EPSILON);
        assert!((radians_to_degrees(-std::f32::consts::PI / 2.0) + 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalize_radians() {
        // Test values within range
        assert!((normalize_radians(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((normalize_radians(std::f32::consts::PI / 2.0) - std::f32::consts::PI / 2.0).abs() < f32::EPSILON);
        
        // Test values outside range - use larger tolerance for floating point precision
        let result1 = normalize_radians(3.0 * std::f32::consts::PI);
        let expected1 = std::f32::consts::PI;
        assert!((result1 - expected1).abs() < 0.0001, 
            "Expected {}, got {}, diff: {}", expected1, result1, (result1 - expected1).abs());
        
        let result2 = normalize_radians(-3.0 * std::f32::consts::PI);
        let expected2 = -std::f32::consts::PI;
        assert!((result2 - expected2).abs() < 0.0001,
            "Expected {}, got {}, diff: {}", expected2, result2, (result2 - expected2).abs());
        
        // Test edge cases
        assert!((normalize_radians(std::f32::consts::PI) - std::f32::consts::PI).abs() < f32::EPSILON);
        assert!((normalize_radians(-std::f32::consts::PI) + std::f32::consts::PI).abs() < f32::EPSILON);
    }

    #[test]
    fn test_round_trip_conversion() {
        let test_angles = [0.0, 45.0, 90.0, 135.0, 180.0, -45.0, -90.0, -135.0, -180.0];
        
        for &angle in &test_angles {
            let radians = degrees_to_radians(angle);
            let back_to_degrees = radians_to_degrees(radians);
            assert!((angle - back_to_degrees).abs() < 0.001, 
                "Round trip failed for {}: got {}", angle, back_to_degrees);
        }
    }
}
