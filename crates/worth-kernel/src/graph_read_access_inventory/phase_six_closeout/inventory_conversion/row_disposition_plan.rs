use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

use super::super::super::candidates::{
    WorthGraphReadDeclarationCandidate, WorthGraphReadReadFamilyTarget,
    WorthGraphReadRequirementVocabulary,
};
use super::super::super::capability_gaps::{
    WorthGraphReadExpectedDenial, WorthGraphReadMissingQueryCapability,
    WorthGraphReadQueryAccessCapabilityGap,
};
use super::super::super::deletion_ledger::WorthGraphReadDeletionLedgerItem;
use super::super::super::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessInventoryRow, WorthGraphReadAccessScopeFamily,
};
use super::super::errors::{
    WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind,
};

pub(crate) enum WorthGraphReadAccessPhaseSixPlannedDisposition {
    DeclarationCandidate(WorthGraphReadDeclarationCandidate),
    CapabilityGap(WorthGraphReadQueryAccessCapabilityGap),
    DeletionItem(WorthGraphReadDeletionLedgerItem),
    Excluded,
}

pub(crate) fn plan_phase_six_disposition_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> Result<WorthGraphReadAccessPhaseSixPlannedDisposition, WorthGraphReadAccessPhaseSixError> {
    match row.classification() {
        WorthGraphReadAccessClassification::QueryDeclarationCandidate => {
            declaration_candidate_for_row(row)
                .map(WorthGraphReadAccessPhaseSixPlannedDisposition::DeclarationCandidate)
        }
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap
        | WorthGraphReadAccessClassification::CappedResidue => capability_gap_for_row(row)
            .map(WorthGraphReadAccessPhaseSixPlannedDisposition::CapabilityGap),
        WorthGraphReadAccessClassification::DeletionTarget => deletion_item_for_row(row)
            .map(WorthGraphReadAccessPhaseSixPlannedDisposition::DeletionItem),
        WorthGraphReadAccessClassification::CertificationOnlySupport
        | WorthGraphReadAccessClassification::OutOfScopeNonGraphRead => {
            Ok(WorthGraphReadAccessPhaseSixPlannedDisposition::Excluded)
        }
    }
}

fn declaration_candidate_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> Result<WorthGraphReadDeclarationCandidate, WorthGraphReadAccessPhaseSixError> {
    WorthGraphReadDeclarationCandidate::for_inventory_row(row)
        .read_family_target(read_family_target_for_row(row)?)
        .touched_authority_input(touched_authority_input_for_row(row)?)
        .requirement_vocabulary(requirement_vocabulary_for_row(row))
        .milestone_seven_lowering_target(milestone_seven_lowering_target_for_row(row))
        .build()
}

fn capability_gap_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> Result<WorthGraphReadQueryAccessCapabilityGap, WorthGraphReadAccessPhaseSixError> {
    let residue = row.capped_residue();
    WorthGraphReadQueryAccessCapabilityGap::for_inventory_row(row)
        .missing_capability(missing_query_capability_for_row(row))
        .expected_denial(expected_denial_for_row(row))
        .must_not_exceed_count(
            residue
                .map(|residue| residue.must_not_exceed_count())
                .unwrap_or(1),
        )
        .blocker(
            residue
                .map(|residue| residue.blocker().to_string())
                .unwrap_or_else(|| query_gap_blocker_for_row(row)),
        )
        .removal_trigger(
            residue
                .map(|residue| residue.removal_trigger().to_string())
                .unwrap_or_else(|| query_gap_removal_trigger_for_row(row)),
        )
        .build()
}

fn deletion_item_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> Result<WorthGraphReadDeletionLedgerItem, WorthGraphReadAccessPhaseSixError> {
    WorthGraphReadDeletionLedgerItem::for_inventory_row(row)
        .deletion_trigger(deletion_trigger_for_row(row))
        .blocker("Phase 6 graph-read inventory closeout must own replacement seed first")
        .build()
}

fn read_family_target_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> Result<WorthGraphReadReadFamilyTarget, WorthGraphReadAccessPhaseSixError> {
    match row.scope_binding().scope_family() {
        WorthGraphReadAccessScopeFamily::TopologyReadLedger => {
            Ok(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
        }
        WorthGraphReadAccessScopeFamily::TopologyRuntimeReadExecution => {
            Ok(WorthGraphReadReadFamilyTarget::TopologyLocalRewireNeighborhood)
        }
        WorthGraphReadAccessScopeFamily::KernelWorkloadComposition => {
            Ok(WorthGraphReadReadFamilyTarget::TopologyHalfEdgeSharedVertexNeighborhood)
        }
        WorthGraphReadAccessScopeFamily::KernelBindingNeighborhood => {
            Ok(WorthGraphReadReadFamilyTarget::TopologyLocalRewireNeighborhood)
        }
        WorthGraphReadAccessScopeFamily::SpatialEvidenceLookup
        | WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation => {
            Ok(WorthGraphReadReadFamilyTarget::SpatialPlanarBooleanContinuationIndex)
        }
        _ => Err(error(
            WorthGraphReadAccessPhaseSixErrorKind::MissingReadFamilyTarget,
        )),
    }
}

fn touched_authority_input_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> Result<String, WorthGraphReadAccessPhaseSixError> {
    row.scope_binding()
        .authority_digest()
        .map(str::to_string)
        .ok_or_else(|| error(WorthGraphReadAccessPhaseSixErrorKind::MissingTouchedAuthorityInput))
}

fn requirement_vocabulary_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> WorthGraphReadRequirementVocabulary {
    match row.cost_posture() {
        WorthGraphReadAccessCostPosture::BroadScan
        | WorthGraphReadAccessCostPosture::FrontierOrVisitedSet => {
            WorthGraphReadRequirementVocabulary::predicate_filtered_relation()
        }
        _ => WorthGraphReadRequirementVocabulary::relation_frontier(),
    }
}

fn missing_query_capability_for_row(
    row: &WorthGraphReadAccessInventoryRow,
) -> WorthGraphReadMissingQueryCapability {
    match row.cost_posture() {
        WorthGraphReadAccessCostPosture::BroadScan
        | WorthGraphReadAccessCostPosture::FrontierOrVisitedSet => {
            WorthGraphReadMissingQueryCapability::PersistentContinuationIndex
        }
        WorthGraphReadAccessCostPosture::AdjacencyMapMaterialization
        | WorthGraphReadAccessCostPosture::LocalCache => {
            WorthGraphReadMissingQueryCapability::StoreBackedGraphIndex
        }
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow => {
            WorthGraphReadMissingQueryCapability::DomainOperationRegistration
        }
        _ => WorthGraphReadMissingQueryCapability::AsyncMaterializedGraphRead,
    }
}

fn expected_denial_for_row(row: &WorthGraphReadAccessInventoryRow) -> WorthGraphReadExpectedDenial {
    match missing_query_capability_for_row(row) {
        WorthGraphReadMissingQueryCapability::PersistentContinuationIndex => {
            WorthGraphReadExpectedDenial::new(
                ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex,
                ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
            )
        }
        WorthGraphReadMissingQueryCapability::AsyncMaterializedGraphRead => {
            WorthGraphReadExpectedDenial::new(
                ForgeQueryGraphReadAccessDenialKind::RequiredAsyncMaterialization,
                ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired,
            )
        }
        WorthGraphReadMissingQueryCapability::StoreBackedGraphIndex => {
            WorthGraphReadExpectedDenial::new(
                ForgeQueryGraphReadAccessDenialKind::UnsupportedGraphIndexSupport,
                ForgeQueryGraphReadAccessAdmissionPosture::StoreBackedCapabilityRequired,
            )
        }
        WorthGraphReadMissingQueryCapability::DomainOperationRegistration => {
            WorthGraphReadExpectedDenial::new(
                ForgeQueryGraphReadAccessDenialKind::RequiredAccessCapabilityRegistration,
                ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired,
            )
        }
    }
}

fn milestone_seven_lowering_target_for_row(row: &WorthGraphReadAccessInventoryRow) -> String {
    format!(
        "Milestone 7 graph-read declaration seed for {} via {}",
        row.source_path(),
        row.current_caller()
    )
}

fn query_gap_blocker_for_row(row: &WorthGraphReadAccessInventoryRow) -> String {
    format!(
        "Query lacks {} support for {}",
        missing_query_capability_for_row(row).as_str(),
        row.source_path()
    )
}

fn query_gap_removal_trigger_for_row(row: &WorthGraphReadAccessInventoryRow) -> String {
    format!(
        "Milestone 8 access-plan adoption removes {} from Worth-local graph reads",
        row.source_path()
    )
}

fn deletion_trigger_for_row(row: &WorthGraphReadAccessInventoryRow) -> String {
    format!(
        "Phase 7 public firewall deletes {} after graph-read inventory cutover",
        row.source_path()
    )
}

const fn error(kind: WorthGraphReadAccessPhaseSixErrorKind) -> WorthGraphReadAccessPhaseSixError {
    WorthGraphReadAccessPhaseSixError::new(kind)
}
