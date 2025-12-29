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
fn test_modifier_padding_parsing() {
    // Data from testSimple in androidx_samples.rs
    // OpCode 58 (0x3a), left 22.0, top 22.0, right 22.0, bottom 22.0
    let hex_data = "3a41b0000041b0000041b0000041b00000";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read ModifierPadding");

    assert_eq!(n, 17); // 1 opcode + 16 data bytes
    if let Operations::ModifierPadding {
        left,
        top,
        right,
        bottom,
    } = op
    {
        assert_eq!(left, FloatExpression::Value(22.0));
        assert_eq!(top, FloatExpression::Value(22.0));
        assert_eq!(right, FloatExpression::Value(22.0));
        assert_eq!(bottom, FloatExpression::Value(22.0));
    } else {
        panic!("Expected ModifierPadding operation, got {:?}", op);
    }
}
