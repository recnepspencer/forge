use crate::admission::{
    UiAdmissionDecision, UiAdmissionFamily, UiAdmissionHostCapability, UiAdmissionOutcome,
    UiAdmissionQueryBasis, UiAdmissionReport, UiAdmissionStaleEvidence, UiAdmissionTarget,
    UiAdmissionWorld, UiLegalityDecision, UiLegalityReason, UiSupportPosture, UiSupportReason,
    UiSupportSnapshot,
};
use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportRow, UiDeclarationSupportRowSchemaKind,
    UiDeclaredPostureApplicability,
};
use crate::evidence::UiEvidenceAuthorityGeneration;
use crate::graph::{UiGraphNodeRecord, UiGraphSnapshot};
pub struct UiAdmissionBoundary<'a> {
    pub(super) support_artifacts: &'a [UiDeclarationArtifact],
    pub(super) declaration_artifacts: &'a [UiDeclarationArtifact],
    pub(super) graph_snapshot: &'a UiGraphSnapshot,
}

impl<'a> UiAdmissionBoundary<'a> {
    pub(crate) const fn new(
        declaration_artifacts: &'a [UiDeclarationArtifact],
        graph_snapshot: &'a UiGraphSnapshot,
    ) -> Self {
        Self::from_authority_parts(declaration_artifacts, declaration_artifacts, graph_snapshot)
    }

    pub(crate) const fn from_authority_parts(
        support_artifacts: &'a [UiDeclarationArtifact],
        declaration_artifacts: &'a [UiDeclarationArtifact],
        graph_snapshot: &'a UiGraphSnapshot,
    ) -> Self {
        Self {
            support_artifacts,
            declaration_artifacts,
            graph_snapshot,
        }
    }

    pub fn support_snapshot(&self, target: &UiAdmissionTarget) -> UiSupportSnapshot {
        UiSupportSnapshot::new(target.clone(), self.touch_support_posture(target))
    }

    fn touch_support_posture(&self, target: &UiAdmissionTarget) -> UiSupportPosture {
        if target.world().graph_world_profile() != self.graph_snapshot.world_profile() {
            return UiSupportPosture::WrongWorld {
                family: UiAdmissionFamily::TouchMeaning,
                expected: UiAdmissionWorld::from_graph_world_profile(
                    self.graph_snapshot.world_profile().clone(),
                ),
                observed: target.world().clone(),
            };
        }
        let Some(node_record) = self.graph_node_record(target) else {
            return unsupported_touch_posture(
                target,
                UiSupportReason::TargetOutsideAdmissionBoundary,
            );
        };
        let Some(row) = self.touch_support_row(node_record.declaration_identity()) else {
            return unsupported_touch_posture(
                target,
                UiSupportReason::MissingDeclarationSupportEvidence,
            );
        };
        declared_touch_support_posture(target, row)
    }

    fn touch_support_row(
        &self,
        declaration_identity: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<&UiDeclarationSupportRow> {
        self.support_artifact(declaration_identity)?
            .support_snapshot()
            .ok()?
            .row(UiDeclarationSupportRowSchemaKind::TouchMeaning)
    }

    pub fn admit(&self, target: UiAdmissionTarget) -> UiAdmissionDecision {
        let support_snapshot = self.support_snapshot(&target);
        let outcome = match support_snapshot.posture() {
            UiSupportPosture::DiagnosticOnly { .. } => UiAdmissionOutcome::DiagnosticOnly,
            UiSupportPosture::Unsupported { .. } => UiAdmissionOutcome::Unsupported,
            UiSupportPosture::WrongWorld { .. } => UiAdmissionOutcome::WrongWorld,
            UiSupportPosture::Deferred { .. } => UiAdmissionOutcome::Deferred,
            UiSupportPosture::Supported { .. } => self.legality_outcome(&target),
        };

        UiAdmissionDecision::new(support_snapshot, outcome)
    }

    pub fn report(&self, target: UiAdmissionTarget) -> UiAdmissionReport {
        self.admit(target)
            .into_report(UiEvidenceAuthorityGeneration::new(
                self.graph_snapshot.generation().as_u64(),
            ))
    }

    fn legality_outcome(&self, target: &UiAdmissionTarget) -> UiAdmissionOutcome {
        if let Some(denial) = selection_budget_denial(target) {
            return denial;
        }
        let node_record = self
            .graph_node_record(target)
            .expect("support-gated admission targets must resolve inside the local graph snapshot");
        let Some(artifact) = self.declaration_artifact(node_record.declaration_identity()) else {
            return missing_declaration_artifact_denial(&node_record);
        };
        if let Some(denial) = query_basis_denial(target, &node_record, artifact) {
            return denial;
        }
        if let Some(denial) = host_capability_denial(target, &node_record, artifact) {
            return denial;
        }
        admitted_legality_outcome(&node_record, artifact)
    }

    pub(super) fn graph_node_record(
        &self,
        target: &UiAdmissionTarget,
    ) -> Option<UiGraphNodeRecord> {
        self.graph_snapshot
            .lookup()
            .graph_node(target.graph_node_identity())
            .map(|lookup| lookup.value().clone())
    }

    pub(super) fn declaration_artifact(
        &self,
        declaration_identity: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<&'a UiDeclarationArtifact> {
        self.declaration_artifacts
            .iter()
            .find(|artifact| artifact.identity() == declaration_identity)
    }

    pub(super) fn support_artifact(
        &self,
        declaration_identity: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<&'a UiDeclarationArtifact> {
        self.support_artifacts
            .iter()
            .find(|artifact| artifact.identity() == declaration_identity)
    }
}

const fn required_lane_cost(_target: &UiAdmissionTarget) -> u8 {
    1
}

fn unsupported_touch_posture(
    target: &UiAdmissionTarget,
    reason: UiSupportReason,
) -> UiSupportPosture {
    UiSupportPosture::Unsupported {
        family: UiAdmissionFamily::TouchMeaning,
        reason,
        world: target.world().clone(),
    }
}

fn declared_touch_support_posture(
    target: &UiAdmissionTarget,
    row: &UiDeclarationSupportRow,
) -> UiSupportPosture {
    let family = UiAdmissionFamily::TouchMeaning;
    let world = target.world().clone();
    if let Some(unsupported_posture) = row.unsupported_posture() {
        return UiSupportPosture::Deferred {
            family,
            expected_in: unsupported_posture.expected_in(),
            world,
        };
    }
    match row.applicability() {
        UiDeclaredPostureApplicability::DiagnosticOnly => {
            UiSupportPosture::DiagnosticOnly { family, world }
        }
        UiDeclaredPostureApplicability::Required | UiDeclaredPostureApplicability::Optional => {
            UiSupportPosture::Supported { family, world }
        }
        UiDeclaredPostureApplicability::NotApplicable => UiSupportPosture::Unsupported {
            family,
            reason: UiSupportReason::TouchMeaningNotApplicable,
            world,
        },
        UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted => {
            UiSupportPosture::Deferred {
                family,
                expected_in:
                    crate::declaration::UiDeclarationSupportMilestoneExpectation::Milestone32,
                world,
            }
        }
    }
}

fn selection_budget_denial(target: &UiAdmissionTarget) -> Option<UiAdmissionOutcome> {
    let attempted_lane_cost = required_lane_cost(target);
    (!target
        .selection_budget()
        .admits_lane_cost(attempted_lane_cost))
    .then(|| {
        UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
            Some(target.graph_node_identity()),
            None,
            UiLegalityReason::BudgetExceeded {
                budget: target.selection_budget(),
                attempted_lane_cost,
            },
        ))
    })
}

fn missing_declaration_artifact_denial(node_record: &UiGraphNodeRecord) -> UiAdmissionOutcome {
    UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
        Some(node_record.graph_node_identity()),
        Some(node_record.declaration_identity().clone()),
        UiLegalityReason::Stale {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::StaleReceipt,
            evidence: UiAdmissionStaleEvidence::DeclarationArtifactMissing,
        },
    ))
}

fn query_basis_denial(
    target: &UiAdmissionTarget,
    node_record: &UiGraphNodeRecord,
    artifact: &UiDeclarationArtifact,
) -> Option<UiAdmissionOutcome> {
    if !node_record.attachment_posture().query_binding_attached() {
        return None;
    }
    let reason = match target.query_basis() {
        UiAdmissionQueryBasis::WrongWorldProjection => UiLegalityReason::WrongQueryBasis {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: target.query_basis(),
        },
        UiAdmissionQueryBasis::RebindRequired => UiLegalityReason::RebindRequired {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: target.query_basis(),
        },
        UiAdmissionQueryBasis::StaleReceipt => UiLegalityReason::Stale {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: target.query_basis(),
            evidence: UiAdmissionStaleEvidence::QueryReceiptExpired,
        },
        UiAdmissionQueryBasis::AmbiguousSources => UiLegalityReason::Ambiguous {
            required_query_basis: Some(UiAdmissionQueryBasis::GraphAligned),
            observed_query_basis: Some(target.query_basis()),
            required_host_capability: None,
            observed_host_capability: None,
        },
        UiAdmissionQueryBasis::GraphAligned => return None,
    };
    Some(legality_denial(node_record, artifact, reason))
}

fn host_capability_denial(
    target: &UiAdmissionTarget,
    node_record: &UiGraphNodeRecord,
    artifact: &UiDeclarationArtifact,
) -> Option<UiAdmissionOutcome> {
    if !node_record.attachment_posture().service_usage_attached() {
        return None;
    }
    let reason = match target.host_capability_report() {
        None => UiLegalityReason::MissingHostCapabilityReport,
        Some(report) => match report.posture() {
            worth_ui_host_contract::WorthUiHostCapabilityPosture::Missing => {
                UiLegalityReason::WrongHostCapability {
                    required: UiAdmissionHostCapability::Available,
                    observed: target.host_capability(),
                }
            }
            worth_ui_host_contract::WorthUiHostCapabilityPosture::Ambiguous => {
                UiLegalityReason::Ambiguous {
                    required_query_basis: None,
                    observed_query_basis: None,
                    required_host_capability: Some(UiAdmissionHostCapability::Available),
                    observed_host_capability: Some(target.host_capability()),
                }
            }
            worth_ui_host_contract::WorthUiHostCapabilityPosture::DiagnosticOnly
            | worth_ui_host_contract::WorthUiHostCapabilityPosture::Available => return None,
        },
    };
    Some(legality_denial(node_record, artifact, reason))
}

fn legality_denial(
    node_record: &UiGraphNodeRecord,
    artifact: &UiDeclarationArtifact,
    reason: UiLegalityReason,
) -> UiAdmissionOutcome {
    UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
        Some(node_record.graph_node_identity()),
        Some(artifact.identity().clone()),
        reason,
    ))
}

fn admitted_legality_outcome(
    node_record: &UiGraphNodeRecord,
    artifact: &UiDeclarationArtifact,
) -> UiAdmissionOutcome {
    let advisory = if node_record.attachment_posture().query_binding_attached() {
        Some(UiLegalityReason::QueryBindingRequiresLaterRuntimeLane)
    } else if node_record.attachment_posture().service_usage_attached() {
        Some(UiLegalityReason::ServiceUsageRequiresLaterRuntimeLane)
    } else {
        None
    };
    match advisory {
        Some(reason) => {
            UiAdmissionOutcome::AdmittedWithAdvisory(UiLegalityDecision::admitted_with_advisory(
                node_record.graph_node_identity(),
                artifact.identity().clone(),
                reason,
            ))
        }
        None => UiAdmissionOutcome::Admitted(UiLegalityDecision::admitted(
            node_record.graph_node_identity(),
            artifact.identity().clone(),
        )),
    }
}

#[cfg(test)]
#[path = "ui_admission_boundary_tests.rs"]
mod ui_admission_boundary_tests;
