use std::collections::{BTreeSet, HashSet};
use std::path::Path;

use super::closeout_bypass::{
    closeout_bypass_evidence_from_gate, validate_bypass_evidence_complete,
};
use super::closeout_counters::{closeout_counters, WorthGraphAuthorityCloseoutCounters};
use super::closeout_deletion_evidence::closeout_deletion_class_evidence;
use super::closeout_doc::{validate_closeout_doc, CLOSEOUT_DOC};
use super::closeout_facade::validate_public_facade_evidence;
use super::closeout_types::{
    closeout_row_for_inventory, WorthGraphAuthorityCloseoutBypassClass,
    WorthGraphAuthorityCloseoutBypassEvidence, WorthGraphAuthorityCloseoutMatrixRow,
    WorthGraphAuthorityDeletionClassCloseoutEvidence, WorthGraphAuthorityPublicFacadeProof,
};
use super::gate_report_types::WorthGraphAuthorityGateReport;
use super::report::WorthGraphAuthorityGateViolation;
use super::source_discovery::current_worth_graph_authority_audited_source_paths;
use super::types::{
    WorthGraphAuthorityDeletionTarget, WorthGraphAuthorityRootFamily,
    WorthLowerAuthorityPromotionCase,
};

const CLOSEOUT_DELETION_TARGETS: [WorthGraphAuthorityDeletionTarget; 11] = [
    WorthGraphAuthorityDeletionTarget::DuplicateSupportReport,
    WorthGraphAuthorityDeletionTarget::LocalSupportPinWrapper,
    WorthGraphAuthorityDeletionTarget::BlueprintProofObligation,
    WorthGraphAuthorityDeletionTarget::CeremonyAudit,
    WorthGraphAuthorityDeletionTarget::HandoffOnlyHelper,
    WorthGraphAuthorityDeletionTarget::RawEvidenceScan,
    WorthGraphAuthorityDeletionTarget::CopiedEvidenceRows,
    WorthGraphAuthorityDeletionTarget::StringStageLink,
    WorthGraphAuthorityDeletionTarget::SyntheticFixture,
    WorthGraphAuthorityDeletionTarget::CompatibilityReport,
    WorthGraphAuthorityDeletionTarget::ResidueManifest,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthGraphAuthorityCloseoutViolation {
    Gate(WorthGraphAuthorityGateViolation),
    AuditedSourceCoverageDrift {
        discovered: usize,
        certified: usize,
    },
    MissingInventoryMatrixRow(&'static str),
    MissingDeletionTargetClass(WorthGraphAuthorityDeletionTarget),
    MissingRootFamily(WorthGraphAuthorityRootFamily),
    MissingLowerAuthorityGuard(WorthLowerAuthorityPromotionCase),
    MissingLowerAuthorityFixture(&'static str),
    MissingBypassRejection(WorthGraphAuthorityCloseoutBypassClass),
    PublicFacadeMissing(&'static str),
    PublicFacadeContractMissing(&'static str),
    PublicFacadeContractSymbolMissing {
        source_id: &'static str,
        symbol: String,
    },
    PublicFacadeRootMismatch {
        source_id: &'static str,
        ordinary_api: &'static str,
        expected_prefix: &'static str,
    },
    PublicFacadeApiMismatch {
        source_id: &'static str,
        ordinary_api: &'static str,
        expected_api: &'static str,
    },
    PublicFacadePostureMismatch {
        source_id: &'static str,
        posture_accessor: &'static str,
        expected_accessor: &'static str,
    },
    PublicFacadeProofMissing(WorthGraphAuthorityPublicFacadeProof),
    RawCertifierExposedAsFacade(&'static str),
    CloseoutDocMissingClaim(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityCloseoutReport {
    matrix: Vec<WorthGraphAuthorityCloseoutMatrixRow>,
    rejected_bypass_classes: Vec<WorthGraphAuthorityCloseoutBypassClass>,
    bypass_evidence: Vec<WorthGraphAuthorityCloseoutBypassEvidence>,
    deletion_class_evidence: Vec<WorthGraphAuthorityDeletionClassCloseoutEvidence>,
    counters: WorthGraphAuthorityCloseoutCounters,
}

impl WorthGraphAuthorityCloseoutReport {
    pub fn matrix(&self) -> &[WorthGraphAuthorityCloseoutMatrixRow] {
        &self.matrix
    }

    pub fn rejected_bypass_classes(&self) -> &[WorthGraphAuthorityCloseoutBypassClass] {
        &self.rejected_bypass_classes
    }

    pub fn bypass_evidence(&self) -> &[WorthGraphAuthorityCloseoutBypassEvidence] {
        &self.bypass_evidence
    }

    pub fn deletion_class_evidence(&self) -> &[WorthGraphAuthorityDeletionClassCloseoutEvidence] {
        &self.deletion_class_evidence
    }

    pub fn counters(&self) -> &WorthGraphAuthorityCloseoutCounters {
        &self.counters
    }
}

pub fn current_worth_graph_authority_closeout_report(
) -> Result<WorthGraphAuthorityCloseoutReport, WorthGraphAuthorityCloseoutViolation> {
    let gate = super::current_worth_graph_authority_gate_report()
        .map_err(WorthGraphAuthorityCloseoutViolation::Gate)?;
    certify_worth_graph_authority_closeout(&gate, CLOSEOUT_DOC)
}

pub(crate) fn certify_worth_graph_authority_closeout(
    gate: &WorthGraphAuthorityGateReport,
    closeout_doc: &str,
) -> Result<WorthGraphAuthorityCloseoutReport, WorthGraphAuthorityCloseoutViolation> {
    let audited_source_paths = current_worth_graph_authority_audited_source_paths();
    let discovered_sources = audited_source_paths.len();
    let certified_sources = gate.counters().audited_sources();
    if discovered_sources != certified_sources {
        return Err(
            WorthGraphAuthorityCloseoutViolation::AuditedSourceCoverageDrift {
                discovered: discovered_sources,
                certified: certified_sources,
            },
        );
    }

    let matrix: Vec<_> = gate
        .inventory()
        .iter()
        .map(closeout_row_for_inventory)
        .collect();

    validate_matrix_covers_inventory(gate, &matrix)?;
    validate_root_families(gate)?;
    validate_deletion_target_classes(gate)?;
    validate_lower_authority_guards(gate)?;
    validate_public_facade_evidence(&matrix)?;
    let bypass_evidence = closeout_bypass_evidence_from_gate(gate)?;
    let deletion_class_evidence =
        closeout_deletion_class_evidence(gate, &audited_source_paths, &CLOSEOUT_DELETION_TARGETS)?;

    certify_worth_graph_authority_closeout_with_evidence(
        gate,
        matrix,
        bypass_evidence,
        deletion_class_evidence,
        closeout_doc,
    )
}

pub(crate) fn certify_worth_graph_authority_closeout_with_evidence(
    gate: &WorthGraphAuthorityGateReport,
    matrix: Vec<WorthGraphAuthorityCloseoutMatrixRow>,
    bypass_evidence: Vec<WorthGraphAuthorityCloseoutBypassEvidence>,
    deletion_class_evidence: Vec<WorthGraphAuthorityDeletionClassCloseoutEvidence>,
    closeout_doc: &str,
) -> Result<WorthGraphAuthorityCloseoutReport, WorthGraphAuthorityCloseoutViolation> {
    validate_public_facade_evidence(&matrix)?;
    validate_bypass_evidence_complete(&bypass_evidence)?;
    let counters = closeout_counters(gate, &matrix, &bypass_evidence, &deletion_class_evidence);
    validate_closeout_doc(&counters, &deletion_class_evidence, closeout_doc)?;
    let rejected_bypass_classes = bypass_evidence
        .iter()
        .map(|evidence| evidence.bypass_class())
        .collect();

    Ok(WorthGraphAuthorityCloseoutReport {
        matrix,
        rejected_bypass_classes,
        bypass_evidence,
        deletion_class_evidence,
        counters,
    })
}

fn validate_matrix_covers_inventory(
    gate: &WorthGraphAuthorityGateReport,
    matrix: &[WorthGraphAuthorityCloseoutMatrixRow],
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let rows: BTreeSet<_> = matrix.iter().map(|row| row.source_id()).collect();
    for inventory_row in gate.inventory() {
        if !rows.contains(inventory_row.source_id()) {
            return Err(
                WorthGraphAuthorityCloseoutViolation::MissingInventoryMatrixRow(
                    inventory_row.source_id(),
                ),
            );
        }
    }
    Ok(())
}

fn validate_root_families(
    gate: &WorthGraphAuthorityGateReport,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let families: HashSet<_> = gate
        .discovery_records()
        .iter()
        .map(|row| row.root_family())
        .collect();
    for family in WorthGraphAuthorityRootFamily::ALL {
        if !families.contains(&family) {
            return Err(WorthGraphAuthorityCloseoutViolation::MissingRootFamily(
                family,
            ));
        }
    }
    Ok(())
}

fn validate_deletion_target_classes(
    gate: &WorthGraphAuthorityGateReport,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let classes: HashSet<_> = gate
        .deletion_ledger()
        .iter()
        .map(|row| row.deletion_target())
        .filter(|target| *target != WorthGraphAuthorityDeletionTarget::None)
        .collect();
    for target in CLOSEOUT_DELETION_TARGETS {
        if !classes.contains(&target) {
            return Err(WorthGraphAuthorityCloseoutViolation::MissingDeletionTargetClass(target));
        }
    }
    Ok(())
}

fn validate_lower_authority_guards(
    gate: &WorthGraphAuthorityGateReport,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let cases: HashSet<_> = gate
        .lower_authority_guard_plan()
        .iter()
        .map(|plan| plan.promotion_case())
        .collect();
    for case in WorthLowerAuthorityPromotionCase::ALL {
        if !cases.contains(&case) {
            return Err(WorthGraphAuthorityCloseoutViolation::MissingLowerAuthorityGuard(case));
        }
    }
    for plan in gate.lower_authority_guard_plan() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(plan.planned_compile_fail_path());
        if !path.is_file() {
            return Err(
                WorthGraphAuthorityCloseoutViolation::MissingLowerAuthorityFixture(
                    plan.planned_compile_fail_path(),
                ),
            );
        }
    }
    Ok(())
}
