#[derive(Debug, Clone, PartialEq)]
pub enum TextAlign {
    Left = 1,
    Right = 2,
    Center = 3,
    Justify = 4,
    Start = 5,
    End = 6,
}

impl From<i32> for TextAlign {
    fn from(v: i32) -> Self {
        match v {
            1 => TextAlign::Left,
            2 => TextAlign::Right,
            3 => TextAlign::Center,
            4 => TextAlign::Justify,
            5 => TextAlign::Start,
            6 => TextAlign::End,
            _ => TextAlign::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextOverflow {
    Clip = 1,
    Visible = 2,
    Ellipsis = 3,
    StartEllipsis = 4,
    MiddleEllipsis = 5,
}

impl From<i32> for TextOverflow {
    fn from(v: i32) -> Self {
        match v {
            1 => TextOverflow::Clip,
            2 => TextOverflow::Visible,
            3 => TextOverflow::Ellipsis,
            4 => TextOverflow::StartEllipsis,
            5 => TextOverflow::MiddleEllipsis,
            _ => TextOverflow::Clip,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontStyle {
    Normal = 0,
    Italic = 1,
}

impl From<i32> for FontStyle {
    fn from(v: i32) -> Self {
        match v {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutAlignment {
    Start = 1,
    Center = 2,
    End = 3,
    Top = 4,
    Bottom = 5,
    SpaceBetween = 6,
    SpaceEvenly = 7,
    SpaceAround = 8,
}

impl From<i32> for LayoutAlignment {
    fn from(v: i32) -> Self {
        match v {
            1 => LayoutAlignment::Start,
            2 => LayoutAlignment::Center,
            3 => LayoutAlignment::End,
            4 => LayoutAlignment::Top,
            5 => LayoutAlignment::Bottom,
            6 => LayoutAlignment::SpaceBetween,
            7 => LayoutAlignment::SpaceEvenly,
            8 => LayoutAlignment::SpaceAround,
            _ => LayoutAlignment::Start,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DimensionType {
    Exact = 0,
    Fill = 1,
    Wrap = 2,
    Weight = 3,
    IntrinsicMin = 4,
    IntrinsicMax = 5,
    ExactDp = 6,
}

impl From<i32> for DimensionType {
    fn from(v: i32) -> Self {
        match v {
            0 => DimensionType::Exact,
            1 => DimensionType::Fill,
            2 => DimensionType::Wrap,
            3 => DimensionType::Weight,
            4 => DimensionType::IntrinsicMin,
            5 => DimensionType::IntrinsicMax,
            6 => DimensionType::ExactDp,
            _ => DimensionType::Exact,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeType {
    Rectangle = 0,
    Circle = 1,
}

impl From<i32> for ShapeType {
    fn from(v: i32) -> Self {
        match v {
            0 => ShapeType::Rectangle,
            1 => ShapeType::Circle,
            _ => ShapeType::Rectangle,
        }
    }
}
