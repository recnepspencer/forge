//! Opaque, diagnostic-grade correlation carried across the Bank external
//! rail wire.
//!
//! This identity is Bank-owned and diagnostic only. Query never treats it as
//! its own typed correlation identity, and the rail never treats it as
//! anything but an opaque tag scoped to one dispatch attempt.

use serde::{Deserialize, Serialize};

/// One dispatch attempt's opaque correlation, as seen by the external rail.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RailCorrelation {
    family: String,
    token: Vec<u8>,
}

impl RailCorrelation {
    /// `family` names the correlation family (for example
    /// `"estate-death-notice-rail"`); `token` is opaque bytes identifying
    /// this specific attempt within it.
    pub fn new(family: impl Into<String>, token: impl Into<Vec<u8>>) -> Self {
        Self {
            family: family.into(),
            token: token.into(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn token(&self) -> &[u8] {
        &self.token
    }
}
