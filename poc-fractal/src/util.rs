pub fn snap_f32(value: f32, snap_modulus: f32) -> f32 {
    (value / snap_modulus).round() * snap_modulus
}

pub fn dist_to_snap_f32(value: f32, snap_modulus: f32) -> f32 {
    dbg!(snap_f32(value, snap_modulus)) - dbg!(value)
}
