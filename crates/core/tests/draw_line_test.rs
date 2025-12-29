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
fn test_draw_line_parsing() {
    // Data from testLayoutAndValues in androidx_samples.rs
    // OpCode 47 (0x2f), x1 0.0, y1 0.0, x2 NaN (Variable 42), y2 NaN (Variable 43)
    let hex_data = "2f0000000000000000ff80002aff80002b";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read DrawLine");

    assert_eq!(n, 17); // 1 opcode + 16 data bytes
    if let Operations::DrawLine { x1, y1, x2, y2 } = op {
        assert_eq!(x1, FloatExpression::Value(0.0));
        assert_eq!(y1, FloatExpression::Value(0.0));
        assert_eq!(x2, FloatExpression::Variable(42));
        assert_eq!(y2, FloatExpression::Variable(43));
    } else {
        panic!("Expected DrawLine operation, got {:?}", op);
    }
}
