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
fn test_matrix_translate_parsing() {
    // Data from testSimple in androidx_samples.rs
    // OpCode 127 (0x7f), tx 358.0, ty 413.0
    let hex_data = "7f43b3000043ce8000";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read MatrixTranslate");

    assert_eq!(n, 9); // 1 opcode + 8 data bytes
    if let Operations::MatrixTranslate { tx, ty } = op {
        assert_eq!(tx, FloatExpression::Value(358.0));
        assert_eq!(ty, FloatExpression::Value(413.0));
    } else {
        panic!("Expected MatrixTranslate operation, got {:?}", op);
    }
}
