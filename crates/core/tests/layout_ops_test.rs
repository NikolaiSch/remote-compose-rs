use remote_compose_core::operations::{
    DrawLine, FloatExpression, FontStyle, LayoutAlignment, LayoutBox, LayoutColumn, LayoutImage,
    LayoutRow, LayoutText, ModifierBackground, ShapeType, TextAlign, TextOverflow,
};

#[test]
fn test_layout_column_read() {
    let mut data = Vec::new();
    // component_id: 10
    data.extend_from_slice(&10i32.to_be_bytes());
    // animation_id: 20
    data.extend_from_slice(&20i32.to_be_bytes());
    // horizontal_alignment: Center (2)
    data.extend_from_slice(&2i32.to_be_bytes());
    // vertical_alignment: Top (4)
    data.extend_from_slice(&4i32.to_be_bytes());
    // spaced_by: 16.0
    data.extend_from_slice(&16.0f32.to_be_bytes());

    let (op, bytes_read) = LayoutColumn::read(&data).expect("Failed to read LayoutColumn");
    assert_eq!(bytes_read, 20);
    assert_eq!(op.component_id, 10);
    assert_eq!(op.animation_id, 20);
    assert_eq!(op.horizontal_alignment, LayoutAlignment::Center);
    assert_eq!(op.vertical_alignment, LayoutAlignment::Top);
    assert_eq!(op.spaced_by, FloatExpression::Value(16.0));
}

#[test]
fn test_layout_row_read() {
    let mut data = Vec::new();
    data.extend_from_slice(&11i32.to_be_bytes());
    data.extend_from_slice(&21i32.to_be_bytes());
    data.extend_from_slice(&1i32.to_be_bytes()); // Start
    data.extend_from_slice(&5i32.to_be_bytes()); // Bottom
    data.extend_from_slice(&8.0f32.to_be_bytes());

    let (op, bytes_read) = LayoutRow::read(&data).expect("Failed to read LayoutRow");
    assert_eq!(bytes_read, 20);
    assert_eq!(op.component_id, 11);
    assert_eq!(op.animation_id, 21);
    assert_eq!(op.horizontal_alignment, LayoutAlignment::Start);
    assert_eq!(op.vertical_alignment, LayoutAlignment::Bottom);
    assert_eq!(op.spaced_by, FloatExpression::Value(8.0));
}

#[test]
fn test_layout_box_read() {
    let mut data = Vec::new();
    data.extend_from_slice(&12i32.to_be_bytes());
    data.extend_from_slice(&22i32.to_be_bytes());
    data.extend_from_slice(&3i32.to_be_bytes()); // End
    data.extend_from_slice(&2i32.to_be_bytes()); // Center

    let (op, bytes_read) = LayoutBox::read(&data).expect("Failed to read LayoutBox");
    assert_eq!(bytes_read, 16);
    assert_eq!(op.component_id, 12);
    assert_eq!(op.animation_id, 22);
    assert_eq!(op.horizontal_alignment, LayoutAlignment::End);
    assert_eq!(op.vertical_alignment, LayoutAlignment::Center);
}

#[test]
fn test_draw_line_read() {
    let mut data = Vec::new();
    data.extend_from_slice(&0.0f32.to_be_bytes());
    data.extend_from_slice(&0.0f32.to_be_bytes());
    data.extend_from_slice(&100.0f32.to_be_bytes());
    data.extend_from_slice(&100.0f32.to_be_bytes());

    let (op, bytes_read) = DrawLine::read(&data).expect("Failed to read DrawLine");
    assert_eq!(bytes_read, 16);
    assert_eq!(op.x1, FloatExpression::Value(0.0));
    assert_eq!(op.y1, FloatExpression::Value(0.0));
    assert_eq!(op.x2, FloatExpression::Value(100.0));
    assert_eq!(op.y2, FloatExpression::Value(100.0));
}

#[test]
fn test_modifier_background_read() {
    let mut data = Vec::new();
    data.extend_from_slice(&0i32.to_be_bytes()); // flags
    data.extend_from_slice(&1i32.to_be_bytes()); // color_id
    data.extend_from_slice(&0i32.to_be_bytes()); // reserve1
    data.extend_from_slice(&0i32.to_be_bytes()); // reserve2
    data.extend_from_slice(&0.0f32.to_be_bytes()); // r
    data.extend_from_slice(&1.0f32.to_be_bytes()); // g
    data.extend_from_slice(&0.0f32.to_be_bytes()); // b
    data.extend_from_slice(&1.0f32.to_be_bytes()); // a
    data.extend_from_slice(&0i32.to_be_bytes()); // shape_type (Rectangle)

    let (op, bytes_read) =
        ModifierBackground::read(&data).expect("Failed to read ModifierBackground");
    assert_eq!(bytes_read, 36);
    assert_eq!(op.color_id, 1);
    assert_eq!(op.g, FloatExpression::Value(1.0));
    assert_eq!(op.shape_type, ShapeType::Rectangle);
}

#[test]
fn test_layout_text_read() {
    let mut data = Vec::new();
    data.extend_from_slice(&13i32.to_be_bytes()); // component_id
    data.extend_from_slice(&23i32.to_be_bytes()); // animation_id
    data.extend_from_slice(&33i32.to_be_bytes()); // text_id
    data.extend_from_slice(&44i32.to_be_bytes()); // color
    data.extend_from_slice(&12.0f32.to_be_bytes()); // font_size
    data.extend_from_slice(&1i32.to_be_bytes()); // font_style (Italic)
    data.extend_from_slice(&700.0f32.to_be_bytes()); // font_weight
    data.extend_from_slice(&5i32.to_be_bytes()); // font_family_id
    data.extend_from_slice(&3i32.to_be_bytes()); // text_align (Center)
    data.extend_from_slice(&3i32.to_be_bytes()); // overflow (Ellipsis)
    data.extend_from_slice(&2i32.to_be_bytes()); // max_lines

    let (op, bytes_read) = LayoutText::read(&data).expect("Failed to read LayoutText");
    assert_eq!(bytes_read, 44);
    assert_eq!(op.component_id, 13);
    assert_eq!(op.text_id, 33);
    assert_eq!(op.font_style, FontStyle::Italic);
    assert_eq!(op.text_align, TextAlign::Center);
    assert_eq!(op.overflow, TextOverflow::Ellipsis);
}

#[test]
fn test_layout_image_read() {
    let mut data = Vec::new();
    data.extend_from_slice(&14i32.to_be_bytes()); // component_id
    data.extend_from_slice(&24i32.to_be_bytes()); // animation_id
    data.extend_from_slice(&34i32.to_be_bytes()); // bitmap_id
    data.extend_from_slice(&0i32.to_be_bytes()); // scale_type
    data.extend_from_slice(&0.5f32.to_be_bytes()); // alpha

    let (op, bytes_read) = LayoutImage::read(&data).expect("Failed to read LayoutImage");
    assert_eq!(bytes_read, 20);
    assert_eq!(op.component_id, 14);
    assert_eq!(op.bitmap_id, 34);
    assert_eq!(op.alpha, FloatExpression::Value(0.5));
}
