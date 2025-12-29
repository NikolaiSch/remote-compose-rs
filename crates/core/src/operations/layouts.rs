use crate::operations::{
    primitives::*, read_float_expression_from_single_float, Container, Operations,
};
use remote_compose_expressions::FloatExpression;

/// [ComponentStart](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/ComponentStart.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentStart {
    pub component_type: i32,
    pub component_id: i32,
    pub width: FloatExpression,
    pub height: FloatExpression,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for ComponentStart {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl ComponentStart {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        if data.len() < 8 {
            // Minimum size for component_type and component_id
            return Err("Data too short for ComponentStart".to_string());
        }
        let component_type = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_type".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_type".to_string())?,
        );
        offset += 4;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let (width, n_width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing width".to_string())?,
        )?;
        offset += n_width;
        let (height, n_height) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing height".to_string())?,
        )?;
        offset += n_height;

        Ok((
            ComponentStart {
                component_type,
                component_id,
                width,
                height,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [ContainerEnd](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/ContainerEnd.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerEnd;

impl ContainerEnd {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((ContainerEnd, 0))
    }
}

/// [RootContentBehavior](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/RootContentBehavior.java)
#[derive(Debug, Clone, PartialEq)]
pub struct RootContentBehavior {
    pub scroll: i32,
    pub alignment: i32,
    pub sizing: i32,
    pub mode: i32,
}

impl RootContentBehavior {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 16 {
            return Err("Data too short for RootContentBehavior".to_string());
        }
        let scroll = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let alignment = i32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let sizing = i32::from_be_bytes([data[8], data[9], data[10], data[11]]);
        let mode = i32::from_be_bytes([data[12], data[13], data[14], data[15]]);
        Ok((
            RootContentBehavior {
                scroll,
                alignment,
                sizing,
                mode,
            },
            16,
        ))
    }
}

/// [LayoutRoot](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutRoot {
    pub component_id: i32,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutRoot {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutRoot {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 4 {
            return Err("Data too short for LayoutRoot".to_string());
        }
        let component_id = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((
            LayoutRoot {
                component_id,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            4,
        ))
    }
}

/// [LayoutContent](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutContent {
    pub component_id: i32,
}

impl LayoutContent {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 4 {
            return Err("Data too short for LayoutContent".to_string());
        }
        let component_id = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((LayoutContent { component_id }, 4))
    }
}

/// [LayoutContent](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutCanvasContent {
    pub component_id: i32,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutCanvasContent {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutCanvasContent {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 4 {
            return Err("Data too short for LayoutCanvasContent".to_string());
        }
        let component_id = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((
            LayoutCanvasContent {
                component_id,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            4,
        ))
    }
}

/// [LayoutText](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/TextLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutText {
    pub component_id: i32,
    pub animation_id: i32,
    pub text_id: i32,
    pub color: i32,
    pub font_size: FloatExpression, // Changed from f32
    pub font_style: FontStyle,
    pub font_weight: FloatExpression, // Changed from f32
    pub font_family_id: i32,
    pub text_align: TextAlign,
    pub overflow: TextOverflow,
    pub max_lines: i32,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutText {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutText {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let animation_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read animation_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        offset += 4;
        let text_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read text_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert text_id".to_string())?,
        );
        offset += 4;
        let color = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read color".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert color".to_string())?,
        );
        offset += 4;
        let (font_size, n) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing font_size".to_string())?,
        )?;
        offset += n;
        let font_style = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read font_style".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert font_style".to_string())?,
        );
        offset += 4;
        let (font_weight, n) = read_float_expression_from_single_float(
            data.get(offset..)
                .ok_or("Missing font_weight".to_string())?,
        )?;
        offset += n;
        let font_family_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read font_family_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert font_family_id".to_string())?,
        );
        offset += 4;
        let text_align = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read text_align".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert text_align".to_string())?,
        );
        offset += 4;
        let overflow = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read overflow".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert overflow".to_string())?,
        );
        offset += 4;
        let max_lines = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read max_lines".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert max_lines".to_string())?,
        );
        offset += 4;
        Ok((
            LayoutText {
                component_id,
                animation_id,
                text_id,
                color,
                font_size,
                font_style: FontStyle::from(font_style),
                font_weight,
                font_family_id,
                text_align: TextAlign::from(text_align),
                overflow: TextOverflow::from(overflow),
                max_lines,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [LayoutImage](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/ImageLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutImage {
    pub component_id: i32,
    pub animation_id: i32,
    pub bitmap_id: i32,
    pub scale_type: i32,
    pub alpha: FloatExpression, // Changed from f32
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutImage {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutImage {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let animation_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read animation_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        offset += 4;
        let bitmap_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read bitmap_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert bitmap_id".to_string())?,
        );
        offset += 4;
        let scale_type = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read scale_type".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert scale_type".to_string())?,
        );
        offset += 4;
        let (alpha, n) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing alpha".to_string())?,
        )?;
        offset += n;
        Ok((
            LayoutImage {
                component_id,
                animation_id,
                bitmap_id,
                scale_type,
                alpha,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [LayoutColumn](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/ColumnLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutColumn {
    pub component_id: i32,
    pub animation_id: i32,
    pub horizontal_alignment: LayoutAlignment,
    pub vertical_alignment: LayoutAlignment,
    pub spaced_by: FloatExpression, // Changed from f32
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutColumn {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutColumn {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let animation_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read animation_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        offset += 4;
        let horizontal_alignment = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read horizontal_alignment".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert horizontal_alignment".to_string())?,
        );
        offset += 4;
        let vertical_alignment = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read vertical_alignment".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert vertical_alignment".to_string())?,
        );
        offset += 4;
        let (spaced_by, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing spaced_by".to_string())?,
        )?;
        offset += width;
        Ok((
            LayoutColumn {
                component_id,
                animation_id,
                horizontal_alignment: LayoutAlignment::from(horizontal_alignment),
                vertical_alignment: LayoutAlignment::from(vertical_alignment),
                spaced_by,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [LayoutRow](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/RowLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutRow {
    pub component_id: i32,
    pub animation_id: i32,
    pub horizontal_alignment: LayoutAlignment,
    pub vertical_alignment: LayoutAlignment,
    pub spaced_by: FloatExpression, // Changed from f32
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutRow {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutRow {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let animation_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read animation_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        offset += 4;
        let horizontal_alignment = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read horizontal_alignment".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert horizontal_alignment".to_string())?,
        );
        offset += 4;
        let vertical_alignment = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read vertical_alignment".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert vertical_alignment".to_string())?,
        );
        offset += 4;
        let (spaced_by, width) = read_float_expression_from_single_float(
            data.get(offset..).ok_or("Missing spaced_by".to_string())?,
        )?;
        offset += width;
        Ok((
            LayoutRow {
                component_id,
                animation_id,
                horizontal_alignment: LayoutAlignment::from(horizontal_alignment),
                vertical_alignment: LayoutAlignment::from(vertical_alignment),
                spaced_by,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [LayoutBox](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/BoxLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBox {
    pub component_id: i32,
    pub animation_id: i32,
    pub horizontal_alignment: LayoutAlignment,
    pub vertical_alignment: LayoutAlignment,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutBox {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutBox {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let animation_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read animation_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        offset += 4;
        let horizontal_alignment = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read horizontal_alignment".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert horizontal_alignment".to_string())?,
        );
        offset += 4;
        let vertical_alignment = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read vertical_alignment".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert vertical_alignment".to_string())?,
        );
        offset += 4;
        Ok((
            LayoutBox {
                component_id,
                animation_id,
                horizontal_alignment: LayoutAlignment::from(horizontal_alignment),
                vertical_alignment: LayoutAlignment::from(vertical_alignment),
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [LayoutCanvas](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/CanvasLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutCanvas {
    pub component_id: i32,
    pub animation_id: i32,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutCanvas {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutCanvas {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let component_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read component_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        offset += 4;
        let animation_id = i32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read animation_id".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        offset += 4;
        Ok((
            LayoutCanvas {
                component_id,
                animation_id,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            offset,
        ))
    }
}

/// [LayoutState](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/StateLayout.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutState {
    pub component_id: i32,
    pub animation_id: i32,
    pub index_id: i32,
    pub modifiers: Vec<crate::operations::Operations>,
    pub children: Vec<crate::operations::Operations>,
}

impl Container for LayoutState {
    fn children(&self) -> &[crate::operations::Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[crate::operations::Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<crate::operations::Operations> {
        &mut self.modifiers
    }
}

impl LayoutState {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 20 {
            return Err("Data too short for LayoutState".to_string());
        }
        let component_id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        let animation_id = i32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Failed to convert animation_id".to_string())?,
        );
        // data[8..12] and data[12..16] are horizontal/vertical positioning (unused in read)
        let index_id = i32::from_be_bytes(
            data[16..20]
                .try_into()
                .map_err(|_| "Failed to convert index_id".to_string())?,
        );
        Ok((
            LayoutState {
                component_id,
                animation_id,
                index_id,
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            20,
        ))
    }
}

/// [LayoutFitBox](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutFitBox {
    pub modifiers: Vec<Operations>,
    pub children: Vec<Operations>,
}

impl Container for LayoutFitBox {
    fn children(&self) -> &[Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.modifiers
    }
}

impl LayoutFitBox {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            LayoutFitBox {
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            0,
        ))
    }
}

/// [LayoutCollapsibleRow](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutCollapsibleRow {
    pub modifiers: Vec<Operations>,
    pub children: Vec<Operations>,
}

impl Container for LayoutCollapsibleRow {
    fn children(&self) -> &[Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.modifiers
    }
}

impl LayoutCollapsibleRow {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            LayoutCollapsibleRow {
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            0,
        ))
    }
}

/// [LayoutCollapsibleColumn](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutCollapsibleColumn {
    pub modifiers: Vec<Operations>,
    pub children: Vec<Operations>,
}

impl Container for LayoutCollapsibleColumn {
    fn children(&self) -> &[Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.modifiers
    }
}

impl LayoutCollapsibleColumn {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            LayoutCollapsibleColumn {
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            0,
        ))
    }
}

/// [LayoutCompute](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutCompute {
    pub modifiers: Vec<Operations>,
    pub children: Vec<Operations>,
}

impl Container for LayoutCompute {
    fn children(&self) -> &[Operations] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.children
    }
    fn modifiers(&self) -> &[Operations] {
        &self.modifiers
    }
    fn modifiers_mut(&mut self) -> &mut Vec<Operations> {
        &mut self.modifiers
    }
}

impl LayoutCompute {
    pub fn read(_data: &[u8]) -> Result<(Self, usize), String> {
        Ok((
            LayoutCompute {
                modifiers: Vec::new(),
                children: Vec::new(),
            },
            0,
        ))
    }
}
