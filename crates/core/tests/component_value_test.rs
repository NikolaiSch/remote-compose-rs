use remote_compose_core::operations::Operations;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn test_component_value_parsing() {
    // Data from testLayoutAndValues in androidx_samples.rs
    // OpCode 150 (0x96), type_ 0, component_id -7, value_id 42
    let hex_data = "9600000000fffffff90000002a";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read ComponentValue");

    assert_eq!(n, 13); // 1 opcode + 12 data bytes
    if let Operations::ComponentValue {
        type_,
        component_id,
        value_id,
    } = op
    {
        assert_eq!(type_, 0);
        assert_eq!(component_id, -7);
        assert_eq!(value_id, 42);
    } else {
        panic!("Expected ComponentValue operation, got {:?}", op);
    }
}
