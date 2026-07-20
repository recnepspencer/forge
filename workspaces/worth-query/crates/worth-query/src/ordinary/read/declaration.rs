use super::WorthQueryDeclaredReadIntent;
use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial};

/// Stable identity for one canonical read declaration.
///
/// This is an observation of Query-owned canonical meaning, not execution or
/// basis authority.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryReadDeclarationIdentity {
    declaration_digest: String,
    canonical_query_digest: String,
    canonical_result_shape_digest: String,
}

impl WorthQueryReadDeclarationIdentity {
    pub fn as_str(&self) -> &str {
        &self.declaration_digest
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.canonical_query_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &str {
        &self.canonical_result_shape_digest
    }

    fn from_declared_intent(intent: &WorthQueryDeclaredReadIntent) -> Self {
        Self {
            declaration_digest: intent.digest().to_string(),
            canonical_query_digest: intent.canonical_query_digest().as_str().to_string(),
            canonical_result_shape_digest: intent
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
        }
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

    pub(crate) fn clone_for_installed_execution(&self) -> Self {
        Self {
            identity: self.identity.clone(),
            intent: self.intent.clone(),
        }
    }

    /// Split one declaration into the two single-use observations required by
    /// Query-owned comparison execution.
    ///
    /// This is deliberately crate-private: ordinary consumers still receive a
    /// move-only declaration, while the comparison owner can prove that the
    /// same canonical meaning must be observed against two distinct bases.
    pub(crate) fn into_comparison_pair(self) -> (Self, Self) {
        let second = Self {
            identity: self.identity.clone(),
            intent: self.intent.clone(),
        };
        (self, second)
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
