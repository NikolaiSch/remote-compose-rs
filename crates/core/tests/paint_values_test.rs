use remote_compose_core::operations::{Operations, PaintChange};
use remote_compose_expressions::FloatExpression;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn test_paint_values_parsing() {
    // Data from testLayoutAndValues in androidx_samples.rs
    // OpCode 40 (0x28), count 14
    let hex_data = "280000000e00000004ff0000ff00000005408000000000000141400000000000070000000f000100080000001000000000000300120000000900000000";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read PaintValues");

    assert_eq!(n, 61); // 1 opcode + 4 count + 56 data bytes
    if let Operations::PaintValues(pv) = op {
        assert_eq!(pv.changes.len(), 9); // 14 raw values, but some are multi-value changes

        // 1. Color(0xff0000ff)
        if let PaintChange::Color(color) = pv.changes[0] {
            assert_eq!(color, 0xff0000ffu32 as i32);
        } else {
            panic!("Expected Color, got {:?}", pv.changes[0]);
        }

        // 2. StrokeWidth(4.0)
        if let PaintChange::StrokeWidth(expr) = &pv.changes[1] {
            assert_eq!(*expr, FloatExpression::Value(4.0));
        } else {
            panic!("Expected StrokeWidth, got {:?}", pv.changes[1]);
        }

        // 3. TextSize(12.0)
        if let PaintChange::TextSize(expr) = &pv.changes[2] {
            assert_eq!(*expr, FloatExpression::Value(12.0));
        } else {
            panic!("Expected TextSize, got {:?}", pv.changes[2]);
        }

        // 4. StrokeCap(0) - Wait, cmd was 0x00070000. 7 is STROKE_CAP. high bits are 0.
        assert_eq!(pv.changes[3], PaintChange::StrokeCap(0));

        // 5. StrokeJoin(0) - cmd was 0x000f0000. 15 is STROKE_JOIN. high bits are 0.
        assert_eq!(pv.changes[4], PaintChange::StrokeJoin(0));

        // 6. Style(1) - cmd was 0x00010008. 8 is STYLE. high bits are 1.
        assert_eq!(pv.changes[5], PaintChange::Style(1));

        // 7. Typeface { weight: 0, italic: false, font_data: false, font_type: 0 }
        // cmd was 0x00100000. 16 is TYPEFACE. style is 0. next val is 0.
        assert_eq!(
            pv.changes[6],
            PaintChange::Typeface {
                weight: 0,
                italic: false,
                font_data: false,
                font_type: 0,
            }
        );

        // 8. BlendMode(3) - cmd was 0x00120003. 18 is BLEND_MODE. high bits are 3.
        assert_eq!(pv.changes[7], PaintChange::BlendMode(3));

        // 9. Shader(0) - cmd was 0x00000009. 9 is SHADER. next val is 0.
        assert_eq!(pv.changes[8], PaintChange::Shader(0));
    } else {
        panic!("Expected PaintValues operation, got {:?}", op);
    }
}
