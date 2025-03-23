pub fn add_alpha_3(color: &str, alpha: f32) -> String {
    if color.len() == 4 {
        let alpha_digit = (alpha * 15.0).round() as u8;
        format!("{}{:x}", color, alpha_digit)
    } else {
        let alpha_digit = (alpha * 255.0).round() as u8;
        format!("{}{:x}", color, alpha_digit)
    }
}
