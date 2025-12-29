/// [CoreSemantics](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/semantics/CoreSemantics.java)
#[derive(Debug, Clone, PartialEq)]
pub struct CoreSemantics {
    pub content_description_id: i32,
    pub role: i8,
    pub text_id: i32,
    pub state_description_id: i32,
    pub mode: i8,
    pub enabled: bool,
    pub clickable: bool,
}

impl CoreSemantics {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 16 {
            return Err("Data too short for CoreSemantics".to_string());
        }
        let content_description_id = i32::from_be_bytes(data[0..4].try_into().unwrap());
        let role = data[4] as i8;
        let text_id = i32::from_be_bytes(data[5..9].try_into().unwrap());
        let state_description_id = i32::from_be_bytes(data[9..13].try_into().unwrap());
        let mode = data[13] as i8;
        let enabled = data[14] != 0;
        let clickable = data[15] != 0;
        Ok((
            CoreSemantics {
                content_description_id,
                role,
                text_id,
                state_description_id,
                mode,
                enabled,
                clickable,
            },
            16,
        ))
    }
}
