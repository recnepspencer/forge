use core::fmt;

use super::stable_identity::assert_valid_stable_identity;

/// Stable schema identity and version carried by intent definitions.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentSchema {
    stable_identity: &'static str,
    version: u16,
}

impl UiIntentSchema {
    pub const fn stable(stable_identity: &'static str, version: u16) -> Self {
        assert_valid_stable_identity(stable_identity);
        assert!(version > 0, "intent schema version must be nonzero");
        Self {
            stable_identity,
            version,
        }
    }

    pub const fn stable_identity(self) -> &'static str {
        self.stable_identity
    }

    pub const fn version(self) -> u16 {
        self.version
    }
}

impl fmt::Debug for UiIntentSchema {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UiIntentSchema")
            .field("stable_identity", &self.stable_identity)
            .field("version", &self.version)
            .finish()
    }
}

/// Typed product outcomes declare the schema that consequence mapping accepts.
pub trait UiIntentProductOutcome: Send + 'static {
    const SCHEMA: UiIntentSchema;
}
