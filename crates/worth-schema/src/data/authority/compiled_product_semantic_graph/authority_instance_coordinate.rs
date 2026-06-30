use serde::{Deserialize, Serialize};

use super::error::{
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompiledProductAuthorityInstanceCoordinate {
    coordinate_kind: String,
    coordinate_value: String,
}

impl CompiledProductAuthorityInstanceCoordinate {
    pub fn branch_identity(
        coordinate_value: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        Self::named("branch-identity", coordinate_value)
    }

    pub fn snapshot_identity(
        coordinate_value: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        Self::named("snapshot-identity", coordinate_value)
    }

    pub fn stage_receipt_identity(
        coordinate_value: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        Self::named("stage-receipt-identity", coordinate_value)
    }

    pub fn named(
        coordinate_kind: impl Into<String>,
        coordinate_value: impl Into<String>,
    ) -> Result<Self, CompiledProductSemanticGraphVocabularyError> {
        let coordinate_kind = coordinate_kind.into();
        let coordinate_value = coordinate_value.into();
        require_non_blank(
            &coordinate_kind,
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthorityInstanceKind,
            "compiled-product authority instance coordinate requires a named kind",
        )?;
        require_non_blank(
            &coordinate_value,
            CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthorityInstanceValue,
            "compiled-product authority instance coordinate requires a non-empty value",
        )?;
        Ok(Self {
            coordinate_kind,
            coordinate_value,
        })
    }

    pub fn coordinate_kind(&self) -> &str {
        &self.coordinate_kind
    }

    pub fn coordinate_value(&self) -> &str {
        &self.coordinate_value
    }
}

fn require_non_blank(
    value: &str,
    kind: CompiledProductSemanticGraphVocabularyErrorKind,
    detail: &'static str,
) -> Result<(), CompiledProductSemanticGraphVocabularyError> {
    if value.trim().is_empty() {
        Err(CompiledProductSemanticGraphVocabularyError::new(
            kind, detail,
        ))
    } else {
        Ok(())
    }
}
