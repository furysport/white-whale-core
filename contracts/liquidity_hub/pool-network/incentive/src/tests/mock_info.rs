use cosmwasm_std::{testing::mock_info, MessageInfo};

/// Creates a mock creator
pub fn mock_creator() -> MessageInfo {
    mock_info("creator", &[])
}

pub fn mock_admin() -> MessageInfo {
    mock_info("admin", &[])
}
