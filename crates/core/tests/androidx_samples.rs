use remote_compose_core::document::Document;

fn parse_sample(name: &str, data: &[u8], lenient: bool) {
    println!("--- Testing {} ({} bytes) ---", name, data.len());
    let doc = if lenient {
        Document::parse_lenient(data)
    } else {
        Document::parse(data)
    };
    match doc {
        Ok(_) => {}
        Err(e) => {
            panic!("Parsing failed for {}: {}", name, e);
        }
    }
}

#[test]
fn test_simple() {
    parse_sample("testSimple", include_bytes!("../examples/simple.rc"), false);
}

#[test]
fn test_color_filter1() {
    parse_sample(
        "testColorFilter1",
        include_bytes!("../examples/color_filter1.rc"),
        // TODO fix and set to false
        true,
    );
}

#[test]
fn test_color_filter2() {
    parse_sample(
        "testColorFilter2",
        include_bytes!("../examples/color_filter2.rc"),
        // TODO fix and set to false
        true,
    );
}

#[test]
fn test_basic_click_action_param() {
    parse_sample(
        "testBasicClickActionParam",
        include_bytes!("../examples/basic_click_action_param.rc"),
        false,
    );
}

#[test]
fn test_touch() {
    parse_sample("testTouch", include_bytes!("../examples/touch.rc"), false);
}

#[test]
fn test_intrinsics1() {
    parse_sample(
        "testIntrinsics1",
        include_bytes!("../examples/intrinsics1.rc"),
        false,
    );
}

#[test]
fn test_intrinsics2() {
    parse_sample(
        "testIntrinsics2",
        include_bytes!("../examples/intrinsics2.rc"),
        false,
    );
}

#[test]
fn test_layout_and_values() {
    parse_sample(
        "testLayoutAndValues",
        include_bytes!("../examples/layout_and_values.rc"),
        false,
    );
}

#[test]
fn test_basic_click_action() {
    parse_sample(
        "testBasicClickAction",
        include_bytes!("../examples/basic_click_action.rc"),
        false,
    );
}
