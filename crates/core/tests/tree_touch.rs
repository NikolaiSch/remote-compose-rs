use remote_compose_core::document::Document;
use remote_compose_core::operations::header::{Header, Metadata};
use remote_compose_core::operations::{DimensionType, LayoutAlignment, Operations, ShapeType};
use remote_compose_expressions::FloatExpression;
use std::collections::HashMap;

fn get_touch_operations() -> Vec<Operations> {
    vec![
        Operations::Header {
            major: 1,
            minor: 1,
            patch: 0,
            metadata: {
                let mut m = HashMap::new();
                m.insert(Header::ROOT_ID_ID, Metadata::String("".to_string()));
                m.insert(Header::HEIGHT_ID, Metadata::Int(825));
                m.insert(Header::CAPABILITIES_ID, Metadata::Int(512));
                m.insert(Header::WIDTH_ID, Metadata::Int(715));
                m
            },
        },
        Operations::DataInt { id: 42, value: 1 },
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
        Operations::LayoutState {
            component_id: -5,
            animation_id: -1,
            index_id: 42,
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
        Operations::ModifierTouchDown {
            actions: vec![Operations::ValueIntegerChangeAction {
                value_id: 42,
                value: 0,
            }],
        },
        Operations::ContainerEnd,
        Operations::ModifierTouchUp {
            actions: vec![Operations::ValueIntegerChangeAction {
                value_id: 42,
                value: 1,
            }],
        },
        Operations::ContainerEnd,
        Operations::ModifierTouchCancel {
            actions: vec![Operations::ValueIntegerChangeAction {
                value_id: 42,
                value: 1,
            }],
        },
        Operations::ContainerEnd,
        Operations::LayoutContent { component_id: -6 },
        Operations::MatrixSave,
        Operations::LayoutBox {
            component_id: -7,
            animation_id: -1,
            horizontal_alignment: LayoutAlignment::Start,
            vertical_alignment: LayoutAlignment::Top,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::LayoutContent { component_id: -8 },
        Operations::MatrixSave,
        Operations::LayoutBox {
            component_id: -9,
            animation_id: -1,
            horizontal_alignment: LayoutAlignment::Start,
            vertical_alignment: LayoutAlignment::Top,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::ModifierWidth {
            type_: DimensionType::ExactDp,
            value: FloatExpression::Value(60.0),
        },
        Operations::ModifierHeight {
            type_: DimensionType::ExactDp,
            value: FloatExpression::Value(60.0),
        },
        Operations::ModifierBackground {
            flags: 0,
            color_id: 0,
            r: FloatExpression::Value(1.0),
            g: FloatExpression::Value(0.0),
            b: FloatExpression::Value(0.0),
            a: FloatExpression::Value(1.0),
            shape_type: ShapeType::Rectangle,
        },
        Operations::LayoutContent { component_id: -10 },
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
        Operations::MatrixSave,
        Operations::LayoutBox {
            component_id: -11,
            animation_id: -1,
            horizontal_alignment: LayoutAlignment::Start,
            vertical_alignment: LayoutAlignment::Top,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::LayoutContent { component_id: -12 },
        Operations::MatrixSave,
        Operations::LayoutBox {
            component_id: -13,
            animation_id: -1,
            horizontal_alignment: LayoutAlignment::Start,
            vertical_alignment: LayoutAlignment::Top,
            modifiers: Vec::new(),
            children: Vec::new(),
        },
        Operations::ModifierWidth {
            type_: DimensionType::ExactDp,
            value: FloatExpression::Value(80.0),
        },
        Operations::ModifierHeight {
            type_: DimensionType::ExactDp,
            value: FloatExpression::Value(80.0),
        },
        Operations::ModifierBackground {
            flags: 0,
            color_id: 0,
            r: FloatExpression::Value(0.0),
            g: FloatExpression::Value(1.0),
            b: FloatExpression::Value(0.0),
            a: FloatExpression::Value(1.0),
            shape_type: ShapeType::Rectangle,
        },
        Operations::LayoutContent { component_id: -14 },
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
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

fn get_touch_document() -> Document {
    Document {
        header: Header {
            major: 1,
            minor: 1,
            patch: 0,
            metadata: {
                let mut m = HashMap::new();
                m.insert(Header::ROOT_ID_ID, Metadata::String("".to_string()));
                m.insert(Header::HEIGHT_ID, Metadata::Int(825));
                m.insert(Header::CAPABILITIES_ID, Metadata::Int(512));
                m.insert(Header::WIDTH_ID, Metadata::Int(715));
                m
            },
        },
        root: vec![
            Operations::DataInt { id: 42, value: 1 },
            Operations::LayoutRoot {
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
                            Operations::LayoutState {
                                component_id: -5,
                                animation_id: -1,
                                index_id: 42,
                                modifiers: vec![
                                    Operations::ModifierWidth {
                                        type_: DimensionType::Fill,
                                        value: FloatExpression::Value(1.0),
                                    },
                                    Operations::ModifierHeight {
                                        type_: DimensionType::Fill,
                                        value: FloatExpression::Value(1.0),
                                    },
                                    Operations::ModifierTouchDown {
                                        actions: vec![Operations::ValueIntegerChangeAction {
                                            value_id: 42,
                                            value: 0,
                                        }],
                                    },
                                    Operations::ModifierTouchUp {
                                        actions: vec![Operations::ValueIntegerChangeAction {
                                            value_id: 42,
                                            value: 1,
                                        }],
                                    },
                                    Operations::ModifierTouchCancel {
                                        actions: vec![Operations::ValueIntegerChangeAction {
                                            value_id: 42,
                                            value: 1,
                                        }],
                                    },
                                ],
                                children: vec![
                                    Operations::MatrixSave,
                                    Operations::LayoutBox {
                                        component_id: -7,
                                        animation_id: -1,
                                        horizontal_alignment: LayoutAlignment::Start,
                                        vertical_alignment: LayoutAlignment::Top,
                                        modifiers: vec![],
                                        children: vec![
                                            Operations::MatrixSave,
                                            Operations::LayoutBox {
                                                component_id: -9,
                                                animation_id: -1,
                                                horizontal_alignment: LayoutAlignment::Start,
                                                vertical_alignment: LayoutAlignment::Top,
                                                modifiers: vec![
                                                    Operations::ModifierWidth {
                                                        type_: DimensionType::ExactDp,
                                                        value: FloatExpression::Value(60.0),
                                                    },
                                                    Operations::ModifierHeight {
                                                        type_: DimensionType::ExactDp,
                                                        value: FloatExpression::Value(60.0),
                                                    },
                                                    Operations::ModifierBackground {
                                                        flags: 0,
                                                        color_id: 0,
                                                        r: FloatExpression::Value(1.0),
                                                        g: FloatExpression::Value(0.0),
                                                        b: FloatExpression::Value(0.0),
                                                        a: FloatExpression::Value(1.0),
                                                        shape_type: ShapeType::Rectangle,
                                                    },
                                                ],
                                                children: vec![],
                                            },
                                            Operations::MatrixRestore,
                                        ],
                                    },
                                    Operations::MatrixRestore,
                                    Operations::MatrixSave,
                                    Operations::LayoutBox {
                                        component_id: -11,
                                        animation_id: -1,
                                        horizontal_alignment: LayoutAlignment::Start,
                                        vertical_alignment: LayoutAlignment::Top,
                                        modifiers: vec![],
                                        children: vec![
                                            Operations::MatrixSave,
                                            Operations::LayoutBox {
                                                component_id: -13,
                                                animation_id: -1,
                                                horizontal_alignment: LayoutAlignment::Start,
                                                vertical_alignment: LayoutAlignment::Top,
                                                modifiers: vec![
                                                    Operations::ModifierWidth {
                                                        type_: DimensionType::ExactDp,
                                                        value: FloatExpression::Value(80.0),
                                                    },
                                                    Operations::ModifierHeight {
                                                        type_: DimensionType::ExactDp,
                                                        value: FloatExpression::Value(80.0),
                                                    },
                                                    Operations::ModifierBackground {
                                                        flags: 0,
                                                        color_id: 0,
                                                        r: FloatExpression::Value(0.0),
                                                        g: FloatExpression::Value(1.0),
                                                        b: FloatExpression::Value(0.0),
                                                        a: FloatExpression::Value(1.0),
                                                        shape_type: ShapeType::Rectangle,
                                                    },
                                                ],
                                                children: vec![],
                                            },
                                            Operations::MatrixRestore,
                                        ],
                                    },
                                    Operations::MatrixRestore,
                                ],
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
            },
        ],
    }
}

#[test]
fn test_manual_reproduction_touch() {
    let doc = Document::from_operations(get_touch_operations()).expect("Document build failed");
    let expected = get_touch_document();
    assert_eq!(doc, expected);
}
