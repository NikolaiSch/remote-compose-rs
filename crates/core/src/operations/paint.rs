use crate::operations::read_float_expression_from_single_float;
use remote_compose_expressions::FloatExpression;

#[derive(Debug, Clone, PartialEq)]
pub enum PaintChange {
    Color(i32),
    TextSize(FloatExpression),
    Typeface {
        weight: i32,
        italic: bool,
        font_data: bool,
        font_type: i32,
    },
    StrokeWidth(FloatExpression),
    StrokeMiter(FloatExpression),
    StrokeCap(i32),
    StrokeJoin(i32),
    Style(i32),
    Alpha(FloatExpression),
    BlendMode(i32),
    Shader(i32),
    ColorFilter {
        color: i32,
        mode: i32,
    },
    MaskFilter(i32),
    FilterQuality(i32),
    DrawStyle(i32),
    LinearGradient {
        colors: Vec<i32>,
        stops: Vec<FloatExpression>,
        start_x: FloatExpression,
        start_y: FloatExpression,
        end_x: FloatExpression,
        end_y: FloatExpression,
        tile_mode: i32,
    },
    RadialGradient {
        colors: Vec<i32>,
        stops: Vec<FloatExpression>,
        center_x: FloatExpression,
        center_y: FloatExpression,
        radius: FloatExpression,
        tile_mode: i32,
    },
    SweepGradient {
        colors: Vec<i32>,
        stops: Vec<FloatExpression>,
        center_x: FloatExpression,
        center_y: FloatExpression,
    },
    AntiAlias(bool),
    FilterBitmap(bool),
    ShaderMatrix(FloatExpression),
    ClearColorFilter,
    Unknown(i32, i32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaintValues {
    pub changes: Vec<PaintChange>,
}

impl PaintValues {
    pub const TEXT_SIZE: i32 = 1;
    pub const COLOR: i32 = 4;
    pub const STROKE_WIDTH: i32 = 5;
    pub const STROKE_MITER: i32 = 6;
    pub const STROKE_CAP: i32 = 7;
    pub const STYLE: i32 = 8;
    pub const SHADER: i32 = 9;
    pub const IMAGE_FILTER_QUALITY: i32 = 10;
    pub const GRADIENT: i32 = 11;
    pub const ALPHA: i32 = 12;
    pub const COLOR_FILTER: i32 = 13;
    pub const ANTI_ALIAS: i32 = 14;
    pub const STROKE_JOIN: i32 = 15;
    pub const TYPEFACE: i32 = 16;
    pub const FILTER_BITMAP: i32 = 17;
    pub const BLEND_MODE: i32 = 18;
    pub const COLOR_ID: i32 = 19;
    pub const COLOR_FILTER_ID: i32 = 20;
    pub const CLEAR_COLOR_FILTER: i32 = 21;
    pub const SHADER_MATRIX: i32 = 22;
    pub const FONT_AXIS: i32 = 23;
    pub const TEXTURE: i32 = 24;
    pub const PATH_EFFECT: i32 = 25;
    pub const FALLBACK_TYPEFACE: i32 = 26;

    pub const LINEAR_GRADIENT: i32 = 0;
    pub const RADIAL_GRADIENT: i32 = 1;
    pub const SWEEP_GRADIENT: i32 = 2;

    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;
        if data.len() < 4 {
            return Err("Data too short for PaintValues header".to_string());
        }
        let count = i32::from_be_bytes(
            data[offset..offset + 4]
                .try_into()
                .map_err(|_| "Failed to convert count".to_string())?,
        ) as usize;
        offset += 4;

        if data.len() < offset + count * 4 {
            return Err("Data too short for PaintValues content".to_string());
        }

        let mut raw_values = Vec::with_capacity(count);
        for _ in 0..count {
            raw_values.push(i32::from_be_bytes(
                data[offset..offset + 4]
                    .try_into()
                    .map_err(|_| "Failed to convert value".to_string())?,
            ));
            offset += 4;
        }

        let mut changes = Vec::new();
        let mut i = 0;
        let data_start = 4;
        while i < raw_values.len() {
            let cmd = raw_values[i];
            i += 1;
            let type_ = cmd & 0xFFFF;
            let change = match type_ {
                Self::TEXT_SIZE => {
                    if i < raw_values.len() {
                        let (expr, _) =
                            read_float_expression_from_single_float(&data[data_start + i * 4..])?;
                        i += 1;
                        PaintChange::TextSize(expr)
                    } else {
                        return Err("Missing data for TEXT_SIZE".to_string());
                    }
                }
                Self::COLOR | Self::COLOR_ID => {
                    if i < raw_values.len() {
                        let val = raw_values[i];
                        i += 1;
                        PaintChange::Color(val)
                    } else {
                        return Err("Missing data for COLOR".to_string());
                    }
                }
                Self::STROKE_WIDTH => {
                    if i < raw_values.len() {
                        let (expr, _) =
                            read_float_expression_from_single_float(&data[data_start + i * 4..])?;
                        i += 1;
                        PaintChange::StrokeWidth(expr)
                    } else {
                        return Err("Missing data for STROKE_WIDTH".to_string());
                    }
                }
                Self::STROKE_MITER => {
                    if i < raw_values.len() {
                        let (expr, _) =
                            read_float_expression_from_single_float(&data[data_start + i * 4..])?;
                        i += 1;
                        PaintChange::StrokeMiter(expr)
                    } else {
                        return Err("Missing data for STROKE_MITER".to_string());
                    }
                }
                Self::STROKE_CAP => PaintChange::StrokeCap(cmd >> 16),
                Self::STYLE => PaintChange::Style(cmd >> 16),
                Self::SHADER => {
                    if i < raw_values.len() {
                        let val = raw_values[i];
                        i += 1;
                        PaintChange::Shader(val)
                    } else {
                        return Err("Missing data for SHADER".to_string());
                    }
                }
                Self::IMAGE_FILTER_QUALITY => PaintChange::FilterQuality(cmd >> 16),
                Self::ALPHA => {
                    if i < raw_values.len() {
                        let (expr, _) =
                            read_float_expression_from_single_float(&data[data_start + i * 4..])?;
                        i += 1;
                        PaintChange::Alpha(expr)
                    } else {
                        return Err("Missing data for ALPHA".to_string());
                    }
                }
                Self::COLOR_FILTER | Self::COLOR_FILTER_ID => {
                    if i < raw_values.len() {
                        let val = raw_values[i];
                        i += 1;
                        PaintChange::ColorFilter {
                            color: val,
                            mode: cmd >> 16,
                        }
                    } else {
                        return Err("Missing data for COLOR_FILTER".to_string());
                    }
                }
                Self::STROKE_JOIN => PaintChange::StrokeJoin(cmd >> 16),
                Self::TYPEFACE => {
                    let style = cmd >> 16;
                    let weight = style & 0x3ff;
                    let italic = (style >> 10) > 0;
                    let font_data = (style & 1024) > 0;
                    if i < raw_values.len() {
                        let font_type = raw_values[i];
                        i += 1;
                        PaintChange::Typeface {
                            weight,
                            italic,
                            font_data,
                            font_type,
                        }
                    } else {
                        return Err("Missing data for TYPEFACE".to_string());
                    }
                }
                Self::BLEND_MODE => PaintChange::BlendMode(cmd >> 16),
                Self::GRADIENT => {
                    let gradient_type = cmd >> 16;
                    if i >= raw_values.len() {
                        return Err("Missing data for GRADIENT length".to_string());
                    }
                    let colors_len = (raw_values[i] & 0xFF) as usize;
                    i += 1;
                    let mut colors = Vec::with_capacity(colors_len);
                    for _ in 0..colors_len {
                        if i >= raw_values.len() {
                            return Err("Missing GRADIENT color".to_string());
                        }
                        colors.push(raw_values[i]);
                        i += 1;
                    }
                    if i >= raw_values.len() {
                        return Err("Missing data for STOPS length".to_string());
                    }
                    let stops_len = raw_values[i] as usize;
                    i += 1;
                    let mut stops = Vec::new();
                    if stops_len > 0 {
                        stops.reserve(colors_len);
                        for _ in 0..colors_len {
                            if i >= raw_values.len() {
                                return Err("Missing GRADIENT stop".to_string());
                            }
                            let (expr, _) = read_float_expression_from_single_float(
                                &data[data_start + i * 4..],
                            )?;
                            stops.push(expr);
                            i += 1;
                        }
                    }

                    match gradient_type {
                        Self::LINEAR_GRADIENT => {
                            if i + 4 >= raw_values.len() {
                                return Err("Missing data for LINEAR_GRADIENT".to_string());
                            }
                            let (start_x, _) = read_float_expression_from_single_float(
                                &data[data_start + (i) * 4..],
                            )?;
                            let (start_y, _) = read_float_expression_from_single_float(
                                &data[data_start + (i + 1) * 4..],
                            )?;
                            let (end_x, _) = read_float_expression_from_single_float(
                                &data[data_start + (i + 2) * 4..],
                            )?;
                            let (end_y, _) = read_float_expression_from_single_float(
                                &data[data_start + (i + 3) * 4..],
                            )?;
                            let tile_mode = raw_values[i + 4];
                            i += 5;
                            PaintChange::LinearGradient {
                                colors,
                                stops,
                                start_x,
                                start_y,
                                end_x,
                                end_y,
                                tile_mode,
                            }
                        }
                        Self::RADIAL_GRADIENT => {
                            if i + 3 >= raw_values.len() {
                                return Err("Missing data for RADIAL_GRADIENT".to_string());
                            }
                            let (center_x, _) = read_float_expression_from_single_float(
                                &data[data_start + (i) * 4..],
                            )?;
                            let (center_y, _) = read_float_expression_from_single_float(
                                &data[data_start + (i + 1) * 4..],
                            )?;
                            let (radius, _) = read_float_expression_from_single_float(
                                &data[data_start + (i + 2) * 4..],
                            )?;
                            let tile_mode = raw_values[i + 3];
                            i += 4;
                            PaintChange::RadialGradient {
                                colors,
                                stops,
                                center_x,
                                center_y,
                                radius,
                                tile_mode,
                            }
                        }
                        Self::SWEEP_GRADIENT => {
                            if i + 1 >= raw_values.len() {
                                return Err("Missing data for SWEEP_GRADIENT".to_string());
                            }
                            let (center_x, _) = read_float_expression_from_single_float(
                                &data[data_start + (i) * 4..],
                            )?;
                            let (center_y, _) = read_float_expression_from_single_float(
                                &data[data_start + (i + 1) * 4..],
                            )?;
                            i += 2;
                            PaintChange::SweepGradient {
                                colors,
                                stops,
                                center_x,
                                center_y,
                            }
                        }
                        _ => PaintChange::Unknown(type_, cmd >> 16),
                    }
                }
                Self::ANTI_ALIAS => PaintChange::AntiAlias((cmd >> 16) != 0),
                Self::FILTER_BITMAP => PaintChange::FilterBitmap((cmd >> 16) != 0),
                Self::SHADER_MATRIX => {
                    if i < raw_values.len() {
                        let (expr, _) =
                            read_float_expression_from_single_float(&data[data_start + i * 4..])?;
                        i += 1;
                        PaintChange::ShaderMatrix(expr)
                    } else {
                        return Err("Missing data for SHADER_MATRIX".to_string());
                    }
                }
                Self::CLEAR_COLOR_FILTER => PaintChange::ClearColorFilter,
                _ => return Err(format!("Unknown paint change type {}", type_).to_string()),
            };
            changes.push(change);
        }

        Ok((PaintValues { changes }, offset))
    }
}
