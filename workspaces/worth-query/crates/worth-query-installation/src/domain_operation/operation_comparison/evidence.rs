use worth_foundational::facade::{
    CanonicalEquivalentBasis, CanonicalExportManifestMismatch, CanonicalMismatchBasis,
};

use super::super::WorthQueryOperationConditionalComparisonEquivalent;
use super::WorthQueryPortableOperationDimension;

#[derive(Debug)]
pub enum WorthQueryPortableOperationComparisonOutcome {
    Equivalent(WorthQueryPortableOperationComparisonEquivalent),
    Mismatched(WorthQueryPortableOperationComparisonMismatch),
    Unsupported(WorthQueryPortableOperationComparisonUnsupported),
}

#[derive(Debug)]
pub struct WorthQueryPortableOperationComparisonEquivalent {
    _evidence: EquivalentEvidence,
    work: WorthQueryPortableOperationComparisonWork,
}

impl WorthQueryPortableOperationComparisonEquivalent {
    pub(super) fn new(
        evidence: EquivalentEvidence,
        work: WorthQueryPortableOperationComparisonWork,
    ) -> Self {
        Self {
            _evidence: evidence,
            work,
        }
    }

    pub const fn work(&self) -> WorthQueryPortableOperationComparisonWork {
        self.work
    }
}

#[derive(Debug)]
pub(super) struct EquivalentEvidence {
    _native_contract: CanonicalEquivalentBasis,
    _native_mask: CanonicalEquivalentBasis,
    _conditional: WorthQueryOperationConditionalComparisonEquivalent,
}

impl EquivalentEvidence {
    pub(super) fn new(
        native_contract: CanonicalEquivalentBasis,
        native_mask: CanonicalEquivalentBasis,
        conditional: WorthQueryOperationConditionalComparisonEquivalent,
    ) -> Self {
        Self {
            _native_contract: native_contract,
            _native_mask: native_mask,
            _conditional: conditional,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableOperationComparisonMismatchCategory {
    Foundational,
    FoundationalExportManifest,
    DeclarationOwner,
    InstallationOwner,
}

#[derive(Debug)]
pub struct WorthQueryPortableOperationComparisonMismatch {
    dimension: WorthQueryPortableOperationDimension,
    category: WorthQueryPortableOperationComparisonMismatchCategory,
    foundational: Option<CanonicalMismatchBasis>,
    export_manifest: Option<CanonicalExportManifestMismatch>,
    work: WorthQueryPortableOperationComparisonWork,
}

impl WorthQueryPortableOperationComparisonMismatch {
    pub fn dimension(&self) -> &WorthQueryPortableOperationDimension {
        &self.dimension
    }

    pub const fn category(&self) -> WorthQueryPortableOperationComparisonMismatchCategory {
        self.category
    }

    pub fn foundational_basis(&self) -> Option<&CanonicalMismatchBasis> {
        self.foundational.as_ref()
    }

    pub fn export_manifest_mismatch(&self) -> Option<&CanonicalExportManifestMismatch> {
        self.export_manifest.as_ref()
    }

    pub const fn work(&self) -> WorthQueryPortableOperationComparisonWork {
        self.work
    }
}

#[derive(Debug)]
pub struct WorthQueryPortableOperationComparisonUnsupported {
    dimension: WorthQueryPortableOperationDimension,
    foundational: CanonicalMismatchBasis,
    work: WorthQueryPortableOperationComparisonWork,
}

impl WorthQueryPortableOperationComparisonUnsupported {
    pub fn dimension(&self) -> &WorthQueryPortableOperationDimension {
        &self.dimension
    }

    pub fn foundational_basis(&self) -> &CanonicalMismatchBasis {
        &self.foundational
    }

    pub const fn work(&self) -> WorthQueryPortableOperationComparisonWork {
        self.work
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryPortableOperationComparisonWork {
    owner_dimensions_inspected: u32,
    variable_items_submitted: u32,
    direct_foundational_comparison_requests: u32,
    canonical_export_comparison_requests: u32,
    conditional_owner_comparison_requests: u32,
    delegated_conditional_foundational_comparison_requests: u32,
    subject_conditional_nodes_submitted: u32,
    candidate_conditional_nodes_submitted: u32,
}

impl WorthQueryPortableOperationComparisonWork {
    pub const fn owner_dimensions_inspected(self) -> u32 {
        self.owner_dimensions_inspected
    }

    pub const fn direct_foundational_comparison_requests(self) -> u32 {
        self.direct_foundational_comparison_requests
    }

    /// Variable-width semantic items handed to comparisons that were reached.
    ///
    /// This is an input-width counter, not a claim about short-circuit behavior
    /// inside an owner's equality implementation.
    pub const fn variable_items_submitted(self) -> u32 {
        self.variable_items_submitted
    }

    pub const fn canonical_export_comparison_requests(self) -> u32 {
        self.canonical_export_comparison_requests
    }

    pub const fn conditional_owner_comparison_requests(self) -> u32 {
        self.conditional_owner_comparison_requests
    }

    pub const fn delegated_conditional_foundational_comparison_requests(self) -> u32 {
        self.delegated_conditional_foundational_comparison_requests
    }

    pub const fn subject_conditional_nodes_submitted(self) -> u32 {
        self.subject_conditional_nodes_submitted
    }

    pub const fn candidate_conditional_nodes_submitted(self) -> u32 {
        self.candidate_conditional_nodes_submitted
    }

    pub(super) fn inspect_owner_dimension(&mut self) {
        self.owner_dimensions_inspected = self.owner_dimensions_inspected.saturating_add(1);
    }

    pub(super) fn submit_variable_items(&mut self, count: usize) {
        self.variable_items_submitted = self
            .variable_items_submitted
            .saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
    }

    pub(super) fn inspect_foundational_dimension(&mut self) {
        self.inspect_owner_dimension();
        self.direct_foundational_comparison_requests = self
            .direct_foundational_comparison_requests
            .saturating_add(1);
    }

    pub(super) fn inspect_canonical_export(&mut self) {
        self.inspect_owner_dimension();
        self.canonical_export_comparison_requests =
            self.canonical_export_comparison_requests.saturating_add(1);
    }

    pub(super) fn submit_conditional_inventory(
        &mut self,
        left: &super::super::WorthQueryPortableDomainOperationDefinition,
        right: &super::super::WorthQueryPortableDomainOperationDefinition,
    ) {
        self.conditional_owner_comparison_requests =
            self.conditional_owner_comparison_requests.saturating_add(1);
        self.subject_conditional_nodes_submitted = conditional_width(left);
        self.candidate_conditional_nodes_submitted = conditional_width(right);
    }

    pub(super) fn record_conditional_comparisons(&mut self, comparison_count: u32) {
        self.delegated_conditional_foundational_comparison_requests = comparison_count;
    }
}

fn conditional_width(
    definition: &super::super::WorthQueryPortableDomainOperationDefinition,
) -> u32 {
    let semantics = definition.semantics();
    let stage_width = match &semantics.workflow {
        super::super::WorthQueryOperationWorkflowContract::NotRequired => 0,
        super::super::WorthQueryOperationWorkflowContract::Declared(workflow) => workflow
            .stages()
            .iter()
            .map(|stage| stage.semantics().conditional_nodes.len() as u32)
            .sum(),
    };
    (semantics.conditional_nodes.len() as u32).saturating_add(stage_width)
}

pub(super) enum MismatchEvidence {
    Foundational {
        dimension: WorthQueryPortableOperationDimension,
        basis: CanonicalMismatchBasis,
        unsupported: bool,
    },
    ExportManifest {
        dimension: WorthQueryPortableOperationDimension,
        mismatch: CanonicalExportManifestMismatch,
    },
    Owner {
        dimension: WorthQueryPortableOperationDimension,
        category: WorthQueryPortableOperationComparisonMismatchCategory,
    },
}

impl MismatchEvidence {
    pub(super) fn foundational(
        dimension: WorthQueryPortableOperationDimension,
        basis: CanonicalMismatchBasis,
    ) -> Self {
        Self::Foundational {
            dimension,
            basis,
            unsupported: false,
        }
    }

    pub(super) fn unsupported(
        dimension: WorthQueryPortableOperationDimension,
        basis: CanonicalMismatchBasis,
    ) -> Self {
        Self::Foundational {
            dimension,
            basis,
            unsupported: true,
        }
    }

    pub(super) fn export_manifest(
        dimension: WorthQueryPortableOperationDimension,
        mismatch: CanonicalExportManifestMismatch,
    ) -> Self {
        Self::ExportManifest {
            dimension,
            mismatch,
        }
    }

    pub(super) fn installation_owner(dimension: WorthQueryPortableOperationDimension) -> Self {
        Self::Owner {
            dimension,
            category: WorthQueryPortableOperationComparisonMismatchCategory::InstallationOwner,
        }
    }

    pub(super) fn declaration_owner(dimension: WorthQueryPortableOperationDimension) -> Self {
        Self::Owner {
            dimension,
            category: WorthQueryPortableOperationComparisonMismatchCategory::DeclarationOwner,
        }
    }

    pub(super) fn into_outcome(
        self,
        work: WorthQueryPortableOperationComparisonWork,
    ) -> WorthQueryPortableOperationComparisonOutcome {
        match self {
            Self::Foundational {
                dimension,
                basis,
                unsupported: true,
            } => WorthQueryPortableOperationComparisonOutcome::Unsupported(
                WorthQueryPortableOperationComparisonUnsupported {
                    dimension,
                    foundational: basis,
                    work,
                },
            ),
            Self::Foundational {
                dimension,
                basis,
                unsupported: false,
            } => WorthQueryPortableOperationComparisonOutcome::Mismatched(
                WorthQueryPortableOperationComparisonMismatch {
                    dimension,
                    category: WorthQueryPortableOperationComparisonMismatchCategory::Foundational,
                    foundational: Some(basis),
                    export_manifest: None,
                    work,
                },
            ),
            Self::ExportManifest {
                dimension,
                mismatch,
            } => WorthQueryPortableOperationComparisonOutcome::Mismatched(
                WorthQueryPortableOperationComparisonMismatch {
                    dimension,
                    category:
                        WorthQueryPortableOperationComparisonMismatchCategory::FoundationalExportManifest,
                    foundational: None,
                    export_manifest: Some(mismatch),
                    work,
                },
            ),
            Self::Owner {
                dimension,
                category,
            } => WorthQueryPortableOperationComparisonOutcome::Mismatched(
                WorthQueryPortableOperationComparisonMismatch {
                    dimension,
                    category,
                    foundational: None,
                    export_manifest: None,
                    work,
                },
            ),
        }
    }
}
