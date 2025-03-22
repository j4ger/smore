pub fn add_alpha_3(color: &str, alpha: f32) -> String {
    let alpha_digit = (alpha * 15.0).round() as u8;
    format!("{}{:x}", color, alpha_digit)
}
