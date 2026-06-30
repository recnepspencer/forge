use std::collections::BTreeSet;

use topology::derived_invalidation_milestone_ten_closeout::DerivedInvalidationMilestoneElevenSeed;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::catalog::current_family_declarations;
use super::declaration::EvidenceLookupFamilyDeclaration;
use super::error::{EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyCatalogErrorKind};
use super::selection::EvidenceLookupFamilyStageSelection;
use super::stage_receipt_identity::EvidenceLookupStageReceiptFamilyIdentity;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupFamilyCatalogCounters {
    family_count: usize,
    query_required_family_count: usize,
    topology_required_family_count: usize,
    sparse_index_family_count: usize,
    bounded_dense_index_family_count: usize,
    diagnostic_witness_family_count: usize,
    declare_once_multi_stage_family_count: usize,
    source_inventory_migrate_row_count: usize,
}

impl EvidenceLookupFamilyCatalogCounters {
    fn from_declarations(declarations: &[EvidenceLookupFamilyDeclaration]) -> Self {
        let mut counters = Self {
            family_count: declarations.len(),
            ..Self::default()
        };
        for declaration in declarations {
            counters.count_declaration(declaration);
        }
        counters
    }

    fn count_declaration(&mut self, declaration: &EvidenceLookupFamilyDeclaration) {
        if declaration.query_posture().requires_query_evidence() {
            self.query_required_family_count += 1;
        }
        if declaration
            .topology_input_posture()
            .requires_topology_receipt()
        {
            self.topology_required_family_count += 1;
        }
        match declaration.index_posture().kind() {
            super::posture::EvidenceLookupFamilyIndexPostureKind::SparseLookupPlanRequired => {
                self.sparse_index_family_count += 1;
            }
            super::posture::EvidenceLookupFamilyIndexPostureKind::BoundedDenseLookupPlanRequired => {
                self.bounded_dense_index_family_count += 1;
            }
            super::posture::EvidenceLookupFamilyIndexPostureKind::IndexNotRequiredForDeclarationOnly => {}
        }
        self.diagnostic_witness_family_count += 1;
        if declaration
            .stage_applicability()
            .declares_multiple_matching_stages()
        {
            self.declare_once_multi_stage_family_count += 1;
        }
        self.source_inventory_migrate_row_count = self
            .source_inventory_migrate_row_count
            .max(declaration.source_inventory_pressure().migrate_row_count());
    }

    pub const fn family_count(&self) -> usize {
        self.family_count
    }

    pub const fn query_required_family_count(&self) -> usize {
        self.query_required_family_count
    }

    pub const fn topology_required_family_count(&self) -> usize {
        self.topology_required_family_count
    }

    pub const fn sparse_index_family_count(&self) -> usize {
        self.sparse_index_family_count
    }

    pub const fn bounded_dense_index_family_count(&self) -> usize {
        self.bounded_dense_index_family_count
    }

    pub const fn diagnostic_witness_family_count(&self) -> usize {
        self.diagnostic_witness_family_count
    }

    pub const fn declare_once_multi_stage_family_count(&self) -> usize {
        self.declare_once_multi_stage_family_count
    }

    pub const fn source_inventory_migrate_row_count(&self) -> usize {
        self.source_inventory_migrate_row_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyCatalogCloseout {
    declarations: Vec<EvidenceLookupFamilyDeclaration>,
    counters: EvidenceLookupFamilyCatalogCounters,
    catalog_digest: String,
}

impl EvidenceLookupFamilyCatalogCloseout {
    pub(crate) fn from_declarations(
        declarations: Vec<EvidenceLookupFamilyDeclaration>,
    ) -> Result<Self, EvidenceLookupFamilyCatalogError> {
        if declarations.is_empty() {
            return Err(EvidenceLookupFamilyCatalogError::new(
                EvidenceLookupFamilyCatalogErrorKind::EmptyCatalog,
            ));
        }
        reject_duplicate_family_identities(&declarations)?;
        let counters = EvidenceLookupFamilyCatalogCounters::from_declarations(&declarations);
        let catalog_digest = catalog_digest(&declarations, &counters);
        Ok(Self {
            declarations,
            counters,
            catalog_digest,
        })
    }

    pub fn declarations(&self) -> &[EvidenceLookupFamilyDeclaration] {
        &self.declarations
    }

    pub const fn counters(&self) -> &EvidenceLookupFamilyCatalogCounters {
        &self.counters
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn family_by_identity(&self, identity: &str) -> Option<&EvidenceLookupFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity().as_str() == identity)
    }

    pub fn families_for_stage(
        &self,
        stage: WorkloadEvidenceStage,
        receipt_family: &EvidenceLookupStageReceiptFamilyIdentity,
    ) -> EvidenceLookupFamilyStageSelection {
        EvidenceLookupFamilyStageSelection::select(&self.declarations, stage, receipt_family)
    }

    pub fn validate_topology_requirements_against_seed(
        &self,
        seed: &DerivedInvalidationMilestoneElevenSeed,
    ) -> Result<EvidenceLookupTopologyRequirementReport, EvidenceLookupFamilyCatalogError> {
        EvidenceLookupTopologyRequirementReport::from_catalog_and_seed(&self.declarations, seed)
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }

    pub const fn claims_family_selection(&self) -> bool {
        false
    }

    pub const fn claims_index_construction(&self) -> bool {
        false
    }

    pub const fn claims_query_support_authority(&self) -> bool {
        false
    }
}

pub fn current_evidence_lookup_family_catalog(
) -> Result<EvidenceLookupFamilyCatalogCloseout, EvidenceLookupFamilyCatalogError> {
    EvidenceLookupFamilyCatalogCloseout::from_declarations(current_family_declarations()?)
}

fn reject_duplicate_family_identities(
    declarations: &[EvidenceLookupFamilyDeclaration],
) -> Result<(), EvidenceLookupFamilyCatalogError> {
    let mut identities = BTreeSet::new();
    for declaration in declarations {
        if !identities.insert(declaration.identity().digest().to_string()) {
            return Err(EvidenceLookupFamilyCatalogError::new(
                EvidenceLookupFamilyCatalogErrorKind::DuplicateFamilyIdentity,
            ));
        }
    }
    Ok(())
}

fn catalog_digest(
    declarations: &[EvidenceLookupFamilyDeclaration],
    counters: &EvidenceLookupFamilyCatalogCounters,
) -> String {
    let mut parts = vec![
        "worth-spatial:evidence-lookup-family-catalog-closeout:v1".to_string(),
        format!("families:{}", counters.family_count()),
        format!("query-required:{}", counters.query_required_family_count()),
        format!(
            "topology-required:{}",
            counters.topology_required_family_count()
        ),
        format!(
            "declare-once-multi-stage:{}",
            counters.declare_once_multi_stage_family_count()
        ),
        format!(
            "source-inventory-migrate:{}",
            counters.source_inventory_migrate_row_count()
        ),
    ];
    parts.extend(declarations.iter().map(|declaration| {
        format!(
            "family:{}:{}",
            declaration.identity().digest(),
            declaration.declaration_digest()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupTopologyRequirementReport {
    required_family_count: usize,
    seed_receipt_count: usize,
}

impl EvidenceLookupTopologyRequirementReport {
    fn from_catalog_and_seed(
        declarations: &[EvidenceLookupFamilyDeclaration],
        seed: &DerivedInvalidationMilestoneElevenSeed,
    ) -> Result<Self, EvidenceLookupFamilyCatalogError> {
        let seed_receipts = seed.topology_derived_product_receipts();
        let mut required_family_count = 0;
        for declaration in declarations {
            let Some(required_family) = declaration.topology_input_posture().required_family()
            else {
                continue;
            };
            required_family_count += 1;
            if !seed_receipts
                .iter()
                .any(|receipt| receipt.family_identity() == required_family)
            {
                return Err(EvidenceLookupFamilyCatalogError::new(
                    EvidenceLookupFamilyCatalogErrorKind::MissingRequiredTopologyReceipt,
                ));
            }
        }
        Ok(Self {
            required_family_count,
            seed_receipt_count: seed_receipts.len(),
        })
    }

    pub const fn required_family_count(&self) -> usize {
        self.required_family_count
    }

    pub const fn seed_receipt_count(&self) -> usize {
        self.seed_receipt_count
    }
}
