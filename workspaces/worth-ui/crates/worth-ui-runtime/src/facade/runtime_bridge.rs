use worth_ui_inspection::{UiInspectionScopeInventory, RUNTIME_INSPECTION_SCOPE_INVENTORY};

use crate::declaration::{
    derive_declaration_inspection_support_projection, UiDeclarationArtifact,
    UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial,
    UiDeclarationInspectionSupportProjection, UiDeclarationLowering,
};
use crate::facade::measurement_inspection_evidence::UiMeasurementInspectionEvidenceSnapshot;
use crate::facade::{
    inspection_observation::WorthUiInspectionObservationState, CapabilitySnapshot,
    UiInspectionScope, UiInspectionSupportReport, UiMeasurementInspectionEvidenceBundle,
    WorthUiDslPackage, WorthUiHostContract, WorthUiRuntimeSupportInventory,
    PHASE3_RUNTIME_SUPPORT_INVENTORY,
};
use crate::graph::{admit_graph_handoffs, UiGraphSnapshot, UiGraphWorldProfile};
use crate::runtime::WorthUiSourceBackedDeclarationWitness;
pub(crate) struct WorthUiFacadeLifecycleBootstrap {
    inspection_scope_inventory: UiInspectionScopeInventory,
    declaration_inspection_support: UiDeclarationInspectionSupportProjection,
    measurement_inspection_evidence: UiMeasurementInspectionEvidenceSnapshot,
    inspection_observation: WorthUiInspectionObservationState,
    runtime_support_inventory: WorthUiRuntimeSupportInventory,
    _dsl_package: WorthUiDslPackage,
    _host_contract: WorthUiHostContract,
}

impl WorthUiFacadeLifecycleBootstrap {
    fn new(
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        declaration_artifacts: &[UiDeclarationArtifact],
        measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
    ) -> Self {
        Self::new_with_inspection_scope_inventory(
            dsl_package,
            host_contract,
            declaration_artifacts,
            measurement_inspection_evidence,
            RUNTIME_INSPECTION_SCOPE_INVENTORY,
        )
    }

    pub(crate) fn new_with_inspection_scope_inventory(
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        declaration_artifacts: &[UiDeclarationArtifact],
        measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
        inspection_scope_inventory: UiInspectionScopeInventory,
    ) -> Self {
        Self {
            inspection_scope_inventory,
            declaration_inspection_support: derive_declaration_inspection_support_projection(
                declaration_artifacts,
            ),
            measurement_inspection_evidence: UiMeasurementInspectionEvidenceSnapshot::from_bundles(
                measurement_inspection_evidence,
            ),
            inspection_observation: WorthUiInspectionObservationState::new(),
            runtime_support_inventory: PHASE3_RUNTIME_SUPPORT_INVENTORY,
            _dsl_package: dsl_package,
            _host_contract: host_contract,
        }
    }

    pub(crate) fn inspection_support_report(
        &self,
        scope: UiInspectionScope,
    ) -> UiInspectionSupportReport {
        self.inspection_observation.record_support_report();
        if let Some(report) = self.declaration_inspection_support.support_report(scope) {
            return report;
        }
        self.inspection_scope_inventory.support_report(scope)
    }

    pub(crate) fn inspection_closure_report(&self) -> crate::facade::UiInspectionClosureReport {
        self.inspection_scope_inventory.closure_report()
    }

    pub(crate) fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        &self.runtime_support_inventory
    }

    pub(crate) fn inspection_observation(&self) -> crate::facade::UiInspectionFacadeObservation {
        self.inspection_observation.snapshot()
    }

    pub(crate) fn measurement_inspection_evidence(
        &self,
    ) -> &UiMeasurementInspectionEvidenceSnapshot {
        &self.measurement_inspection_evidence
    }

    pub(crate) fn record_inspection_query(&self) {
        self.inspection_observation.record_query();
    }

    pub(crate) fn record_unsupported_inspection_query(&self) {
        self.inspection_observation.record_unsupported_query();
    }

    pub(crate) fn record_rich_artifact_materialization(&self) {
        self.inspection_observation
            .record_rich_artifact_materialization();
    }

    pub(crate) fn record_authored_lookup(&self) {
        self.inspection_observation.record_authored_lookup();
    }

    pub(crate) fn record_graph_node_evidence_index_rebuild(&self) {
        self.inspection_observation
            .record_graph_node_evidence_index_rebuild();
    }

    pub(crate) fn record_graph_aspect_evidence_index_rebuild(&self) {
        self.inspection_observation
            .record_graph_aspect_evidence_index_rebuild();
    }
}

pub(crate) struct WorthUiCapabilityRegistrationFreezeCore {
    capability_snapshot: CapabilitySnapshot,
    declaration_artifacts: Vec<UiDeclarationArtifact>,
    graph_snapshot: UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
}

impl WorthUiCapabilityRegistrationFreezeCore {
    pub(crate) fn new(
        capability_snapshot: CapabilitySnapshot,
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        graph_world_profile: UiGraphWorldProfile,
        measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
        source_backed_declaration_witness: Option<WorthUiSourceBackedDeclarationWitness>,
    ) -> Self {
        let declaration_artifacts =
            lower_declaration_artifacts(&dsl_package, source_backed_declaration_witness.as_ref());
        let graph_handoffs = lower_graph_handoffs(&declaration_artifacts)
            .expect("freeze path must deny graph instantiation before mutation when sealed handoff lowering fails");
        let graph_snapshot = admit_graph_handoffs(&graph_handoffs, &[])
            .expect("sealed graph handoff freeze path should not admit contradictory runtime basis")
            .commit_initial_generation(graph_world_profile)
            .expect("freeze path must deny before publishing graph authority")
            .into_committed_snapshot();
        let lifecycle = WorthUiFacadeLifecycleBootstrap::new(
            dsl_package,
            host_contract,
            &declaration_artifacts,
            measurement_inspection_evidence,
        );
        Self {
            capability_snapshot,
            graph_snapshot,
            lifecycle,
            declaration_artifacts,
        }
    }

    pub(crate) fn new_with_inspection_scope_inventory(
        capability_snapshot: CapabilitySnapshot,
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        graph_world_profile: UiGraphWorldProfile,
        measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
        source_backed_declaration_witness: Option<WorthUiSourceBackedDeclarationWitness>,
        inspection_scope_inventory: UiInspectionScopeInventory,
    ) -> Self {
        let declaration_artifacts =
            lower_declaration_artifacts(&dsl_package, source_backed_declaration_witness.as_ref());
        let graph_handoffs = lower_graph_handoffs(&declaration_artifacts)
            .expect("freeze path must deny graph instantiation before mutation when sealed handoff lowering fails");
        let graph_snapshot = admit_graph_handoffs(&graph_handoffs, &[])
            .expect("sealed graph handoff freeze path should not admit contradictory runtime basis")
            .commit_initial_generation(graph_world_profile)
            .expect("freeze path must deny before publishing graph authority")
            .into_committed_snapshot();
        let lifecycle = WorthUiFacadeLifecycleBootstrap::new_with_inspection_scope_inventory(
            dsl_package,
            host_contract,
            &declaration_artifacts,
            measurement_inspection_evidence,
            inspection_scope_inventory,
        );
        Self {
            capability_snapshot,
            graph_snapshot,
            declaration_artifacts,
            lifecycle,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        Vec<UiDeclarationArtifact>,
        UiGraphSnapshot,
        WorthUiFacadeLifecycleBootstrap,
    ) {
        (
            self.capability_snapshot,
            self.declaration_artifacts,
            self.graph_snapshot,
            self.lifecycle,
        )
    }
}

fn lower_declaration_artifacts(
    dsl_package: &WorthUiDslPackage,
    source_backed_declaration_witness: Option<&WorthUiSourceBackedDeclarationWitness>,
) -> Vec<UiDeclarationArtifact> {
    let mut declaration_artifacts = dsl_package
        .runtime_lowering_receipts()
        .iter()
        .cloned()
        .map(UiDeclarationLowering::lower)
        .collect::<Vec<_>>();
    if let Some(source_backed_declaration_witness) = source_backed_declaration_witness {
        admit_source_backed_mosaic_sizing_contracts(
            &mut declaration_artifacts,
            source_backed_declaration_witness,
        )
        .expect(
            "freeze path must deny source-backed declaration authority drift before graph handoff",
        );
    }
    declaration_artifacts
}

fn lower_graph_handoffs(
    declaration_artifacts: &[UiDeclarationArtifact],
) -> Result<Vec<UiDeclarationGraphHandoff>, UiDeclarationGraphHandoffDenial> {
    declaration_artifacts
        .iter()
        .map(UiDeclarationArtifact::graph_handoff)
        .collect()
}

fn admit_source_backed_mosaic_sizing_contracts(
    declaration_artifacts: &mut [UiDeclarationArtifact],
    source_backed_declaration_witness: &WorthUiSourceBackedDeclarationWitness,
) -> Result<(), UiDeclarationGraphHandoffDenial> {
    for artifact in declaration_artifacts {
        let provenance = artifact.provenance().source_provenance();
        if let Some(claims) = source_backed_declaration_witness
            .claims_for(provenance.module_path(), provenance.declaration_index())
        {
            artifact.admit_source_backed_mosaic_sizing_contract_id(
                claims.mosaic_sizing_contract_id().clone(),
            )?;
            artifact
                .admit_source_backed_mosaic_membership_name(claims.mosaic_membership_name());
            artifact.admit_source_backed_measurement_constraint_modifier(
                claims.measurement_constraint_modifier(),
            );
        }
    }

    Ok(())
}
