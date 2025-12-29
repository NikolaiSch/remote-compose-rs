use crate::opcode::OpCode;

pub const NAN_OFFSET: u32 = 0x310000;
pub const NAN_MASK: u32 = 0xFF800000;

/// Helper to create an opcode float.
pub fn as_nan(op: OpCode) -> f32 {
    let bits = (op as u32) + NAN_OFFSET | NAN_MASK;
    f32::from_bits(bits)
}

pub fn nan_payload(v: f32) -> u32 {
    v.to_bits() & 0x7FFFFF
}

pub fn id_from_nan(v: f32) -> u32 {
    v.to_bits() & 0x3FFFFF
}

pub fn is_system_variable(v: f32) -> bool {
    (nan_payload(v) >> 20) == 0
}

pub fn is_normal_variable(v: f32) -> bool {
    (nan_payload(v) >> 20) == 1
}

pub fn is_data_variable(v: f32) -> bool {
    (nan_payload(v) >> 20) == 2
}

pub fn is_var1(v: f32) -> bool {
    v.to_bits() == as_nan(OpCode::Var1).to_bits()
}

pub fn is_operation_variable(v: f32) -> bool {
    (nan_payload(v) >> 20) == 3
}

pub fn bezier_coord(p1: f32, p2: f32, t: f32) -> f32 {
    3.0 * (1.0 - t).powi(2) * t * p1 + 3.0 * (1.0 - t) * t.powi(2) * p2 + t.powi(3)
}

pub fn cubic_easing(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    // Binary search for t_bezier such that x_bezier(t_bezier) = t
    let mut low = 0.0;
    let mut high = 1.0;
    for _ in 0..12 {
        let mid = (low + high) / 2.0;
        let x = bezier_coord(x1, x2, mid);
        if x < t {
            low = mid;
        } else {
            high = mid;
        }
    }

    bezier_coord(y1, y2, low)
}
