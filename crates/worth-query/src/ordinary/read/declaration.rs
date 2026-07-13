use super::WorthQueryDeclaredReadIntent;
use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial};

/// Stable identity for one canonical read declaration.
///
/// This is an observation of Query-owned canonical meaning, not execution or
/// basis authority.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryReadDeclarationIdentity(String);

impl WorthQueryReadDeclarationIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_declared_intent(intent: &WorthQueryDeclaredReadIntent) -> Self {
        Self(intent.digest().to_string())
    }
}

/// A Query-minted, canonical one-shot read declaration.
///
/// Its fields are private so consumers can inspect canonical identity without
/// manufacturing or replacing the graph Query admitted during authoring.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryReadDeclaration {
    identity: WorthQueryReadDeclarationIdentity,
    intent: WorthQueryDeclaredReadIntent,
}

impl WorthQueryReadDeclaration {
    pub fn identity(&self) -> &WorthQueryReadDeclarationIdentity {
        &self.identity
    }

    pub(crate) fn into_declared_intent(self) -> WorthQueryDeclaredReadIntent {
        self.intent
    }

    fn from_declared_intent(intent: WorthQueryDeclaredReadIntent) -> Self {
        let identity = WorthQueryReadDeclarationIdentity::from_declared_intent(&intent);
        Self { identity, intent }
    }
}

/// Authoring stop produced before runtime admission or lower-runtime contact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadDeclarationStop {
    denial: WorthQueryReadDenial,
}

impl WorthQueryReadDeclarationStop {
    pub fn denial(&self) -> &WorthQueryReadDenial {
        &self.denial
    }

    pub fn next_action(&self) -> super::WorthQueryReadNextAction {
        super::WorthQueryReadNextAction::ReviseDeclaration
    }

    fn new(denial: WorthQueryReadDenial) -> Self {
        Self { denial }
    }
}

/// Declare one bounded read capability through Query's canonical authoring
/// path.
///
/// The closure receives only read-family vocabulary. Canonicalization,
/// and validation occur inside the selected read operation. Planning remains
/// mechanically unavailable until the declaration is paired with an admitted
/// authority context.
pub fn declare(
    author: impl FnOnce(
        WorthQueryReadBuilder<WorthQueryDeclaredReadIntent>,
    ) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial>,
) -> Result<WorthQueryReadDeclaration, WorthQueryReadDeclarationStop> {
    author(WorthQueryReadBuilder::declaration())
        .map(WorthQueryReadDeclaration::from_declared_intent)
        .map_err(WorthQueryReadDeclarationStop::new)
}
