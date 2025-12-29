use remote_compose_core::operations::Operations;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn test_layout_root_parsing() {
    // Data from testSimple in androidx_samples.rs
    // OpCode 200 (0xc8), component_id -2 (0xfffffffe)
    let hex_data = "c8fffffffe";
    let data = hex_to_bytes(hex_data);

    let (op, n) = Operations::read(&data).expect("Failed to read LayoutRoot");

    assert_eq!(n, 5); // 1 opcode + 4 data bytes
    if let Operations::LayoutRoot { component_id, .. } = op {
        assert_eq!(component_id, -2);
    } else {
        panic!("Expected LayoutRoot operation, got {:?}", op);
    }
}
