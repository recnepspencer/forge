use crate::admission::{UiSupportPosture, UiSupportSnapshot};
use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportRowSchemaKind, UiDeclaredPostureApplicability,
    UiDeclaredServiceUsagePosture,
};
use crate::graph::UiGraphSnapshot;
use crate::obligations::catalog::{UiObligationCheckKind, UiObligationFamilyCatalog};
use crate::obligations::inspection::{
    UiObligationEvidenceDecision, UiObligationEvidenceHandle, UiObligationEvidenceHandleKind,
    UiObligationEvidenceIndex, UiObligationEvidencePrerequisiteSource, UiObligationEvidenceRecord,
    UiObligationNonSelectionReason,
};
use crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef;
use crate::obligations::touch::UiGraphTouchDescriptor;

use super::{
    UiObligationSelectionMatrix, UiObligationSupportBasis, UiObligationSupportSelectionPosture,
    UiSelectedObligation, UiSelectedObligationIdentity, UiSelectedObligationSet,
};

pub struct UiObligationSelectionBoundary<'a> {
    support_artifacts: &'a [UiDeclarationArtifact],
    graph_snapshot: &'a UiGraphSnapshot,
    family_catalog: UiObligationFamilyCatalog,
}

impl<'a> UiObligationSelectionBoundary<'a> {
    pub(crate) const fn new(
        support_artifacts: &'a [UiDeclarationArtifact],
        graph_snapshot: &'a UiGraphSnapshot,
    ) -> Self {
        Self {
            support_artifacts,
            graph_snapshot,
            family_catalog: UiObligationFamilyCatalog::closed(),
        }
    }

    pub fn select(
        &self,
        touch: &UiGraphTouchDescriptor,
        support_snapshot: UiSupportSnapshot,
    ) -> UiSelectedObligationSet {
        let (obligations, evidence_records) = match support_snapshot.posture() {
            UiSupportPosture::WrongWorld { .. } => (Vec::new(), Vec::new()),
            UiSupportPosture::Supported { .. }
            | UiSupportPosture::Unsupported { .. }
            | UiSupportPosture::Deferred { .. }
            | UiSupportPosture::DiagnosticOnly { .. } => {
                self.select_supported_matrix(touch, &support_snapshot)
            }
        };

        UiSelectedObligationSet::new(
            touch.clone(),
            support_snapshot,
            obligations.into_boxed_slice(),
            UiObligationEvidenceIndex::new(evidence_records.into_boxed_slice()),
        )
    }

    fn select_supported_matrix(
        &self,
        touch: &UiGraphTouchDescriptor,
        support_snapshot: &UiSupportSnapshot,
    ) -> (Vec<UiSelectedObligation>, Vec<UiObligationEvidenceRecord>) {
        let node_record = self
            .graph_snapshot
            .lookup()
            .graph_node(touch.target().graph_node_identity())
            .map(|lookup| lookup.value().clone());
        let target = support_snapshot.target();
        let mut obligations = Vec::new();
        let mut evidence_records = Vec::new();
        for (ordinal, row) in UiObligationSelectionMatrix::starter()
            .rows()
            .iter()
            .copied()
            .enumerate()
        {
            let row_support_posture = self.row_support_posture(
                row.support_basis(),
                node_record.as_ref(),
                support_snapshot.posture(),
            );
            let graph_node_digest = touch.target().graph_node_identity().digest();
            let base_reasons = row
                .selection_reasons(touch, row_support_posture)
                .into_boxed_slice();
            if !row.matches(touch, node_record.as_ref(), row_support_posture) {
                evidence_records.push(UiObligationEvidenceRecord::new(
                    UiObligationEvidenceHandle::new(
                        UiObligationEvidenceHandleKind::NotSelected,
                        touch.identity_digest() ^ (ordinal as u64).rotate_left(17),
                    ),
                    graph_node_digest,
                    Some(touch.identity_digest()),
                    Some(row.family()),
                    UiObligationEvidenceDecision::NotSelected,
                    None,
                    base_reasons,
                    prerequisite_sources(row.support_basis(), target),
                    Some(UiObligationNonSelectionReason::RuleDidNotMatch),
                    None,
                ));
                continue;
            }

            let Some(family) = self.family_for_row(row, node_record.as_ref()) else {
                evidence_records.push(UiObligationEvidenceRecord::new(
                    UiObligationEvidenceHandle::new(
                        UiObligationEvidenceHandleKind::NotSelected,
                        touch.identity_digest() ^ (ordinal as u64).rotate_left(23),
                    ),
                    graph_node_digest,
                    Some(touch.identity_digest()),
                    Some(row.family()),
                    UiObligationEvidenceDecision::NotSelected,
                    None,
                    base_reasons,
                    prerequisite_sources(row.support_basis(), target),
                    Some(UiObligationNonSelectionReason::FamilyUnavailable),
                    None,
                ));
                continue;
            };

            match row_support_posture {
                UiObligationSupportSelectionPosture::Supported
                | UiObligationSupportSelectionPosture::Unsupported
                | UiObligationSupportSelectionPosture::Deferred
                | UiObligationSupportSelectionPosture::DiagnosticOnly => {
                    let evidence_handle = UiObligationEvidenceHandle::new(
                        UiObligationEvidenceHandleKind::Selected,
                        touch.identity_digest()
                            ^ (family as u64).rotate_left(11)
                            ^ (ordinal as u64).rotate_left(29),
                    );
                    let prerequisite_evidence_refs =
                        prerequisite_evidence_refs(row.support_basis(), target);
                    evidence_records.push(UiObligationEvidenceRecord::new(
                        evidence_handle,
                        graph_node_digest,
                        Some(touch.identity_digest()),
                        Some(family),
                        UiObligationEvidenceDecision::Selected,
                        None,
                        base_reasons.clone(),
                        prerequisite_sources(row.support_basis(), target),
                        None,
                        None,
                    ));
                    obligations.push(self.obligation(
                        touch,
                        family,
                        row,
                        row_support_posture,
                        row.check_kind(),
                        evidence_handle,
                        base_reasons,
                        prerequisite_evidence_refs,
                    ));
                }
                UiObligationSupportSelectionPosture::WrongWorld => {
                    evidence_records.push(UiObligationEvidenceRecord::new(
                        UiObligationEvidenceHandle::new(
                            UiObligationEvidenceHandleKind::NotSelected,
                            touch.identity_digest() ^ (ordinal as u64).rotate_left(31),
                        ),
                        graph_node_digest,
                        Some(touch.identity_digest()),
                        Some(family),
                        UiObligationEvidenceDecision::NotSelected,
                        None,
                        base_reasons,
                        prerequisite_sources(row.support_basis(), target),
                        Some(UiObligationNonSelectionReason::WrongWorld),
                        None,
                    ));
                }
            }
        }

        (obligations, evidence_records)
    }

    fn obligation(
        &self,
        touch: &UiGraphTouchDescriptor,
        family: crate::obligations::catalog::UiObligationFamily,
        row: super::UiObligationSelectionMatrixRow,
        support_posture: UiObligationSupportSelectionPosture,
        check_kind: UiObligationCheckKind,
        evidence_handle: UiObligationEvidenceHandle,
        selection_reasons: Box<[super::UiObligationSelectionReason]>,
        prerequisite_evidence_refs: Box<[UiObligationPrerequisiteEvidenceRef]>,
    ) -> UiSelectedObligation {
        debug_assert!(self.family_catalog.contains(family));
        let identity = UiSelectedObligationIdentity::new(
            touch.identity_digest(),
            family,
            touch.target(),
            row.aspect_scope()
                .iter()
                .copied()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            touch.world(),
            row.support_basis(),
        );
        UiSelectedObligation::new(
            identity,
            family,
            check_kind,
            support_posture,
            evidence_handle,
            selection_reasons,
            prerequisite_evidence_refs,
        )
    }

    fn row_support_posture(
        &self,
        support_basis: UiObligationSupportBasis,
        node_record: Option<&crate::graph::UiGraphNodeRecord>,
        touch_support_posture: &UiSupportPosture,
    ) -> UiObligationSupportSelectionPosture {
        if matches!(support_basis, UiObligationSupportBasis::TouchMeaning) {
            return UiObligationSupportSelectionPosture::from_support_posture(
                touch_support_posture,
            );
        }

        let Some(node_record) = node_record else {
            return UiObligationSupportSelectionPosture::Unsupported;
        };
        let Some(artifact) = self
            .support_artifacts
            .iter()
            .find(|artifact| artifact.identity() == node_record.declaration_identity())
        else {
            return UiObligationSupportSelectionPosture::Unsupported;
        };
        let Ok(snapshot) = artifact.support_snapshot() else {
            return UiObligationSupportSelectionPosture::Unsupported;
        };
        let Some(row) = snapshot.row(schema_kind_for_support_basis(support_basis)) else {
            return UiObligationSupportSelectionPosture::Unsupported;
        };

        if row.unsupported_posture().is_some() {
            UiObligationSupportSelectionPosture::Deferred
        } else {
            match row.applicability() {
                UiDeclaredPostureApplicability::Required
                | UiDeclaredPostureApplicability::Optional => {
                    UiObligationSupportSelectionPosture::Supported
                }
                UiDeclaredPostureApplicability::DiagnosticOnly => {
                    UiObligationSupportSelectionPosture::DiagnosticOnly
                }
                UiDeclaredPostureApplicability::NotApplicable => {
                    UiObligationSupportSelectionPosture::Unsupported
                }
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted => {
                    UiObligationSupportSelectionPosture::Deferred
                }
            }
        }
    }

    fn family_for_row(
        &self,
        row: super::UiObligationSelectionMatrixRow,
        node_record: Option<&crate::graph::UiGraphNodeRecord>,
    ) -> Option<crate::obligations::catalog::UiObligationFamily> {
        if row.support_basis() != UiObligationSupportBasis::ServiceUsage
            || row.family()
                != crate::obligations::catalog::UiObligationFamily::PortalHostRequirement
        {
            return Some(row.family());
        }

        let node_record = node_record?;
        let artifact = self
            .support_artifacts
            .iter()
            .find(|artifact| artifact.identity() == node_record.declaration_identity())?;
        let snapshot = artifact.support_snapshot().ok()?;
        let row = snapshot.row(UiDeclarationSupportRowSchemaKind::ServiceUsage)?;
        match row.declared_service_usage_posture()? {
            UiDeclaredServiceUsagePosture::Portal => {
                Some(crate::obligations::catalog::UiObligationFamily::PortalHostRequirement)
            }
            UiDeclaredServiceUsagePosture::FocusRouting => {
                Some(crate::obligations::catalog::UiObligationFamily::FocusRouteRequirement)
            }
            UiDeclaredServiceUsagePosture::Motion => {
                Some(crate::obligations::catalog::UiObligationFamily::MotionSupportRequirement)
            }
            UiDeclaredServiceUsagePosture::Scroll => None,
        }
    }
}

const fn schema_kind_for_support_basis(
    support_basis: UiObligationSupportBasis,
) -> UiDeclarationSupportRowSchemaKind {
    match support_basis {
        UiObligationSupportBasis::TouchMeaning => UiDeclarationSupportRowSchemaKind::TouchMeaning,
        UiObligationSupportBasis::QueryBinding => UiDeclarationSupportRowSchemaKind::QueryBinding,
        UiObligationSupportBasis::ServiceUsage => UiDeclarationSupportRowSchemaKind::ServiceUsage,
        UiObligationSupportBasis::MeasurementPolicy => {
            UiDeclarationSupportRowSchemaKind::MeasurementPolicy
        }
        UiObligationSupportBasis::HostCapability => {
            UiDeclarationSupportRowSchemaKind::HostCapability
        }
    }
}

fn prerequisite_evidence_refs(
    support_basis: UiObligationSupportBasis,
    target: &crate::admission::UiAdmissionTarget,
) -> Box<[UiObligationPrerequisiteEvidenceRef]> {
    let mut refs = Vec::new();
    if matches!(support_basis, UiObligationSupportBasis::QueryBinding) {
        if let Some(query_prerequisites) = target.query_prerequisites() {
            refs.push(UiObligationPrerequisiteEvidenceRef::Query(
                query_prerequisites.clone(),
            ));
        }
    }
    if matches!(
        support_basis,
        UiObligationSupportBasis::HostCapability | UiObligationSupportBasis::ServiceUsage
    ) {
        if let Some(host_capability_report) = target.host_capability_report() {
            refs.push(UiObligationPrerequisiteEvidenceRef::Host(
                host_capability_report.clone(),
            ));
        }
    }
    refs.into_boxed_slice()
}

fn prerequisite_sources(
    support_basis: UiObligationSupportBasis,
    target: &crate::admission::UiAdmissionTarget,
) -> Box<[UiObligationEvidencePrerequisiteSource]> {
    let mut sources = Vec::new();
    if matches!(support_basis, UiObligationSupportBasis::QueryBinding) {
        if let Some(query_prerequisites) = target.query_prerequisites() {
            sources.push(UiObligationEvidencePrerequisiteSource::QueryBasis);
            sources.push(UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption);
            if query_prerequisites.inspection_lane()
                == worth_ui_query_binding::WorthUiQueryInspectionLane::WorkspaceInspect
            {
                sources.push(UiObligationEvidencePrerequisiteSource::QueryInspection);
            }
            if query_prerequisites.causal_explanation_lane()
                == worth_ui_query_binding::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
            {
                sources.push(UiObligationEvidencePrerequisiteSource::QueryCausalExplanation);
            }
        }
    }
    if matches!(
        support_basis,
        UiObligationSupportBasis::HostCapability | UiObligationSupportBasis::ServiceUsage
    ) && target.host_capability_report().is_some()
    {
        sources.push(UiObligationEvidencePrerequisiteSource::HostCapability);
    }
    sources.into_boxed_slice()
}
