/// [HostNamedAction](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/layout/modifiers/HostNamedActionOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct HostNamedAction {
    pub text_id: i32,
    pub type_: i32,
    pub value_id: i32,
}

impl HostNamedAction {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 12 {
            return Err("Data too short for HostNamedAction".to_string());
        }
        let text_id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Invalid text_id".to_string())?,
        );
        let type_ = i32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Invalid type_".to_string())?,
        );
        let value_id = i32::from_be_bytes(
            data[8..12]
                .try_into()
                .map_err(|_| "Invalid value_id".to_string())?,
        );
        Ok((
            HostNamedAction {
                text_id,
                type_,
                value_id,
            },
            12,
        ))
    }
}

/// [ValueIntegerChangeAction](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/actions/ValueIntegerChangeActionOperation.java)
#[derive(Debug, Clone, PartialEq)]
pub struct ValueIntegerChangeAction {
    pub value_id: i32,
    pub value: i32,
}

impl ValueIntegerChangeAction {
    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 8 {
            return Err("Data too short for ValueIntegerChangeAction".to_string());
        }
        let value_id = i32::from_be_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| "Invalid value_id".to_string())?,
        );
        let value = i32::from_be_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| "Invalid value".to_string())?,
        );
        Ok((ValueIntegerChangeAction { value_id, value }, 8))
    }
}
