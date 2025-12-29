pub mod actions;
pub mod canvas;
pub mod header;
pub mod layouts;
pub mod modifiers;
pub mod paint;
pub mod primitives;
pub mod semantics;
pub mod state;

pub use actions::*;
pub use canvas::*;
pub use header::*;
pub use layouts::*;
pub use modifiers::*;
pub use paint::*;
pub use primitives::*;
pub use remote_compose_expressions::remote_path::RemotePath;
pub use remote_compose_expressions::FloatExpression;
pub use semantics::*;
pub use state::*;

pub trait Operation: Sized {
    fn opcode(&self) -> OpCode;
}

pub trait Container {
    fn children(&self) -> &[Operations];
    fn children_mut(&mut self) -> &mut Vec<Operations>;
    fn modifiers(&self) -> &[Operations];
    fn modifiers_mut(&mut self) -> &mut Vec<Operations>;
}

pub trait ActionList {
    fn actions(&self) -> &[Operations];
    fn actions_mut(&mut self) -> &mut Vec<Operations>;
}

impl Container for Operations {
    fn children(&self) -> &[Operations] {
        match self {
            Operations::LayoutRoot { children, .. }
            | Operations::LayoutBox { children, .. }
            | Operations::LayoutRow { children, .. }
            | Operations::LayoutColumn { children, .. }
            | Operations::LayoutCanvas { children, .. }
            | Operations::LayoutCanvasContent { children, .. }
            | Operations::LayoutText { children, .. }
            | Operations::LayoutImage { children, .. }
            | Operations::LayoutState { children, .. }
            | Operations::ComponentStart { children, .. }
            | Operations::LayoutFitBox { children, .. }
            | Operations::LayoutCollapsibleRow { children, .. }
            | Operations::LayoutCollapsibleColumn { children, .. } => children,
            Operations::LayoutCompute { children, .. }
            | Operations::CanvasOperations { children, .. }
            | Operations::ConditionalOperations { children, .. }
            | Operations::LoopStart { children, .. }
            | Operations::ModifierDrawContent { children, .. } => children,
            _ => &[],
        }
    }

    fn children_mut(&mut self) -> &mut Vec<Operations> {
        match self {
            Operations::LayoutRoot { children, .. }
            | Operations::LayoutBox { children, .. }
            | Operations::LayoutRow { children, .. }
            | Operations::LayoutColumn { children, .. }
            | Operations::LayoutCanvas { children, .. }
            | Operations::LayoutCanvasContent { children, .. }
            | Operations::LayoutText { children, .. }
            | Operations::LayoutImage { children, .. }
            | Operations::LayoutState { children, .. }
            | Operations::ComponentStart { children, .. }
            | Operations::LayoutFitBox { children, .. }
            | Operations::LayoutCollapsibleRow { children, .. }
            | Operations::LayoutCollapsibleColumn { children, .. }
            | Operations::LayoutCompute { children, .. }
            | Operations::CanvasOperations { children, .. }
            | Operations::ConditionalOperations { children, .. }
            | Operations::LoopStart { children, .. }
            | Operations::ModifierDrawContent { children, .. } => children,
            _ => panic!("Not a container"),
        }
    }

    fn modifiers(&self) -> &[Operations] {
        match self {
            Operations::LayoutRoot { modifiers, .. }
            | Operations::LayoutBox { modifiers, .. }
            | Operations::LayoutRow { modifiers, .. }
            | Operations::LayoutColumn { modifiers, .. }
            | Operations::LayoutCanvas { modifiers, .. }
            | Operations::LayoutCanvasContent { modifiers, .. }
            | Operations::LayoutText { modifiers, .. }
            | Operations::LayoutImage { modifiers, .. }
            | Operations::LayoutState { modifiers, .. }
            | Operations::ComponentStart { modifiers, .. }
            | Operations::LayoutFitBox { modifiers, .. }
            | Operations::LayoutCollapsibleRow { modifiers, .. }
            | Operations::LayoutCollapsibleColumn { modifiers, .. } => modifiers,
            Operations::LayoutCompute { modifiers, .. }
            | Operations::CanvasOperations { modifiers, .. }
            | Operations::ConditionalOperations { modifiers, .. }
            | Operations::LoopStart { modifiers, .. }
            | Operations::ModifierDrawContent { modifiers, .. } => modifiers,
            _ => &[],
        }
    }

    fn modifiers_mut(&mut self) -> &mut Vec<Operations> {
        match self {
            Operations::LayoutRoot { modifiers, .. }
            | Operations::LayoutBox { modifiers, .. }
            | Operations::LayoutRow { modifiers, .. }
            | Operations::LayoutColumn { modifiers, .. }
            | Operations::LayoutCanvas { modifiers, .. }
            | Operations::LayoutCanvasContent { modifiers, .. }
            | Operations::LayoutText { modifiers, .. }
            | Operations::LayoutImage { modifiers, .. }
            | Operations::LayoutState { modifiers, .. }
            | Operations::ComponentStart { modifiers, .. }
            | Operations::LayoutFitBox { modifiers, .. }
            | Operations::LayoutCollapsibleRow { modifiers, .. }
            | Operations::LayoutCollapsibleColumn { modifiers, .. }
            | Operations::LayoutCompute { modifiers, .. }
            | Operations::CanvasOperations { modifiers, .. }
            | Operations::ConditionalOperations { modifiers, .. }
            | Operations::LoopStart { modifiers, .. }
            | Operations::ModifierDrawContent { modifiers, .. } => modifiers,
            _ => panic!("Not a container"),
        }
    }
}
impl ActionList for Operations {
    fn actions(&self) -> &[Operations] {
        match self {
            Operations::ModifierClick { actions }
            | Operations::ModifierTouchDown { actions }
            | Operations::ModifierTouchUp { actions }
            | Operations::ModifierTouchCancel { actions } => actions,
            _ => &[],
        }
    }

    fn actions_mut(&mut self) -> &mut Vec<Operations> {
        match self {
            Operations::ModifierClick { actions }
            | Operations::ModifierTouchDown { actions }
            | Operations::ModifierTouchUp { actions }
            | Operations::ModifierTouchCancel { actions } => actions,
            _ => panic!("Not an action list"),
        }
    }
}

use remote_compose_expressions::parse_to_tree;

pub fn read_float_expression(data: &[u8]) -> Result<FloatExpression, String> {
    let count = data.len() / 4;
    let mut floats = Vec::with_capacity(count);
    for i in 0..count {
        let val = f32::from_bits(u32::from_be_bytes(
            data[i * 4..i * 4 + 4]
                .try_into()
                .map_err(|_| "Invalid data length".to_string())?,
        ));
        floats.push(val);
    }

    let expr = parse_to_tree(&floats)?;
    Ok(expr)
}

pub fn read_float_expression_from_single_float(
    data: &[u8],
) -> Result<(FloatExpression, usize), String> {
    if data.len() < 4 {
        return Err("Invalid data length".to_string());
    }
    let bits = u32::from_be_bytes(
        data[0..4]
            .try_into()
            .map_err(|_| "Invalid data length".to_string())?,
    );
    let value = f32::from_bits(bits);

    if value.is_nan() {
        if remote_compose_expressions::is_normal_variable(value)
            || remote_compose_expressions::is_system_variable(value)
        {
            let id = remote_compose_expressions::id_from_nan(value);
            return Ok((FloatExpression::Variable(id), 4));
        }
    }
    Ok((FloatExpression::Value(value), 4))
}

/// [Operations](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Header = 0,
    ComponentStart = 2,

    LoadBitmap = 4,
    AnimationSpec = 14,
    ModifierWidth = 16,
    ClipPath = 38,
    ClipRect = 39,
    PaintValues = 40,
    DrawRect = 42,
    DrawTextRun = 43,
    DrawBitmap = 44,
    DataShader = 45,
    DrawCircle = 46,
    DrawLine = 47,
    DrawBitmapFontTextRun = 48,
    DrawBitmapFontTextRunOnPath = 49,
    DrawRoundRect = 51,
    DrawSector = 52,
    DrawTextOnPath = 53,
    ModifierRoundedClipRect = 54,
    ModifierBackground = 55,
    DrawOval = 56,
    DrawTextOnCircle = 57,
    ModifierPadding = 58,
    ModifierClick = 59,
    Theme = 63,
    ClickArea = 64,
    RootContentBehavior = 65,
    DrawBitmapInt = 66,
    ModifierHeight = 67,
    DataFloat = 80,
    AnimatedFloat = 81,
    DataBitmap = 101,
    DataText = 102,
    RootContentDescription = 103,
    ModifierBorder = 107,
    ModifierClipRect = 108,
    DataPath = 123,
    DrawPath = 124,
    DrawTweenPath = 125,
    MatrixScale = 126,
    MatrixTranslate = 127,
    MatrixSkew = 128,
    MatrixRotate = 129,
    MatrixSave = 130,
    MatrixRestore = 131,
    MatrixSet = 132,
    DrawTextAnchor = 133,
    ColorExpressions = 134,
    TextFromFloat = 135,
    TextMerge = 136,
    NamedVariable = 137,
    ColorConstant = 138,
    DrawContent = 139,
    DataInt = 140,
    DataBoolean = 143,
    IntegerExpression = 144,
    IdMap = 145,
    IdList = 146,
    FloatList = 147,
    DataLong = 148,
    DrawBitmapScaled = 149,
    ComponentValue = 150,
    TextLookup = 151,
    DrawArc = 152,
    TextLookupInt = 153,
    DataMapLookup = 154,
    TextMeasure = 155,
    TextLength = 156,
    TouchExpression = 157,
    PathTween = 158,
    PathCreate = 159,
    PathAdd = 160,
    ParticleDefine = 161,
    ParticleProcess = 162,
    ParticleLoop = 163,
    ImpulseStart = 164,
    ImpulseProcess = 165,
    FunctionCall = 166,
    DataBitmapFont = 167,
    FunctionDefine = 168,
    AttributeText = 170,
    AttributeImage = 171,
    AttributeTime = 172,
    CanvasOperations = 173,
    ModifierDrawContent = 174,
    PathCombine = 175,
    LayoutFitBox = 176,
    HapticFeedback = 177,
    ConditionalOperations = 178,
    DebugMessage = 179,
    AttributeColor = 180,
    MatrixFromPath = 181,
    TextSubtext = 182,
    BitmapTextMeasure = 183,
    DrawBitmapTextAnchored = 184,
    Rem = 185,
    MatrixConstant = 186,
    MatrixExpression = 187,
    MatrixVectorMath = 188,
    DataFont = 189,
    DrawToBitmap = 190,
    WakeIn = 191,
    IdLookup = 192,
    PathExpression = 193,
    ParticleCompare = 194,
    Update = 195,
    ColorTheme = 196,
    DynamicFloatList = 197,
    UpdateDynamicFloatList = 198,
    TextTransform = 199,
    LayoutRoot = 200,
    LayoutContent = 201,
    LayoutBox = 202,
    LayoutRow = 203,
    LayoutColumn = 204,
    LayoutCanvas = 205,
    LayoutCanvasContent = 207,
    LayoutText = 208,
    HostAction = 209,
    HostNamedAction = 210,
    ModifierVisibility = 211,
    ValueIntegerChangeAction = 212,
    ValueStringChangeAction = 213,
    ContainerEnd = 214,
    LoopStart = 215,
    HostMetadataAction = 216,
    LayoutState = 217,
    ValueIntegerExpressionChangeAction = 218,
    ModifierTouchDown = 219,
    ModifierTouchUp = 220,
    ModifierOffset = 221,
    ValueFloatChangeAction = 222,
    ModifierZIndex = 223,
    ModifierGraphicsLayer = 224,
    ModifierTouchCancel = 225,
    ModifierScroll = 226,
    ValueFloatExpressionChangeAction = 227,
    ModifierMarquee = 228,
    ModifierRipple = 229,
    LayoutCollapsibleRow = 230,
    ModifierWidthIn = 231,
    ModifierHeightIn = 232,
    LayoutCollapsibleColumn = 233,
    LayoutImage = 234,
    ModifierCollapsiblePriority = 235,
    RunAction = 236,
    ModifierAlignBy = 237,
    LayoutCompute = 238,
    CoreText = 239,
    AccessibilitySemantics = 250,
    ExtendedOpcode = 255,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Header),
            2 => Ok(OpCode::ComponentStart),
            4 => Ok(OpCode::LoadBitmap),
            14 => Ok(OpCode::AnimationSpec),
            16 => Ok(OpCode::ModifierWidth),
            38 => Ok(OpCode::ClipPath),
            39 => Ok(OpCode::ClipRect),
            40 => Ok(OpCode::PaintValues),
            42 => Ok(OpCode::DrawRect),
            43 => Ok(OpCode::DrawTextRun),
            44 => Ok(OpCode::DrawBitmap),
            45 => Ok(OpCode::DataShader),
            46 => Ok(OpCode::DrawCircle),
            47 => Ok(OpCode::DrawLine),
            48 => Ok(OpCode::DrawBitmapFontTextRun),
            49 => Ok(OpCode::DrawBitmapFontTextRunOnPath),
            51 => Ok(OpCode::DrawRoundRect),
            52 => Ok(OpCode::DrawSector),
            53 => Ok(OpCode::DrawTextOnPath),
            54 => Ok(OpCode::ModifierRoundedClipRect),
            55 => Ok(OpCode::ModifierBackground),
            56 => Ok(OpCode::DrawOval),
            57 => Ok(OpCode::DrawTextOnCircle),
            58 => Ok(OpCode::ModifierPadding),
            59 => Ok(OpCode::ModifierClick),
            63 => Ok(OpCode::Theme),
            64 => Ok(OpCode::ClickArea),
            65 => Ok(OpCode::RootContentBehavior),
            66 => Ok(OpCode::DrawBitmapInt),
            67 => Ok(OpCode::ModifierHeight),
            80 => Ok(OpCode::DataFloat),
            81 => Ok(OpCode::AnimatedFloat),
            101 => Ok(OpCode::DataBitmap),
            102 => Ok(OpCode::DataText),
            103 => Ok(OpCode::RootContentDescription),
            107 => Ok(OpCode::ModifierBorder),
            108 => Ok(OpCode::ModifierClipRect),
            123 => Ok(OpCode::DataPath),
            124 => Ok(OpCode::DrawPath),
            125 => Ok(OpCode::DrawTweenPath),
            126 => Ok(OpCode::MatrixScale),
            127 => Ok(OpCode::MatrixTranslate),
            128 => Ok(OpCode::MatrixSkew),
            129 => Ok(OpCode::MatrixRotate),
            130 => Ok(OpCode::MatrixSave),
            131 => Ok(OpCode::MatrixRestore),
            132 => Ok(OpCode::MatrixSet),
            133 => Ok(OpCode::DrawTextAnchor),
            134 => Ok(OpCode::ColorExpressions),
            135 => Ok(OpCode::TextFromFloat),
            136 => Ok(OpCode::TextMerge),
            137 => Ok(OpCode::NamedVariable),
            138 => Ok(OpCode::ColorConstant),
            139 => Ok(OpCode::DrawContent),
            140 => Ok(OpCode::DataInt),
            143 => Ok(OpCode::DataBoolean),
            144 => Ok(OpCode::IntegerExpression),
            145 => Ok(OpCode::IdMap),
            146 => Ok(OpCode::IdList),
            147 => Ok(OpCode::FloatList),
            148 => Ok(OpCode::DataLong),
            149 => Ok(OpCode::DrawBitmapScaled),
            150 => Ok(OpCode::ComponentValue),
            151 => Ok(OpCode::TextLookup),
            152 => Ok(OpCode::DrawArc),
            153 => Ok(OpCode::TextLookupInt),
            154 => Ok(OpCode::DataMapLookup),
            155 => Ok(OpCode::TextMeasure),
            156 => Ok(OpCode::TextLength),
            157 => Ok(OpCode::TouchExpression),
            158 => Ok(OpCode::PathTween),
            159 => Ok(OpCode::PathCreate),
            160 => Ok(OpCode::PathAdd),
            161 => Ok(OpCode::ParticleDefine),
            162 => Ok(OpCode::ParticleProcess),
            163 => Ok(OpCode::ParticleLoop),
            164 => Ok(OpCode::ImpulseStart),
            165 => Ok(OpCode::ImpulseProcess),
            166 => Ok(OpCode::FunctionCall),
            167 => Ok(OpCode::DataBitmapFont),
            168 => Ok(OpCode::FunctionDefine),
            170 => Ok(OpCode::AttributeText),
            171 => Ok(OpCode::AttributeImage),
            172 => Ok(OpCode::AttributeTime),
            173 => Ok(OpCode::CanvasOperations),
            174 => Ok(OpCode::ModifierDrawContent),
            175 => Ok(OpCode::PathCombine),
            176 => Ok(OpCode::LayoutFitBox),
            177 => Ok(OpCode::HapticFeedback),
            178 => Ok(OpCode::ConditionalOperations),
            179 => Ok(OpCode::DebugMessage),
            180 => Ok(OpCode::AttributeColor),
            181 => Ok(OpCode::MatrixFromPath),
            182 => Ok(OpCode::TextSubtext),
            183 => Ok(OpCode::BitmapTextMeasure),
            184 => Ok(OpCode::DrawBitmapTextAnchored),
            185 => Ok(OpCode::Rem),
            186 => Ok(OpCode::MatrixConstant),
            187 => Ok(OpCode::MatrixExpression),
            188 => Ok(OpCode::MatrixVectorMath),
            189 => Ok(OpCode::DataFont),
            190 => Ok(OpCode::DrawToBitmap),
            191 => Ok(OpCode::WakeIn),
            192 => Ok(OpCode::IdLookup),
            193 => Ok(OpCode::PathExpression),
            194 => Ok(OpCode::ParticleCompare),
            195 => Ok(OpCode::Update),
            196 => Ok(OpCode::ColorTheme),
            197 => Ok(OpCode::DynamicFloatList),
            198 => Ok(OpCode::UpdateDynamicFloatList),
            199 => Ok(OpCode::TextTransform),
            200 => Ok(OpCode::LayoutRoot),
            201 => Ok(OpCode::LayoutContent),
            202 => Ok(OpCode::LayoutBox),
            203 => Ok(OpCode::LayoutRow),
            204 => Ok(OpCode::LayoutColumn),
            205 => Ok(OpCode::LayoutCanvas),
            207 => Ok(OpCode::LayoutCanvasContent),
            208 => Ok(OpCode::LayoutText),
            209 => Ok(OpCode::HostAction),
            210 => Ok(OpCode::HostNamedAction),
            211 => Ok(OpCode::ModifierVisibility),
            212 => Ok(OpCode::ValueIntegerChangeAction),
            213 => Ok(OpCode::ValueStringChangeAction),
            214 => Ok(OpCode::ContainerEnd),
            215 => Ok(OpCode::LoopStart),
            216 => Ok(OpCode::HostMetadataAction),
            217 => Ok(OpCode::LayoutState),
            218 => Ok(OpCode::ValueIntegerExpressionChangeAction),
            219 => Ok(OpCode::ModifierTouchDown),
            220 => Ok(OpCode::ModifierTouchUp),
            221 => Ok(OpCode::ModifierOffset),
            222 => Ok(OpCode::ValueFloatChangeAction),
            223 => Ok(OpCode::ModifierZIndex),
            224 => Ok(OpCode::ModifierGraphicsLayer),
            225 => Ok(OpCode::ModifierTouchCancel),
            226 => Ok(OpCode::ModifierScroll),
            227 => Ok(OpCode::ValueFloatExpressionChangeAction),
            228 => Ok(OpCode::ModifierMarquee),
            229 => Ok(OpCode::ModifierRipple),
            230 => Ok(OpCode::LayoutCollapsibleRow),
            231 => Ok(OpCode::ModifierWidthIn),
            232 => Ok(OpCode::ModifierHeightIn),
            233 => Ok(OpCode::LayoutCollapsibleColumn),
            234 => Ok(OpCode::LayoutImage),
            235 => Ok(OpCode::ModifierCollapsiblePriority),
            236 => Ok(OpCode::RunAction),
            237 => Ok(OpCode::ModifierAlignBy),
            238 => Ok(OpCode::LayoutCompute),
            239 => Ok(OpCode::CoreText),
            250 => Ok(OpCode::AccessibilitySemantics),
            255 => Ok(OpCode::ExtendedOpcode),
            _ => Err(()),
        }
    }
}
/// [Operations](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/Operations.java)
#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    Header {
        major: u32,
        minor: u32,
        patch: u32,
        metadata: std::collections::HashMap<u16, Metadata>,
    },
    ComponentStart {
        component_type: i32,
        component_id: i32,
        width: FloatExpression,
        height: FloatExpression,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LoadBitmap,
    AnimationSpec,
    ModifierWidth {
        type_: DimensionType,
        value: FloatExpression,
    },
    ClipPath,
    ClipRect,
    PaintValues(PaintValues),
    DrawRect {
        left: FloatExpression,
        top: FloatExpression,
        right: FloatExpression,
        bottom: FloatExpression,
    },
    DrawTextRun,
    DrawBitmap,
    DataShader,
    DrawCircle,
    DrawLine {
        x1: FloatExpression,
        y1: FloatExpression,
        x2: FloatExpression,
        y2: FloatExpression,
    },
    DrawBitmapFontTextRun,
    DrawBitmapFontTextRunOnPath,
    DrawRoundRect,
    DrawSector,
    DrawTextOnPath,
    ModifierRoundedClipRect,
    ModifierBackground {
        flags: i32,
        color_id: i32,
        r: FloatExpression,
        g: FloatExpression,
        b: FloatExpression,
        a: FloatExpression,
        shape_type: ShapeType,
    },
    DrawOval,
    DrawTextOnCircle,
    ModifierPadding {
        left: FloatExpression,
        top: FloatExpression,
        right: FloatExpression,
        bottom: FloatExpression,
    },
    ModifierClick {
        actions: Vec<Operations>,
    },
    Theme,
    ClickArea,
    RootContentBehavior {
        scroll: i32,
        alignment: i32,
        sizing: i32,
        mode: i32,
    },
    DrawBitmapInt,
    ModifierHeight {
        type_: DimensionType,
        value: FloatExpression,
    },
    DataFloat {
        id: i32,
        value: f32,
    },
    AnimatedFloat {
        id: i32,
        values: FloatExpression,
        animation: Option<FloatExpression>,
    },
    DataBitmap {
        id: i32,
        width: i32,
        height: i32,
        data: Vec<u8>,
    },
    DataText {
        id: i32,
        text: String,
    },
    RootContentDescription,
    ModifierBorder,
    ModifierClipRect,
    DataPath {
        id: i32,
        winding: i32,
        path: RemotePath,
    },
    DrawPath {
        path_id: i32,
    },
    DrawTweenPath,
    MatrixScale {
        sx: FloatExpression,
        sy: FloatExpression,
    },
    MatrixTranslate {
        tx: FloatExpression,
        ty: FloatExpression,
    },
    MatrixSkew,
    MatrixRotate {
        angle: FloatExpression,
        cx: FloatExpression,
        cy: FloatExpression,
    },
    MatrixSave,
    MatrixRestore,
    MatrixSet,
    DrawTextAnchor,
    ColorExpressions,
    TextFromFloat,
    TextMerge,
    NamedVariable,
    ColorConstant,
    DrawContent,
    DataInt {
        id: i32,
        value: i32,
    },
    DataBoolean,
    IntegerExpression,
    IdMap,
    IdList,
    FloatList,
    DataLong,
    DrawBitmapScaled,
    TextLookup,
    DrawArc,
    TextLookupInt,
    DataMapLookup,
    TextMeasure,
    TextLength,
    TouchExpression,
    PathTween,
    PathCreate,
    PathAdd,
    ParticleDefine,
    ParticleProcess,
    ParticleLoop,
    ImpulseStart,
    ImpulseProcess,
    FunctionCall,
    DataBitmapFont,
    FunctionDefine,
    AttributeText,
    AttributeImage,
    AttributeTime,
    CanvasOperations {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    PathCombine,
    LayoutFitBox {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    HapticFeedback,
    ConditionalOperations {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    DebugMessage,
    AttributeColor,
    MatrixFromPath,
    TextSubtext,
    BitmapTextMeasure,
    DrawBitmapTextAnchored,
    Rem,
    MatrixConstant,
    MatrixExpression,
    MatrixVectorMath,
    DataFont,
    DrawToBitmap,
    WakeIn,
    IdLookup,
    PathExpression,
    ParticleCompare,
    Update,
    ColorTheme,
    DynamicFloatList,
    UpdateDynamicFloatList,
    TextTransform,
    LayoutRoot {
        component_id: i32,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutContent {
        component_id: i32,
    },
    LayoutBox {
        component_id: i32,
        animation_id: i32,
        horizontal_alignment: LayoutAlignment,
        vertical_alignment: LayoutAlignment,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutRow {
        component_id: i32,
        animation_id: i32,
        horizontal_alignment: LayoutAlignment,
        vertical_alignment: LayoutAlignment,
        spaced_by: FloatExpression,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutColumn {
        component_id: i32,
        animation_id: i32,
        horizontal_alignment: LayoutAlignment,
        vertical_alignment: LayoutAlignment,
        spaced_by: FloatExpression,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutCanvas {
        component_id: i32,
        animation_id: i32,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutCanvasContent {
        component_id: i32,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutText {
        component_id: i32,
        animation_id: i32,
        text_id: i32,
        color: i32,
        font_size: FloatExpression,
        font_style: FontStyle,
        font_weight: FloatExpression,
        font_family_id: i32,
        text_align: TextAlign,
        overflow: TextOverflow,
        max_lines: i32,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    LayoutImage {
        component_id: i32,
        animation_id: i32,
        bitmap_id: i32,
        scale_type: i32,
        alpha: FloatExpression,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    HostAction,
    HostNamedAction {
        text_id: i32,
        type_: i32,
        value_id: i32,
    },
    ModifierVisibility,
    ValueIntegerChangeAction {
        value_id: i32,
        value: i32,
    },
    ValueStringChangeAction,
    ContainerEnd,
    ComponentValue {
        type_: i32,
        component_id: i32,
        value_id: i32,
    },
    LoopStart {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    HostMetadataAction,
    LayoutState {
        component_id: i32,
        animation_id: i32,
        index_id: i32,
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    ValueIntegerExpressionChangeAction,
    ModifierTouchDown {
        actions: Vec<Operations>,
    },
    ModifierTouchUp {
        actions: Vec<Operations>,
    },
    ModifierOffset,
    ValueFloatChangeAction,
    ModifierZIndex,
    ModifierGraphicsLayer,
    ModifierTouchCancel {
        actions: Vec<Operations>,
    },
    ModifierScroll,
    ValueFloatExpressionChangeAction,
    ModifierMarquee,
    ModifierRipple,
    LayoutCollapsibleRow {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    ModifierWidthIn,
    ModifierHeightIn,
    LayoutCollapsibleColumn {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    ModifierCollapsiblePriority,
    RunAction,
    ModifierAlignBy,
    LayoutCompute {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    CoreText {
        text_id: i32,
        params: Vec<CoreTextParam>,
    },
    AccessibilitySemantics {
        semantics: CoreSemantics,
    },
    ModifierDrawContent {
        modifiers: Vec<Operations>,
        children: Vec<Operations>,
    },
    ExtendedOpcode,
}

impl Operations {
    pub fn as_container(&self) -> Option<&dyn Container> {
        match self {
            Operations::LayoutRoot { .. }
            | Operations::LayoutBox { .. }
            | Operations::LayoutRow { .. }
            | Operations::LayoutColumn { .. }
            | Operations::LayoutCanvas { .. }
            | Operations::LayoutCanvasContent { .. }
            | Operations::LayoutText { .. }
            | Operations::LayoutImage { .. }
            | Operations::LayoutState { .. }
            | Operations::ComponentStart { .. }
            | Operations::LayoutFitBox { .. }
            | Operations::LayoutCollapsibleRow { .. }
            | Operations::LayoutCollapsibleColumn { .. }
            | Operations::LayoutCompute { .. }
            | Operations::CanvasOperations { .. }
            | Operations::ConditionalOperations { .. }
            | Operations::LoopStart { .. }
            | Operations::ModifierDrawContent { .. } => Some(self),
            _ => None,
        }
    }

    pub fn as_container_mut(&mut self) -> Option<&mut dyn Container> {
        match self {
            Operations::LayoutRoot { .. }
            | Operations::LayoutBox { .. }
            | Operations::LayoutRow { .. }
            | Operations::LayoutColumn { .. }
            | Operations::LayoutCanvas { .. }
            | Operations::LayoutCanvasContent { .. }
            | Operations::LayoutText { .. }
            | Operations::LayoutImage { .. }
            | Operations::LayoutState { .. }
            | Operations::ComponentStart { .. }
            | Operations::LayoutFitBox { .. }
            | Operations::LayoutCollapsibleRow { .. }
            | Operations::LayoutCollapsibleColumn { .. }
            | Operations::LayoutCompute { .. }
            | Operations::CanvasOperations { .. }
            | Operations::ConditionalOperations { .. }
            | Operations::LoopStart { .. }
            | Operations::ModifierDrawContent { .. } => Some(self),
            _ => None,
        }
    }

    pub fn as_action_list(&self) -> Option<&dyn ActionList> {
        match self {
            Operations::ModifierClick { .. }
            | Operations::ModifierTouchDown { .. }
            | Operations::ModifierTouchUp { .. }
            | Operations::ModifierTouchCancel { .. } => Some(self),
            _ => None,
        }
    }

    pub fn as_action_list_mut(&mut self) -> Option<&mut dyn ActionList> {
        match self {
            Operations::ModifierClick { .. }
            | Operations::ModifierTouchDown { .. }
            | Operations::ModifierTouchUp { .. }
            | Operations::ModifierTouchCancel { .. } => Some(self),
            _ => None,
        }
    }

    pub fn has_modifiers(&self) -> bool {
        match self {
            Operations::CanvasOperations { .. }
            | Operations::ConditionalOperations { .. }
            | Operations::LoopStart { .. } => false,
            _ => true,
        }
    }
}

impl Operations {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.is_empty() {
            return Err("Empty data".to_string());
        }
        let opcode_byte = data[0];
        let opcode =
            OpCode::try_from(opcode_byte).map_err(|_| "Invalid opcode byte".to_string())?;
        let data_slice = &data[1..];

        let (op, bytes_read) = match opcode {
            OpCode::Header => {
                let (h, n) =
                    Header::read(data_slice).map_err(|e| format!("Invalid Header data: {}", e))?;
                (
                    Operations::Header {
                        major: h.major,
                        minor: h.minor,
                        patch: h.patch,
                        metadata: h.metadata,
                    },
                    n,
                )
            }
            OpCode::ComponentStart => {
                let (c, n) = ComponentStart::read(data_slice)
                    .map_err(|e| format!("Invalid ComponentStart data: {}", e))?;
                (
                    Operations::ComponentStart {
                        component_type: c.component_type,
                        component_id: c.component_id,
                        width: c.width,
                        height: c.height,
                        modifiers: c.modifiers,
                        children: c.children,
                    },
                    n,
                )
            }
            OpCode::RootContentBehavior => {
                let (r, n) = RootContentBehavior::read(data_slice)
                    .map_err(|e| format!("Invalid RootContentBehavior data: {}", e))?;
                (
                    Operations::RootContentBehavior {
                        scroll: r.scroll,
                        alignment: r.alignment,
                        sizing: r.sizing,
                        mode: r.mode,
                    },
                    n,
                )
            }
            OpCode::ContainerEnd => {
                let (_, n) = ContainerEnd::read(data_slice)
                    .map_err(|e| format!("Invalid ContainerEnd data: {}", e))?;
                (Operations::ContainerEnd, n)
            }
            OpCode::MatrixSave => (Operations::MatrixSave, 0),
            OpCode::MatrixRestore => (Operations::MatrixRestore, 0),
            OpCode::MatrixTranslate => {
                let (m, n) = MatrixTranslate::read(data_slice)
                    .map_err(|e| format!("Invalid MatrixTranslate data: {}", e))?;
                (Operations::MatrixTranslate { tx: m.tx, ty: m.ty }, n)
            }
            OpCode::MatrixScale => {
                let (m, n) = MatrixScale::read(data_slice)
                    .map_err(|e| format!("Invalid MatrixScale data: {}", e))?;
                (Operations::MatrixScale { sx: m.sx, sy: m.sy }, n)
            }
            OpCode::MatrixRotate => {
                let (m, n) = MatrixRotate::read(data_slice)
                    .map_err(|e| format!("Invalid MatrixRotate data: {}", e))?;
                (
                    Operations::MatrixRotate {
                        angle: m.angle,
                        cx: m.cx,
                        cy: m.cy,
                    },
                    n,
                )
            }
            OpCode::DataInt => {
                let (d, n) = DataInt::read(data_slice)
                    .map_err(|e| format!("Invalid DataInt data: {}", e))?;
                (
                    Operations::DataInt {
                        id: d.id,
                        value: d.value,
                    },
                    n,
                )
            }
            OpCode::DataText => {
                let (d, n) = DataText::read(data_slice)
                    .map_err(|e| format!("Invalid DataText data: {}", e))?;
                (
                    Operations::DataText {
                        id: d.id,
                        text: d.text,
                    },
                    n,
                )
            }
            OpCode::ModifierWidth => {
                let (m, n) = ModifierWidth::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierWidth data: {}", e))?;
                (
                    Operations::ModifierWidth {
                        type_: m.type_,
                        value: m.value,
                    },
                    n,
                )
            }
            OpCode::DrawLine => {
                let (d, n) = DrawLine::read(data_slice)
                    .map_err(|e| format!("Invalid DrawLine data: {}", e))?;
                (
                    Operations::DrawLine {
                        x1: d.x1,
                        y1: d.y1,
                        x2: d.x2,
                        y2: d.y2,
                    },
                    n,
                )
            }
            OpCode::DrawPath => {
                let (d, n) = DrawPath::read(data_slice)
                    .map_err(|e| format!("Invalid DrawPath data: {}", e))?;
                (Operations::DrawPath { path_id: d.path_id }, n)
            }
            OpCode::DrawRect => {
                let (d, n) = DrawRect::read(data_slice)
                    .map_err(|e| format!("Invalid DrawRect data: {}", e))?;
                (
                    Operations::DrawRect {
                        left: d.left,
                        top: d.top,
                        right: d.right,
                        bottom: d.bottom,
                    },
                    n,
                )
            }
            OpCode::ModifierBackground => {
                let (m, n) = ModifierBackground::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierBackground data: {}", e))?;
                (
                    Operations::ModifierBackground {
                        flags: m.flags,
                        color_id: m.color_id,
                        r: m.r,
                        g: m.g,
                        b: m.b,
                        a: m.a,
                        shape_type: m.shape_type,
                    },
                    n,
                )
            }
            OpCode::ModifierPadding => {
                let (m, n) = ModifierPadding::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierPadding data: {}", e))?;
                (
                    Operations::ModifierPadding {
                        left: m.left,
                        top: m.top,
                        right: m.right,
                        bottom: m.bottom,
                    },
                    n,
                )
            }
            OpCode::ModifierClick => {
                let (m, n) = ModifierClick::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierClick data: {}", e))?;
                (Operations::ModifierClick { actions: m.actions }, n)
            }
            OpCode::ModifierHeight => {
                let (m, n) = ModifierHeight::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierHeight data: {}", e))?;
                (
                    Operations::ModifierHeight {
                        type_: m.type_,
                        value: m.value,
                    },
                    n,
                )
            }
            OpCode::ModifierDrawContent => (
                Operations::ModifierDrawContent {
                    modifiers: Vec::new(),
                    children: Vec::new(),
                },
                0,
            ),
            OpCode::ModifierOffset => (Operations::ModifierOffset, 0), // Placeholder
            OpCode::ModifierZIndex => (Operations::ModifierZIndex, 0), // Placeholder
            OpCode::ModifierGraphicsLayer => (Operations::ModifierGraphicsLayer, 0), // Placeholder
            OpCode::ModifierVisibility => (Operations::ModifierVisibility, 0), // Placeholder
            OpCode::DrawContent => (Operations::DrawContent, 0),
            OpCode::CanvasOperations => (
                Operations::CanvasOperations {
                    modifiers: Vec::new(),
                    children: Vec::new(),
                },
                0,
            ),
            OpCode::ConditionalOperations => (
                Operations::ConditionalOperations {
                    modifiers: Vec::new(),
                    children: Vec::new(),
                },
                0,
            ),
            OpCode::LoopStart => (
                Operations::LoopStart {
                    modifiers: Vec::new(),
                    children: Vec::new(),
                },
                0,
            ),
            OpCode::PathCombine => (Operations::PathCombine, 0),
            OpCode::LayoutRoot => {
                let (l, n) = LayoutRoot::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutRoot data: {}", e))?;
                (
                    Operations::LayoutRoot {
                        component_id: l.component_id,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutContent => {
                let (l, n) = LayoutContent::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutContent data: {}", e))?;
                (
                    Operations::LayoutContent {
                        component_id: l.component_id,
                    },
                    n,
                )
            }
            OpCode::LayoutBox => {
                let (l, n) = LayoutBox::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutBox data: {}", e))?;
                (
                    Operations::LayoutBox {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        horizontal_alignment: l.horizontal_alignment,
                        vertical_alignment: l.vertical_alignment,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutRow => {
                let (l, n) = LayoutRow::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutRow data: {}", e))?;
                (
                    Operations::LayoutRow {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        horizontal_alignment: l.horizontal_alignment,
                        vertical_alignment: l.vertical_alignment,
                        spaced_by: l.spaced_by,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutColumn => {
                let (l, n) = LayoutColumn::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutColumn data: {}", e))?;
                (
                    Operations::LayoutColumn {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        horizontal_alignment: l.horizontal_alignment,
                        vertical_alignment: l.vertical_alignment,
                        spaced_by: l.spaced_by,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutCanvas => {
                let (l, n) = LayoutCanvas::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutCanvas data: {}", e))?;
                (
                    Operations::LayoutCanvas {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutCanvasContent => {
                let (l, n) = LayoutCanvasContent::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutCanvasContent data: {}", e))?;
                (
                    Operations::LayoutCanvasContent {
                        component_id: l.component_id,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutText => {
                let (l, n) = LayoutText::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutText data: {}", e))?;
                (
                    Operations::LayoutText {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        text_id: l.text_id,
                        color: l.color,
                        font_size: l.font_size,
                        font_style: l.font_style,
                        font_weight: l.font_weight,
                        font_family_id: l.font_family_id,
                        text_align: l.text_align,
                        overflow: l.overflow,
                        max_lines: l.max_lines,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutImage => {
                let (l, n) = LayoutImage::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutImage data: {}", e))?;
                (
                    Operations::LayoutImage {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        bitmap_id: l.bitmap_id,
                        scale_type: l.scale_type,
                        alpha: l.alpha,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::ComponentValue => {
                let (c, n) = ComponentValue::read(data_slice)
                    .map_err(|e| format!("Invalid ComponentValue data: {}", e))?;
                (
                    Operations::ComponentValue {
                        type_: c.type_,
                        component_id: c.component_id,
                        value_id: c.value_id,
                    },
                    n,
                )
            }
            OpCode::HostNamedAction => {
                let (h, n) = HostNamedAction::read(data_slice)
                    .map_err(|e| format!("Invalid HostNamedAction data: {}", e))?;
                (
                    Operations::HostNamedAction {
                        text_id: h.text_id,
                        type_: h.type_,
                        value_id: h.value_id,
                    },
                    n,
                )
            }
            OpCode::LayoutState => {
                let (l, n) = LayoutState::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutState data: {}", e))?;
                (
                    Operations::LayoutState {
                        component_id: l.component_id,
                        animation_id: l.animation_id,
                        index_id: l.index_id,
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::ValueIntegerChangeAction => {
                let (v, n) = ValueIntegerChangeAction::read(data_slice)
                    .map_err(|e| format!("Invalid ValueIntegerChangeAction data: {}", e))?;
                (
                    Operations::ValueIntegerChangeAction {
                        value_id: v.value_id,
                        value: v.value,
                    },
                    n,
                )
            }
            OpCode::ModifierTouchDown => {
                let (m, n) = ModifierTouchDown::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierTouchDown data: {}", e))?;
                (Operations::ModifierTouchDown { actions: m.actions }, n)
            }
            OpCode::ModifierTouchUp => {
                let (m, n) = ModifierTouchUp::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierTouchUp data: {}", e))?;
                (Operations::ModifierTouchUp { actions: m.actions }, n)
            }
            OpCode::ModifierTouchCancel => {
                let (m, n) = ModifierTouchCancel::read(data_slice)
                    .map_err(|e| format!("Invalid ModifierTouchCancel data: {}", e))?;
                (Operations::ModifierTouchCancel { actions: m.actions }, n)
            }
            OpCode::RunAction => (Operations::RunAction, 0),
            OpCode::LayoutFitBox => {
                let (l, n) = LayoutFitBox::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutFitBox data: {}", e))?;
                (
                    Operations::LayoutFitBox {
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutCollapsibleRow => {
                let (l, n) = LayoutCollapsibleRow::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutCollapsibleRow data: {}", e))?;
                (
                    Operations::LayoutCollapsibleRow {
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutCollapsibleColumn => {
                let (l, n) = LayoutCollapsibleColumn::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutCollapsibleColumn data: {}", e))?;
                (
                    Operations::LayoutCollapsibleColumn {
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::LayoutCompute => {
                let (l, n) = LayoutCompute::read(data_slice)
                    .map_err(|e| format!("Invalid LayoutCompute data: {}", e))?;
                (
                    Operations::LayoutCompute {
                        modifiers: l.modifiers,
                        children: l.children,
                    },
                    n,
                )
            }
            OpCode::CoreText => {
                let (c, n) = CoreText::read(data_slice)
                    .map_err(|e| format!("Invalid CoreText data: {}", e))?;
                (
                    Operations::CoreText {
                        text_id: c.text_id,
                        params: c.params,
                    },
                    n,
                )
            }
            OpCode::AccessibilitySemantics => {
                let (a, n) = CoreSemantics::read(data_slice)
                    .map_err(|e| format!("Invalid CoreSemantics data: {}", e))?;
                (Operations::AccessibilitySemantics { semantics: a }, n)
            }
            OpCode::DataFloat => {
                let (d, n) = DataFloat::read(data_slice)
                    .map_err(|e| format!("Invalid DataFloat data: {}", e))?;
                (
                    Operations::DataFloat {
                        id: d.id,
                        value: d.value,
                    },
                    n,
                )
            }
            OpCode::AnimatedFloat => {
                let (a, n) = AnimatedFloat::read(data_slice)
                    .map_err(|e| format!("Invalid AnimatedFloat data: {}", e))?;
                (
                    Operations::AnimatedFloat {
                        id: a.id,
                        values: a.values,
                        animation: a.animation,
                    },
                    n,
                )
            }
            OpCode::DataBitmap => {
                let (d, n) = DataBitmap::read(data_slice)
                    .map_err(|e| format!("Invalid DataBitmap data: {}", e))?;
                (
                    Operations::DataBitmap {
                        id: d.id,
                        width: d.width,
                        height: d.height,
                        data: d.data,
                    },
                    n,
                )
            }
            OpCode::DataPath => {
                let (d, n) = DataPath::read(data_slice)
                    .map_err(|e| format!("Invalid DataPath data: {}", e))?;
                (
                    Operations::DataPath {
                        id: d.id,
                        winding: d.winding,
                        path: d.path,
                    },
                    n,
                )
            }
            OpCode::PaintValues => {
                let (p, n) = PaintValues::read(data_slice)
                    .map_err(|e| format!("Invalid PaintValues data: {}", e))?;
                (Operations::PaintValues(p), n)
            }
            _ => return Err("Unknown opcode".to_string()),
        };

        Ok((op, bytes_read + 1))
    }
}

pub struct Document {
    pub operations: Vec<Operations>,
}

impl Document {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let mut operations = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            let (op, n) = Operations::read(&data[offset..])?;
            operations.push(op);
            offset += n;
        }
        Ok(Document { operations })
    }
}
