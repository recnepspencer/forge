use std::sync::Arc;

/// Opaque WORTH UI reference to one exact Query-owned operation binding.
///
/// Query currently defines the canonical binding identity representation. This
/// reference lets WORTH UI preserve and compare that identity without exposing
/// a string that callers could reinterpret or reassemble.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiQueryBindingReference {
    identity: Arc<str>,
}

impl UiQueryBindingReference {
    pub(crate) fn query_issued(identity: &str) -> Self {
        Self {
            identity: Arc::from(identity),
        }
    }
}

impl std::fmt::Debug for UiQueryBindingReference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiQueryBindingReference")
            .field("authority", &"sealed Query binding")
            .finish()
    }
}
