use crate::operations::{
    primitives::*, read_float_expression_from_single_float, ActionList, Operations,
};
use remote_compose_expressions::FloatExpression;

/// [ModifierBackground](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/BackgroundModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierBackground {
    pub flags: i32,
    pub color_id: i32,
    pub r: FloatExpression,
    pub g: FloatExpression,
    pub b: FloatExpression,
    pub a: FloatExpression,
    pub shape_type: ShapeType,
}

impl ModifierBackground {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let flags = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read flags".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert flags".to_string())?,
        );
        offset += 4;
        let color_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read color_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert color_id".to_string())?,
        );
        offset += 4;
        // Skip reserved1 and reserved2
        offset += 8;
        let (r, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing r".to_string())?,
        )?;
        offset += width;
        let (g, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing g".to_string())?,
        )?;
        offset += width;
        let (b, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing b".to_string())?,
        )?;
        offset += width;
        let (a, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing a".to_string())?,
        )?;
        offset += width;
        let shape_type = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read shape_type".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert shape_type".to_string())?,
        );
        offset += 4;
        Ok((
            ModifierBackground {
                flags,
                color_id,
                r,
                g,
                b,
                a,
                shape_type: ShapeType::from(shape_type),
            },
            offset,
        ))
    }
}

/// [ModifierPadding](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/PaddingModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierPadding {
    pub left: FloatExpression,
    pub top: FloatExpression,
    pub right: FloatExpression,
    pub bottom: FloatExpression,
}

impl ModifierPadding {
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
            ModifierPadding {
                left,
                top,
                right,
                bottom,
            },
            offset,
        ))
    }
}

/// [ModifierHeight](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/HeightModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierHeight {
    pub type_: DimensionType,
    pub value: FloatExpression,
}

impl ModifierHeight {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let type_ = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read type_".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert type_".to_string())?,
        );
        offset += 4;
        let (value, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing value".to_string())?,
        )?;
        offset += width;
        Ok((
            ModifierHeight {
                type_: DimensionType::from(type_),
                value,
            },
            offset,
        ))
    }
}

/// [ModifierWidth](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/WidthModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierWidth {
    pub type_: DimensionType,
    pub value: FloatExpression,
}

impl ModifierWidth {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let type_ = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read type_".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert type_".to_string())?,
        );
        offset += 4;
        let (value, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing value".to_string())?,
        )?;
        offset += width;
        Ok((
            ModifierWidth {
                type_: DimensionType::from(type_),
                value,
            },
            offset,
        ))
    }
}

/// [ModifierTouchDown](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/TouchDownModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierTouchDown {
    pub actions: Vec<Operations>,
}

impl ActionList for ModifierTouchDown {
    fn actions(&self) -> &[Operations] {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.actions
    }
}

impl ModifierTouchDown {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            ModifierTouchDown {
                actions: Vec::new(),
            },
            0,
        ))
    }
}

/// [ModifierTouchUp](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/TouchUpModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierTouchUp {
    pub actions: Vec<Operations>,
}

impl ActionList for ModifierTouchUp {
    fn actions(&self) -> &[Operations] {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.actions
    }
}

impl ModifierTouchUp {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            ModifierTouchUp {
                actions: Vec::new(),
            },
            0,
        ))
    }
}

/// [ModifierTouchCancel](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/TouchCancelModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierTouchCancel {
    pub actions: Vec<Operations>,
}

impl ActionList for ModifierTouchCancel {
    fn actions(&self) -> &[Operations] {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.actions
    }
}

impl ModifierTouchCancel {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            ModifierTouchCancel {
                actions: Vec::new(),
            },
            0,
        ))
    }
}

/// [ModifierClick](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/ClickModifierOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierClick {
    pub actions: Vec<Operations>,
}

impl ActionList for ModifierClick {
    fn actions(&self) -> &[Operations] {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.actions
    }
}

impl ModifierClick {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            ModifierClick {
                actions: Vec::new(),
            },
            0,
        ))
    }
}
