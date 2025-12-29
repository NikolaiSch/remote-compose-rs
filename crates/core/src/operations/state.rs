use remote_compose_expressions::remote_path::RemotePath;
use remote_compose_expressions::FloatExpression;

use crate::operations::{read_float_expression, read_float_expression_from_single_float};

/// [DataInt](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/TextData.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DataInt {
    pub id: i32,
    pub value: i32,
}

impl DataInt {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 8 {
            return Err("Data too short for DataInt".to_string());
        }
        let id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Failed to convert id".to_string())?,
        );
        let value = i32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Failed to convert value".to_string())?,
        );
        Ok((DataInt { id, value }, 8))
    }
}

/// [DataFloat](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/FloatData.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DataFloat {
    pub id: i32,
    pub value: f32,
}

impl DataFloat {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 8 {
            return Err("Data too short for DataFloat".to_string());
        }
        let id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Failed to convert id".to_string())?,
        );
        let value = f32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Failed to convert value".to_string())?,
        );
        Ok((DataFloat { id, value }, 8))
    }
}

/// [DataText](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/TextData.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DataText {
    pub id: i32,
    pub text: String,
}

impl DataText {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 8 {
            return Err("Data too short for DataText".to_string());
        }
        let id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Failed to convert id".to_string())?,
        );
        let len = i32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Failed to convert len".to_string())?,
        ) as usize;
        if data.len() < 8 + len {
            return Err("Data too short for DataText text content".to_string());
        }
        let text = String::from_utf8(data[8..8 + len].to_vec())
            .map_err(|_| "Failed to parse text as UTF-8".to_string())?;
        Ok((DataText { id, text }, 8 + len))
    }
}

/// [ComponentValue](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/ComponentValue.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentValue {
    pub type_: i32,
    pub component_id: i32,
    pub value_id: i32,
}

impl ComponentValue {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 12 {
            return Err("Data too short for ComponentValue".to_string());
        }
        let type_ = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Failed to convert type_".to_string())?,
        );
        let component_id = i32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Failed to convert component_id".to_string())?,
        );
        let value_id = i32::from_be_bytes(
            data[8..12]
                .try_into()
                .map_err(|_| "Failed to convert value_id".to_string())?,
        );
        Ok((
            ComponentValue {
                type_,
                component_id,
                value_id,
            },
            12,
        ))
    }
}

/// [AnimatedFloat](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/AnimatedFloat.java)
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedFloat {
    pub id: i32,
    pub values: FloatExpression,
    pub animation: Option<FloatExpression>,
}

impl AnimatedFloat {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        if data.len() < 8 {
            return Err("Data too short for AnimatedFloat".to_string());
        }
        let id = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert id".to_string())?,
        );
        offset += 4;
        let len = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert len".to_string())?,
        );
        offset += 4;

        let value_len = (len & 0xFFFF) as usize;
        let anim_len = ((len >> 16) & 0xFFFF) as usize;

        let values = read_float_expression(
            data.get(offset..(offset + value_len * 4))
                .ok_or("Invalid values length".to_string())?,
        )?;
        offset += value_len * 4;

        // if anim_len is 0, animation is None
        let animation = match anim_len {
            0 => None,
            _ => Some(read_float_expression(
                data.get(offset..(offset + anim_len * 4))
                    .ok_or("Invalid animation length".to_string())?,
            )?),
        };
        offset += anim_len * 4;

        Ok((
            AnimatedFloat {
                id,
                values,
                animation,
            },
            offset,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreTextParam {
    Int(u8, i32),
    Float(u8, FloatExpression), // Changed from f32
    Short(u8, i16),
    Byte(u8, u8),
    Boolean(u8, bool),
    IntArray(u8, Vec<i32>),
    FloatArray(u8, Vec<FloatExpression>), // Changed from f32
    String(u8, String),
}

/// [CoreText](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/managers/CoreText.java)
#[derive(Debug, Clone, PartialEq)]
pub struct CoreText {
    pub text_id: i32,
    pub params: Vec<CoreTextParam>,
}

impl CoreText {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 6 {
            return Err("Data too short for CoreText".to_string());
        }
        let text_id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Failed to convert text_id".to_string())?,
        );
        let params_length = i16::from_be_bytes(
            data[4..6]
                .try_into()
                .map_err(|_| "Failed to convert params_length".to_string())?,
        ) as usize;
        let mut offset = 6;
        let mut params = Vec::with_capacity(params_length);

        for _ in 0..params_length {
            if offset >= data.len() {
                return Err("Data too short for params".to_string());
            }
            let id = data[offset];
            offset += 1;

            // These types are from CommandParameters.java
            // P_INT = 1, P_FLOAT = 2, P_SHORT = 3, P_BYTE = 4, P_BOOLEAN = 5,
            // PA_INT = 6, PA_FLOAT = 7, PA_STRING = 8

            // We need to know the type for each ID. CoreText defines them.
            // P_COMPONENT_ID(1)=Int, P_ANIMATION_ID(2)=Int, P_COLOR(3)=Int, P_COLOR_ID(4)=Int,
            // P_FONT_SIZE(5)=Float, P_FONT_STYLE(6)=Int, P_FONT_WEIGHT(7)=Float, P_FONT_FAMILY(8)=Int,
            // P_TEXT_ALIGN(9)=Int, P_OVERFLOW(10)=Int, P_MAX_LINES(11)=Int, P_LETTER_SPACING(12)=Float,
            // P_LINE_HEIGHT_ADD(13)=Float, P_LINE_HEIGHT_MULTIPLIER(14)=Float, P_BREAK_STRATEGY(15)=Int,
            // P_HYPHENATION_FREQUENCY(16)=Int, P_JUSTIFICATION_MODE(17)=Int, P_UNDERLINE(18)=Boolean,
            // P_STRIKETHROUGH(19)=Boolean, P_AUTOSIZE(20)=Boolean, P_FLAGS(21)=Int,
            // P_FONT_AXIS(22)=IntArray, P_FONT_AXIS_VALUES(23)=FloatArray

            let param = match id {
                1..=4 | 6 | 8..=11 | 15..=17 | 21 => {
                    if offset + 4 > data.len() {
                        return Err("Data too short for int param".to_string());
                    }
                    let val = i32::from_be_bytes(
                        data[offset..offset + 4]
                            .try_into()
                            .map_err(|_| "Failed to convert int param".to_string())?,
                    );
                    offset += 4;
                    CoreTextParam::Int(id, val)
                }
                5 | 7 | 12..=14 => {
                    if offset + 4 > data.len() {
                        return Err("Data too short for float param".to_string());
                    }
                    let (val, width) = read_float_expression_from_single_float(
                        data.get(offset..)
                            .ok_or("Failed to get float param data".to_string())?,
                    )?;
                    offset += width;
                    CoreTextParam::Float(id, val)
                }
                18..=20 => {
                    if offset >= data.len() {
                        return Err("Data too short for boolean param".to_string());
                    }
                    let val = data[offset] != 0;
                    offset += 1;
                    CoreTextParam::Boolean(id, val)
                }
                22 => {
                    if offset + 2 > data.len() {
                        return Err("Data too short for int array param count".to_string());
                    }
                    let count = i16::from_be_bytes(
                        data[offset..offset + 2]
                            .try_into()
                            .map_err(|_| "Failed to convert int array param count".to_string())?,
                    ) as usize;
                    offset += 2;
                    if offset + count * 4 > data.len() {
                        return Err("Data too short for int array param values".to_string());
                    }
                    let mut vals = Vec::with_capacity(count);
                    for _ in 0..count {
                        vals.push(i32::from_be_bytes(
                            data[offset..offset + 4].try_into().map_err(|_| {
                                "Failed to convert int array param value".to_string()
                            })?,
                        ));
                        offset += 4;
                    }
                    CoreTextParam::IntArray(id, vals)
                }
                23 => {
                    if offset + 2 > data.len() {
                        return Err("Data too short for float array param count".to_string());
                    }
                    let count = i16::from_be_bytes(
                        data[offset..offset + 2]
                            .try_into()
                            .map_err(|_| "Failed to convert float array param count".to_string())?,
                    ) as usize;
                    offset += 2;
                    let mut vals = Vec::with_capacity(count);
                    for _ in 0..count {
                        let (value, width) = read_float_expression_from_single_float(
                            data.get(offset..)
                                .ok_or("Failed to get float array param value data".to_string())?,
                        )?;
                        vals.push(value);
                        offset += width;
                    }
                    CoreTextParam::FloatArray(id, vals)
                }
                _ => return Err(format!("Unknown parameter ID: {}", id)),
            };
            params.push(param);
        }
        Ok((CoreText { text_id, params }, offset))
    }
}

/// [DataBitmap](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/BitmapData.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DataBitmap {
    pub id: i32,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl DataBitmap {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        if data.len() < 12 {
            return Err("Data too short for DataBitmap".to_string());
        }
        let id = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert id".to_string())?,
        );
        offset += 4;
        let width = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert width".to_string())?,
        );
        offset += 4;
        let height = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert height".to_string())?,
        );
        offset += 4;

        if data.len() < offset + 4 {
            return Err("Data too short for DataBitmap data length".to_string());
        }
        let len = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert len".to_string())?,
        ) as usize;
        offset += 4;

        if data.len() < offset + len {
            return Err("Data too short for DataBitmap content".to_string());
        }
        let d = data[offset..offset + len].to_vec();
        offset += len;

        Ok((
            DataBitmap {
                id,
                width,
                height,
                data: d,
            },
            offset,
        ))
    }
}

/// [DataPath](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/PathData.java)
#[derive(Debug, Clone, PartialEq)]
pub struct DataPath {
    pub id: i32,
    pub winding: i32,
    pub path: RemotePath,
}

impl DataPath {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        if data.len() < 8 {
            return Err("Data too short for DataPath".to_string());
        }
        let full_id = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert id".to_string())?,
        );
        offset += 4;

        let winding = full_id >> 24;
        let id = full_id & 0xffffff;

        let len = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert len".to_string())?,
        ) as usize;
        offset += 4;

        if data.len() < offset + len * 4 {
            return Err("Data too short for DataPath command".to_string());
        }

        let slice = &data[offset..offset + len * 4];
        let (path, _read_len) =
            RemotePath::read(slice).map_err(|_| "Failed to read RemotePath".to_string())?;

        offset += len * 4;

        Ok((DataPath { id, winding, path }, offset))
    }
}
