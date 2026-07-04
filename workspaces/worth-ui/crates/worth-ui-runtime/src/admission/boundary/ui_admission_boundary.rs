use crate::admission::{
    UiAdmissionDecision, UiAdmissionFamily, UiAdmissionHostCapability, UiAdmissionOutcome,
    UiAdmissionQueryBasis, UiAdmissionReport, UiAdmissionStaleEvidence, UiAdmissionTarget,
    UiAdmissionWorld, UiLegalityDecision, UiLegalityReason, UiSupportPosture, UiSupportReason,
    UiSupportSnapshot,
};
use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportRowSchemaKind, UiDeclaredPostureApplicability,
};
use crate::evidence::UiEvidenceAuthorityGeneration;
use crate::graph::{UiGraphNodeRecord, UiGraphSnapshot};
use crate::obligations::dispatch::{UiObligationDispatchBoundary, UiObligationDispatchPlan};
use crate::obligations::selection::{UiObligationSelectionBoundary, UiSelectedObligationSet};
use crate::obligations::touch::UiGraphTouchDescriptor;
use crate::obligations::verdict::UiObligationVerdict;

pub struct UiAdmissionBoundary<'a> {
    support_artifacts: &'a [UiDeclarationArtifact],
    declaration_artifacts: &'a [UiDeclarationArtifact],
    graph_snapshot: &'a UiGraphSnapshot,
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
        let family = UiAdmissionFamily::TouchMeaning;
        if target.world().graph_world_profile() != self.graph_snapshot.world_profile() {
            return UiSupportSnapshot::new(
                target.clone(),
                UiSupportPosture::WrongWorld {
                    family,
                    expected: UiAdmissionWorld::from_graph_world_profile(
                        self.graph_snapshot.world_profile().clone(),
                    ),
                    observed: target.world().clone(),
                },
            );
        }

        let Some(node_record) = self.graph_node_record(target) else {
            return UiSupportSnapshot::new(
                target.clone(),
                UiSupportPosture::Unsupported {
                    family,
                    reason: UiSupportReason::TargetOutsideAdmissionBoundary,
                    world: target.world().clone(),
                },
            );
        };
        let declaration_identity = node_record.declaration_identity();
        let Some(artifact) = self.support_artifact(&declaration_identity) else {
            return UiSupportSnapshot::new(
                target.clone(),
                UiSupportPosture::Unsupported {
                    family,
                    reason: UiSupportReason::MissingDeclarationSupportEvidence,
                    world: target.world().clone(),
                },
            );
        };
        let Ok(snapshot) = artifact.support_snapshot() else {
            return UiSupportSnapshot::new(
                target.clone(),
                UiSupportPosture::Unsupported {
                    family,
                    reason: UiSupportReason::MissingDeclarationSupportEvidence,
                    world: target.world().clone(),
                },
            );
        };
        let Some(row) = snapshot.row(UiDeclarationSupportRowSchemaKind::TouchMeaning) else {
            return UiSupportSnapshot::new(
                target.clone(),
                UiSupportPosture::Unsupported {
                    family,
                    reason: UiSupportReason::MissingDeclarationSupportEvidence,
                    world: target.world().clone(),
                },
            );
        };

        let posture = match row.unsupported_posture() {
            Some(unsupported_posture) => UiSupportPosture::Deferred {
                family,
                expected_in: unsupported_posture.expected_in(),
                world: target.world().clone(),
            },
            None => match row.applicability() {
                UiDeclaredPostureApplicability::DiagnosticOnly => UiSupportPosture::DiagnosticOnly {
                    family,
                    world: target.world().clone(),
                },
                UiDeclaredPostureApplicability::Required
                | UiDeclaredPostureApplicability::Optional => UiSupportPosture::Supported {
                    family,
                    world: target.world().clone(),
                },
                UiDeclaredPostureApplicability::NotApplicable => UiSupportPosture::Unsupported {
                    family,
                    reason: UiSupportReason::TouchMeaningNotApplicable,
                    world: target.world().clone(),
                },
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted =>
                    UiSupportPosture::Deferred {
                        family,
                        expected_in: crate::declaration::UiDeclarationSupportMilestoneExpectation::Milestone32,
                        world: target.world().clone(),
                    },
            },
        };

        UiSupportSnapshot::new(target.clone(), posture)
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

    pub fn select_obligations(&self, touch: &UiGraphTouchDescriptor) -> UiSelectedObligationSet {
        self.select_obligations_for_target(
            touch,
            UiAdmissionTarget::graph_node(
                touch.target().graph_node_identity(),
                UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
            ),
        )
    }

    pub fn select_obligations_for_target(
        &self,
        touch: &UiGraphTouchDescriptor,
        target: UiAdmissionTarget,
    ) -> UiSelectedObligationSet {
        let selection_target = selection_target_for_touch(touch, target);
        let support_snapshot = self.support_snapshot(&selection_target);

        UiObligationSelectionBoundary::new(self.support_artifacts, self.graph_snapshot)
            .select(touch, support_snapshot)
    }

    pub fn lower_obligation_dispatch(
        &self,
        selected: &UiSelectedObligationSet,
    ) -> UiObligationDispatchPlan {
        let _ = self;
        UiObligationDispatchBoundary::new().lower(selected)
    }

    pub fn dispatch_selected_obligations(
        &self,
        selected: &UiSelectedObligationSet,
    ) -> Box<[UiObligationVerdict]> {
        self.lower_obligation_dispatch(selected).execute()
    }

    pub fn admit_selected_obligations(
        &self,
        selected: &UiSelectedObligationSet,
    ) -> UiAdmissionReport {
        let dispatch_plan = self.lower_obligation_dispatch(selected);
        let verdicts = dispatch_plan.execute();
        UiAdmissionReport::from_selected_execution(selected, dispatch_plan, verdicts)
    }

    fn legality_outcome(&self, target: &UiAdmissionTarget) -> UiAdmissionOutcome {
        let ordinary_lane_cost = required_lane_cost(target);
        if !target
            .selection_budget()
            .admits_lane_cost(ordinary_lane_cost)
        {
            return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                Some(target.graph_node_identity()),
                None,
                UiLegalityReason::BudgetExceeded {
                    budget: target.selection_budget(),
                    attempted_lane_cost: ordinary_lane_cost,
                },
            ));
        }
        let node_record = self
            .graph_node_record(target)
            .expect("support-gated admission targets must resolve inside the local graph snapshot");
        let Some(artifact) = self.declaration_artifact(node_record.declaration_identity()) else {
            return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                Some(node_record.graph_node_identity()),
                Some(node_record.declaration_identity().clone()),
                UiLegalityReason::Stale {
                    required: UiAdmissionQueryBasis::GraphAligned,
                    observed: UiAdmissionQueryBasis::StaleReceipt,
                    evidence: UiAdmissionStaleEvidence::DeclarationArtifactMissing,
                },
            ));
        };

        if node_record.attachment_posture().query_binding_attached() {
            let Some(query_prerequisites) = target.query_prerequisites() else {
                return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                    Some(node_record.graph_node_identity()),
                    Some(artifact.identity().clone()),
                    UiLegalityReason::MissingQueryPrerequisiteEvidence,
                ));
            };

            match query_prerequisites.basis_posture() {
                worth_ui_query_binding::WorthUiQueryBasisPosture::WrongWorldProjection => {
                    return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                        Some(node_record.graph_node_identity()),
                        Some(artifact.identity().clone()),
                        UiLegalityReason::WrongQueryBasis {
                            required: UiAdmissionQueryBasis::GraphAligned,
                            observed: target.query_basis(),
                        },
                    ));
                }
                worth_ui_query_binding::WorthUiQueryBasisPosture::RebindRequired => {
                    return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                        Some(node_record.graph_node_identity()),
                        Some(artifact.identity().clone()),
                        UiLegalityReason::RebindRequired {
                            required: UiAdmissionQueryBasis::GraphAligned,
                            observed: target.query_basis(),
                        },
                    ));
                }
                worth_ui_query_binding::WorthUiQueryBasisPosture::StaleReceipt => {
                    return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                        Some(node_record.graph_node_identity()),
                        Some(artifact.identity().clone()),
                        UiLegalityReason::Stale {
                            required: UiAdmissionQueryBasis::GraphAligned,
                            observed: target.query_basis(),
                            evidence: UiAdmissionStaleEvidence::QueryReceiptExpired,
                        },
                    ));
                }
                worth_ui_query_binding::WorthUiQueryBasisPosture::AmbiguousSources => {
                    return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                        Some(node_record.graph_node_identity()),
                        Some(artifact.identity().clone()),
                        UiLegalityReason::Ambiguous {
                            required_query_basis: Some(UiAdmissionQueryBasis::GraphAligned),
                            observed_query_basis: Some(target.query_basis()),
                            required_host_capability: None,
                            observed_host_capability: None,
                        },
                    ));
                }
                worth_ui_query_binding::WorthUiQueryBasisPosture::GraphAligned => {}
            }
        }

        if node_record.attachment_posture().service_usage_attached() {
            let Some(host_capability_report) = target.host_capability_report() else {
                return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                    Some(node_record.graph_node_identity()),
                    Some(artifact.identity().clone()),
                    UiLegalityReason::MissingHostCapabilityReport,
                ));
            };

            match host_capability_report.posture() {
                worth_ui_host_contract::WorthUiHostCapabilityPosture::Missing => {
                    return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                        Some(node_record.graph_node_identity()),
                        Some(artifact.identity().clone()),
                        UiLegalityReason::WrongHostCapability {
                            required: UiAdmissionHostCapability::Available,
                            observed: target.host_capability(),
                        },
                    ));
                }
                worth_ui_host_contract::WorthUiHostCapabilityPosture::Ambiguous => {
                    return UiAdmissionOutcome::Denied(UiLegalityDecision::denied(
                        Some(node_record.graph_node_identity()),
                        Some(artifact.identity().clone()),
                        UiLegalityReason::Ambiguous {
                            required_query_basis: None,
                            observed_query_basis: None,
                            required_host_capability: Some(UiAdmissionHostCapability::Available),
                            observed_host_capability: Some(target.host_capability()),
                        },
                    ));
                }
                worth_ui_host_contract::WorthUiHostCapabilityPosture::DiagnosticOnly
                | worth_ui_host_contract::WorthUiHostCapabilityPosture::Available => {}
            }
        }

        if node_record.attachment_posture().query_binding_attached() {
            return UiAdmissionOutcome::AdmittedWithAdvisory(
                UiLegalityDecision::admitted_with_advisory(
                    node_record.graph_node_identity(),
                    artifact.identity().clone(),
                    UiLegalityReason::QueryBindingRequiresLaterRuntimeLane,
                ),
            );
        }

        if node_record.attachment_posture().service_usage_attached() {
            return UiAdmissionOutcome::AdmittedWithAdvisory(
                UiLegalityDecision::admitted_with_advisory(
                    node_record.graph_node_identity(),
                    artifact.identity().clone(),
                    UiLegalityReason::ServiceUsageRequiresLaterRuntimeLane,
                ),
            );
        }

        UiAdmissionOutcome::Admitted(UiLegalityDecision::admitted(
            node_record.graph_node_identity(),
            artifact.identity().clone(),
        ))
    }

    fn graph_node_record(&self, target: &UiAdmissionTarget) -> Option<UiGraphNodeRecord> {
        self.graph_snapshot
            .lookup()
            .graph_node(target.graph_node_identity())
            .map(|lookup| lookup.value().clone())
    }

    fn declaration_artifact(
        &self,
        declaration_identity: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<&'a UiDeclarationArtifact> {
        self.declaration_artifacts
            .iter()
            .find(|artifact| artifact.identity() == declaration_identity)
    }

    fn support_artifact(
        &self,
        declaration_identity: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<&'a UiDeclarationArtifact> {
        self.support_artifacts
            .iter()
            .find(|artifact| artifact.identity() == declaration_identity)
    }
}

const fn required_lane_cost(target: &UiAdmissionTarget) -> u8 {
    let _ = target;
    1
}

fn selection_target_for_touch(
    touch: &UiGraphTouchDescriptor,
    target: UiAdmissionTarget,
) -> UiAdmissionTarget {
    let mut selection_target = UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_selection_budget(target.selection_budget());

    if let Some(query_prerequisites) = target.query_prerequisites() {
        selection_target = selection_target.with_query_prerequisites(query_prerequisites.clone());
    }

    if let Some(host_capability_report) = target.host_capability_report() {
        selection_target =
            selection_target.with_host_capability_report(host_capability_report.clone());
    }

    selection_target
}

#[cfg(test)]
#[path = "ui_admission_boundary_tests.rs"]
mod ui_admission_boundary_tests;
