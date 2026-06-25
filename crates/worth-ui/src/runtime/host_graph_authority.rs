use crate::runtime::{WorthUiRuntimeGraphAuthority, WorthUiRuntimeHost};

impl WorthUiRuntimeHost {
    pub fn graph_authority(&self) -> &WorthUiRuntimeGraphAuthority {
        &self.graph_authority
    }
}
