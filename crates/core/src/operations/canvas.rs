use crate::operations::read_float_expression_from_single_float;
use remote_compose_expressions::FloatExpression;

/// [DrawLine](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/DrawLine.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DrawLine {
    pub x1: FloatExpression,
    pub y1: FloatExpression,
    pub x2: FloatExpression,
    pub y2: FloatExpression,
}

impl DrawLine {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let (x1, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing x1".to_string())?,
        )?;
        offset += width;
        let (y1, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing y1".to_string())?,
        )?;
        offset += width;
        let (x2, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing x2".to_string())?,
        )?;
        offset += width;
        let (y2, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing y2".to_string())?,
        )?;
        offset += width;
        Ok((DrawLine { x1, y1, x2, y2 }, offset))
    }
}

/// [MatrixTranslate](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/MatrixTranslate.java)
#[derive(Debug, Clone, PartialEq)]
pub struct MatrixTranslate {
    pub tx: FloatExpression,
    pub ty: FloatExpression,
}

impl MatrixTranslate {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let (tx, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing tx".to_string())?,
        )?;
        offset += width;
        let (ty, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing ty".to_string())?,
        )?;
        offset += width;
        Ok((MatrixTranslate { tx, ty }, offset))
    }
}

/// [MatrixScale](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/MatrixScale.java)
#[derive(Debug, Clone, PartialEq)]
pub struct MatrixScale {
    pub sx: FloatExpression,
    pub sy: FloatExpression,
}

impl MatrixScale {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let (sx, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing sx".to_string())?,
        )?;
        offset += width;
        let (sy, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing sy".to_string())?,
        )?;
        offset += width;
        Ok((MatrixScale { sx, sy }, offset))
    }
}

/// [MatrixRotate](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/MatrixRotate.java)
#[derive(Debug, Clone, PartialEq)]
pub struct MatrixRotate {
    pub angle: FloatExpression,
    pub cx: FloatExpression,
    pub cy: FloatExpression,
}

impl MatrixRotate {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let (angle, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing angle".to_string())?,
        )?;
        offset += width;
        let (cx, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing cx".to_string())?,
        )?;
        offset += width;
        let (cy, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing cy".to_string())?,
        )?;
        offset += width;
        Ok((MatrixRotate { angle, cx, cy }, offset))
    }
}

/// [DrawRect](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/DrawRect.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DrawRect {
    pub left: FloatExpression,
    pub top: FloatExpression,
    pub right: FloatExpression,
    pub bottom: FloatExpression,
}

impl DrawRect {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let (left, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing left".to_string())?,
        )?;
        offset += width;
        let (top, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing top".to_string())?,
        )?;
        offset += width;
        let (right, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing right".to_string())?,
        )?;
        offset += width;
        let (bottom, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing bottom".to_string())?,
        )?;
        offset += width;
        Ok((
            DrawRect {
                left,
                top,
                right,
                bottom,
            },
            offset,
        ))
    }
}

impl crate::operations::Operation for DrawRect {
    fn opcode(&self) -> crate::operations::OpCode {
        crate::operations::OpCode::DrawRect
    }
}

/// [DrawContent](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/DrawContent.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DrawContent {}

impl DrawContent {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((DrawContent {}, 0))
    }
}

impl crate::operations::Operation for DrawContent {
    fn opcode(&self) -> crate::operations::OpCode {
        crate::operations::OpCode::DrawContent
    }
}

/// [CanvasOperations]
#[derive(Debug, Clone, PartialEq)]
pub struct CanvasOperations {}

impl CanvasOperations {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((CanvasOperations {}, 0))
    }
}

/// [DrawPath](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/DrawPath.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DrawPath {
    pub path_id: i32,
}

impl DrawPath {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 4 {
            return Err("Data too short for DrawPath".to_string());
        }
        let path_id = i32::from_be_bytes(data[0..4].try_into().unwrap());
        Ok((DrawPath { path_id }, 4))
    }
}
