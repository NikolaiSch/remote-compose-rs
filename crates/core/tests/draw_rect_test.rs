use remote_compose_core::operations::Operations;
use remote_compose_expressions::FloatExpression;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn test_draw_rect_parsing() {
    // OpCode 42 (0x2a), Left 10.0 (0x41200000), Top 20.0 (0x41a00000), Right 30.0 (0x41f00000), Bottom 40.0 (0x42200000)
    let hex_data = "2a4120000041a0000041f0000042200000";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read DrawRect");

    assert_eq!(n, 17); // 1 opcode + 16 data bytes
    if let Operations::DrawRect {
        left,
        top,
        right,
        bottom,
    } = op
    {
        assert_eq!(left, FloatExpression::Value(10.0));
        assert_eq!(top, FloatExpression::Value(20.0));
        assert_eq!(right, FloatExpression::Value(30.0));
        assert_eq!(bottom, FloatExpression::Value(40.0));
    } else {
        panic!("Expected DrawRect operation, got {:?}", op);
    }
}
