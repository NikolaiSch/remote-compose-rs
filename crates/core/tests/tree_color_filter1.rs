use remote_compose_core::document::Document;
use remote_compose_core::operations::header::{Header, Metadata};
use remote_compose_core::operations::paint::{PaintChange, PaintValues};
use remote_compose_core::operations::primitives::{
    DimensionType, FontStyle, LayoutAlignment, TextAlign, TextOverflow,
};
use remote_compose_core::operations::Operations;
use remote_compose_expressions::FloatExpression;
use std::collections::HashMap;

#[test]
fn test_manual_reproduction_color_filter1() {
    // TODO: fix and remove lenient mode
    let doc = Document::from_operations_lenient(get_color_filter1_operations())
        .expect("Document build failed");
    let expected = get_color_filter1_document();
    assert_eq!(doc, expected);
}

fn get_color_filter1_operations() -> Vec<Operations> {
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
        Operations::DataText {
            id: 42,
            text: "Green".to_string(),
        },
        Operations::LayoutRoot {
            component_id: -2,
            modifiers: vec![],
            children: vec![],
        },
        Operations::MatrixSave,
        Operations::MatrixSave,
        Operations::MatrixSave,
        Operations::MatrixSave,
        Operations::LayoutBox {
            component_id: -3,
            animation_id: -1,
            horizontal_alignment: LayoutAlignment::Center,
            vertical_alignment: LayoutAlignment::Center,
            modifiers: vec![],
            children: vec![],
        },
        Operations::ModifierWidth {
            type_: DimensionType::Fill,
            value: FloatExpression::Value(1.0),
        },
        Operations::ModifierHeight {
            type_: DimensionType::Fill,
            value: FloatExpression::Value(1.0),
        },
        Operations::ModifierDrawContent {
            modifiers: vec![],
            children: vec![],
        },
        Operations::LayoutContent { component_id: -4 },
        Operations::MatrixSave,
        Operations::CanvasOperations {
            modifiers: vec![],
            children: vec![],
        },
        Operations::ComponentValue {
            type_: 0,
            component_id: -4,
            value_id: 42,
        },
        Operations::ComponentValue {
            type_: 1,
            component_id: -4,
            value_id: 43,
        },
        Operations::AnimatedFloat {
            id: 44,
            values: FloatExpression::Divide(
                Box::new(FloatExpression::Variable(43)),
                Box::new(FloatExpression::Value(2.0)),
            ),
            animation: None,
        },
        Operations::AnimatedFloat {
            id: 46,
            values: FloatExpression::Divide(
                Box::new(FloatExpression::Variable(44)),
                Box::new(FloatExpression::Value(2.0)),
            ),
            animation: None,
        },
        Operations::AnimatedFloat {
            id: 47,
            values: FloatExpression::Divide(
                Box::new(FloatExpression::Min(
                    Box::new(FloatExpression::Variable(43)),
                    Box::new(FloatExpression::Variable(44)),
                )),
                Box::new(FloatExpression::Value(2.0)),
            ),
            animation: None,
        },
        Operations::PaintValues(PaintValues {
            changes: vec![
                PaintChange::Color(-16777216),
                PaintChange::StrokeWidth(FloatExpression::Value(0.0)),
                PaintChange::TextSize(FloatExpression::Value(12.0)),
                PaintChange::StrokeCap(0),
                PaintChange::StrokeJoin(0),
                PaintChange::Style(0),
                PaintChange::Typeface {
                    weight: 0,
                    italic: false,
                    font_data: false,
                    font_type: 0,
                },
                PaintChange::BlendMode(3),
                PaintChange::RadialGradient {
                    colors: vec![-8356096, -16777216],
                    stops: vec![],
                    center_x: FloatExpression::Variable(45),
                    center_y: FloatExpression::Variable(46),
                    radius: FloatExpression::Variable(47),
                    tile_mode: 0,
                },
            ],
        }),
        Operations::DrawRect {
            left: FloatExpression::Value(0.0),
            top: FloatExpression::Value(0.0),
            right: FloatExpression::Variable(43),
            bottom: FloatExpression::Variable(44),
        },
        Operations::DrawContent,
        Operations::ContainerEnd,
        Operations::MatrixSave,
        Operations::LayoutText {
            component_id: -5,
            animation_id: -1,
            text_id: 42,
            color: -16711936,
            font_size: FloatExpression::Value(82.5),
            font_style: FontStyle::Normal,
            font_weight: FloatExpression::Value(400.0),
            font_family_id: -1,
            text_align: TextAlign::Left,
            overflow: TextOverflow::Clip,
            max_lines: 2147483647,
            modifiers: vec![],
            children: vec![],
        },
        Operations::LayoutContent { component_id: -6 },
        Operations::ContainerEnd,
        Operations::ContainerEnd,
        Operations::MatrixRestore,
        Operations::MatrixRestore,
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

fn get_color_filter1_document() -> Document {
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
        root: vec![
            Operations::DataText {
                id: 42,
                text: "Green".to_string(),
            },
            Operations::LayoutRoot {
                component_id: -2,
                modifiers: vec![],
                children: vec![
                    Operations::MatrixSave,
                    Operations::MatrixSave,
                    Operations::MatrixSave,
                    Operations::MatrixSave,
                    Operations::LayoutBox {
                        component_id: -3,
                        animation_id: -1,
                        horizontal_alignment: LayoutAlignment::Center,
                        vertical_alignment: LayoutAlignment::Center,
                        modifiers: vec![
                            Operations::ModifierWidth {
                                type_: DimensionType::Fill,
                                value: FloatExpression::Value(1.0),
                            },
                            Operations::ModifierHeight {
                                type_: DimensionType::Fill,
                                value: FloatExpression::Value(1.0),
                            },
                            Operations::ModifierDrawContent {
                                modifiers: vec![],
                                children: vec![
                                    Operations::LayoutContent { component_id: -4 },
                                    Operations::MatrixSave,
                                    Operations::CanvasOperations {
                                        modifiers: vec![],
                                        children: vec![
                                            Operations::ComponentValue {
                                                type_: 0,
                                                component_id: -4,
                                                value_id: 42,
                                            },
                                            Operations::ComponentValue {
                                                type_: 1,
                                                component_id: -4,
                                                value_id: 43,
                                            },
                                            Operations::AnimatedFloat {
                                                id: 44,
                                                values: FloatExpression::Divide(
                                                    Box::new(FloatExpression::Variable(43)),
                                                    Box::new(FloatExpression::Value(2.0)),
                                                ),
                                                animation: None,
                                            },
                                            Operations::AnimatedFloat {
                                                id: 46,
                                                values: FloatExpression::Divide(
                                                    Box::new(FloatExpression::Variable(44)),
                                                    Box::new(FloatExpression::Value(2.0)),
                                                ),
                                                animation: None,
                                            },
                                            Operations::AnimatedFloat {
                                                id: 47,
                                                values: FloatExpression::Divide(
                                                    Box::new(FloatExpression::Min(
                                                        Box::new(FloatExpression::Variable(43)),
                                                        Box::new(FloatExpression::Variable(44)),
                                                    )),
                                                    Box::new(FloatExpression::Value(2.0)),
                                                ),
                                                animation: None,
                                            },
                                            Operations::PaintValues(PaintValues {
                                                changes: vec![
                                                    PaintChange::Color(-16777216),
                                                    PaintChange::StrokeWidth(
                                                        FloatExpression::Value(0.0),
                                                    ),
                                                    PaintChange::TextSize(FloatExpression::Value(
                                                        12.0,
                                                    )),
                                                    PaintChange::StrokeCap(0),
                                                    PaintChange::StrokeJoin(0),
                                                    PaintChange::Style(0),
                                                    PaintChange::Typeface {
                                                        weight: 0,
                                                        italic: false,
                                                        font_data: false,
                                                        font_type: 0,
                                                    },
                                                    PaintChange::BlendMode(3),
                                                    PaintChange::RadialGradient {
                                                        colors: vec![-8356096, -16777216],
                                                        stops: vec![],
                                                        center_x: FloatExpression::Variable(45),
                                                        center_y: FloatExpression::Variable(46),
                                                        radius: FloatExpression::Variable(47),
                                                        tile_mode: 0,
                                                    },
                                                ],
                                            }),
                                            Operations::DrawRect {
                                                left: FloatExpression::Value(0.0),
                                                top: FloatExpression::Value(0.0),
                                                right: FloatExpression::Variable(43),
                                                bottom: FloatExpression::Variable(44),
                                            },
                                            Operations::DrawContent,
                                        ],
                                    },
                                    Operations::MatrixSave,
                                    Operations::LayoutText {
                                        component_id: -5,
                                        animation_id: -1,
                                        text_id: 42,
                                        color: -16711936,
                                        font_size: FloatExpression::Value(82.5),
                                        font_style: FontStyle::Normal,
                                        font_weight: FloatExpression::Value(400.0),
                                        font_family_id: -1,
                                        text_align: TextAlign::Left,
                                        overflow: TextOverflow::Clip,
                                        max_lines: 2147483647,
                                        modifiers: vec![],
                                        children: vec![],
                                    },
                                    Operations::LayoutContent { component_id: -6 },
                                ],
                            },
                        ],
                        children: vec![],
                    },
                    Operations::MatrixRestore,
                    Operations::MatrixRestore,
                    Operations::MatrixRestore,
                    Operations::MatrixRestore,
                    Operations::MatrixRestore,
                    Operations::MatrixSave,
                    Operations::MatrixRestore,
                    Operations::MatrixRestore,
                ],
            },
        ],
    }
}
