use super::error::WorkloadCatalogError;
use super::query::{query_backed_catalog_declaration, query_backed_catalog_support};
use super::recipe_kind::{WorkloadCatalogRecipeKind, WorkloadCatalogSupportPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorkloadCatalogSupportDecision {
    posture: WorkloadCatalogSupportPosture,
    human_reason: String,
}

impl WorkloadCatalogSupportDecision {
    pub(crate) fn admitted(human_reason: String) -> Self {
        Self {
            posture: WorkloadCatalogSupportPosture::Admitted,
            human_reason,
        }
    }

    pub(crate) fn unsupported(human_reason: String) -> Self {
        Self {
            posture: WorkloadCatalogSupportPosture::Unsupported,
            human_reason,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogDeclarationReceipt {
    recipe: WorkloadCatalogRecipeKind,
    declaration: String,
    query_declaration_digest: String,
    query_envelope_digest: String,
    query_handle_digest: String,
}

impl WorkloadCatalogDeclarationReceipt {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: &str,
    ) -> Result<Self, WorkloadCatalogError> {
        let query_receipt = query_backed_catalog_declaration(recipe, declaration)?;
        Ok(Self {
            recipe,
            declaration: declaration.to_string(),
            query_declaration_digest: query_receipt.declaration_digest().to_string(),
            query_envelope_digest: query_receipt.envelope_digest().to_string(),
            query_handle_digest: query_receipt.handle_digest().to_string(),
        })
    }

    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn query_envelope_digest(&self) -> &str {
        &self.query_envelope_digest
    }

    pub fn query_handle_digest(&self) -> &str {
        &self.query_handle_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogSupportReceipt {
    recipe: WorkloadCatalogRecipeKind,
    posture: WorkloadCatalogSupportPosture,
    query_support_digest: String,
    human_reason: String,
}

impl WorkloadCatalogSupportReceipt {
    pub(crate) fn new(
        declaration: &WorkloadCatalogDeclarationReceipt,
        decision: WorkloadCatalogSupportDecision,
    ) -> Result<Self, WorkloadCatalogError> {
        let query_receipt = query_backed_catalog_support(
            declaration.recipe(),
            declaration.declaration(),
            decision.posture,
            declaration.query_declaration_digest(),
        )?;
        Ok(Self {
            recipe: declaration.recipe(),
            posture: decision.posture,
            query_support_digest: query_receipt.declaration_digest().to_string(),
            human_reason: decision.human_reason,
        })
    }

    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn posture(&self) -> WorkloadCatalogSupportPosture {
        self.posture
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
