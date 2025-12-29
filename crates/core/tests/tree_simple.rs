use remote_compose_core::document::Document;
use remote_compose_core::operations::header::{Header, Metadata};
use remote_compose_core::operations::{DimensionType, LayoutAlignment, Operations, ShapeType};
use remote_compose_expressions::FloatExpression;
use std::collections::HashMap;

fn verify_simple_structure(doc: &Document) {
    // Basic verification: Check if we have root nodes
    assert!(!doc.root.is_empty(), "Document should have root nodes");
    println!(
        "Document built successfully with root nodes: {}",
        doc.root.len()
    );
}

fn get_simple_operations() -> Vec<Operations> {
    vec![
        Operations::Header {
            major: 1,
            minor: 1,
            patch: 0,
            metadata: {
                let mut m = HashMap::new();
                m.insert(Header::WIDTH_ID, Metadata::Int(715));
                m.insert(Header::HEIGHT_ID, Metadata::Int(825));
                m.insert(Header::CAPABILITIES_ID, Metadata::Int(512));
                m.insert(Header::ROOT_ID_ID, Metadata::String("".to_string()));
                m
            },
        },
        Operations::LayoutRoot {
            component_id: -2,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::MatrixSave,
        Operations::MatrixSave,
        Operations::MatrixSave,
        Operations::MatrixSave,
        Operations::LayoutColumn {
            component_id: -3,
            animation_id: -1,
            horizontal_alignment: LayoutAlignment::Center,
            vertical_alignment: LayoutAlignment::Center,
            spaced_by: FloatExpression::Value(0.0),
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::ModifierWidth {
            type_: DimensionType::Fill,
            value: FloatExpression::Value(1.0),
        },
        Operations::ModifierHeight {
            type_: DimensionType::Fill,
            value: FloatExpression::Value(1.0),
        },
        Operations::ModifierBackground {
            flags: 0,
            color_id: 0,
            r: FloatExpression::Value(1.0),
            g: FloatExpression::Value(1.0),
            b: FloatExpression::Value(0.0),
            a: FloatExpression::Value(1.0),
            shape_type: ShapeType::Rectangle,
        },
        Operations::LayoutContent { component_id: -4 },
        Operations::MatrixTranslate {
            tx: FloatExpression::Value(358.0),
            ty: FloatExpression::Value(413.0),
        },
        Operations::MatrixSave,
        Operations::LayoutCanvas {
            component_id: -5,
            animation_id: -1,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::ModifierWidth {
            type_: DimensionType::Fill,
            value: FloatExpression::Value(1.0),
        },
        Operations::ModifierHeight {
            type_: DimensionType::ExactDp,
            value: FloatExpression::Value(100.0),
        },
        Operations::ModifierBackground {
            flags: 0,
            color_id: 0,
            r: FloatExpression::Value(1.0),
            g: FloatExpression::Value(1.0),
            b: FloatExpression::Value(1.0),
            a: FloatExpression::Value(1.0),
            shape_type: ShapeType::Rectangle,
        },
        Operations::ModifierPadding {
            left: FloatExpression::Value(22.0),
            top: FloatExpression::Value(22.0),
            right: FloatExpression::Value(22.0),
            bottom: FloatExpression::Value(22.0),
        },
        Operations::ModifierBackground {
            flags: 0,
            color_id: 0,
            r: FloatExpression::Value(0.8),
            g: FloatExpression::Value(0.8),
            b: FloatExpression::Value(0.8),
            a: FloatExpression::Value(1.0),
            shape_type: ShapeType::Rectangle,
        },
        Operations::LayoutContent { component_id: -6 },
        Operations::LayoutCanvasContent {
            component_id: -7,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::MatrixSave,
        Operations::MatrixRestore,
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
        Operations::MatrixTranslate {
            tx: FloatExpression::Value(-358.0),
            ty: FloatExpression::Value(-413.0),
        },
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
        Operations::MatrixRestore,
        Operations::MatrixRestore,
        Operations::MatrixSave,
        Operations::MatrixRestore,
        Operations::MatrixRestore,
        Operations::ContainerEnd,
    ]
}

fn get_simple_document() -> Document {
    Document {
        header: Header {
            major: 1,
            minor: 1,
            patch: 0,
            metadata: {
                let mut m = HashMap::new();
                m.insert(Header::WIDTH_ID, Metadata::Int(715));
                m.insert(Header::HEIGHT_ID, Metadata::Int(825));
                m.insert(Header::CAPABILITIES_ID, Metadata::Int(512));
                m.insert(Header::ROOT_ID_ID, Metadata::String("".to_string()));
                m
            },
        },
        root: vec![Operations::LayoutRoot {
            component_id: -2,
            modifiers: vec![
                Operations::MatrixSave,
                Operations::MatrixSave,
                Operations::MatrixSave,
                Operations::MatrixSave,
                Operations::LayoutColumn {
                    component_id: -3,
                    animation_id: -1,
                    horizontal_alignment: LayoutAlignment::Center,
                    vertical_alignment: LayoutAlignment::Center,
                    spaced_by: FloatExpression::Value(0.0),
                    modifiers: vec![
                        Operations::ModifierWidth {
                            type_: DimensionType::Fill,
                            value: FloatExpression::Value(1.0),
                        },
                        Operations::ModifierHeight {
                            type_: DimensionType::Fill,
                            value: FloatExpression::Value(1.0),
                        },
                        Operations::ModifierBackground {
                            flags: 0,
                            color_id: 0,
                            r: FloatExpression::Value(1.0),
                            g: FloatExpression::Value(1.0),
                            b: FloatExpression::Value(0.0),
                            a: FloatExpression::Value(1.0),
                            shape_type: ShapeType::Rectangle,
                        },
                    ],
                    children: vec![
                        Operations::MatrixTranslate {
                            tx: FloatExpression::Value(358.0),
                            ty: FloatExpression::Value(413.0),
                        },
                        Operations::MatrixSave,
                        Operations::LayoutCanvas {
                            component_id: -5,
                            animation_id: -1,
                            modifiers: vec![
                                Operations::ModifierWidth {
                                    type_: DimensionType::Fill,
                                    value: FloatExpression::Value(1.0),
                                },
                                Operations::ModifierHeight {
                                    type_: DimensionType::ExactDp,
                                    value: FloatExpression::Value(100.0),
                                },
                                Operations::ModifierBackground {
                                    flags: 0,
                                    color_id: 0,
                                    r: FloatExpression::Value(1.0),
                                    g: FloatExpression::Value(1.0),
                                    b: FloatExpression::Value(1.0),
                                    a: FloatExpression::Value(1.0),
                                    shape_type: ShapeType::Rectangle,
                                },
                                Operations::ModifierPadding {
                                    left: FloatExpression::Value(22.0),
                                    top: FloatExpression::Value(22.0),
                                    right: FloatExpression::Value(22.0),
                                    bottom: FloatExpression::Value(22.0),
                                },
                                Operations::ModifierBackground {
                                    flags: 0,
                                    color_id: 0,
                                    r: FloatExpression::Value(0.8),
                                    g: FloatExpression::Value(0.8),
                                    b: FloatExpression::Value(0.8),
                                    a: FloatExpression::Value(1.0),
                                    shape_type: ShapeType::Rectangle,
                                },
                            ],
                            children: vec![Operations::LayoutCanvasContent {
                                component_id: -7,
                                modifiers: vec![Operations::MatrixSave, Operations::MatrixRestore],
                                children: vec![],
                            }],
                        },
                        Operations::MatrixRestore,
                        Operations::MatrixTranslate {
                            tx: FloatExpression::Value(-358.0),
                            ty: FloatExpression::Value(-413.0),
                        },
                    ],
                },
                Operations::MatrixRestore,
                Operations::MatrixRestore,
                Operations::MatrixRestore,
                Operations::MatrixSave,
                Operations::MatrixRestore,
                Operations::MatrixRestore,
            ],
            children: vec![],
        }],
    }
}

#[test]
fn test_manual_reproduction_simple() {
    let doc = Document::from_operations(get_simple_operations()).expect("Document build failed");
    let expected = get_simple_document();
    verify_simple_structure(&doc);
    assert_eq!(doc, expected);
}
