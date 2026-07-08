use worth_ui_inspection::UiInspectionScopeInventory;

use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::registry::CapabilitySnapshot;
use crate::facade::host_observation::WorthUiHostContract;
use worth_ui_dsl::WorthUiDslPackage;
use crate::graph::{admit_graph_handoffs, UiGraphWorldProfile};
use crate::runtime::WorthUiSourceBackedDeclarationWitness;

use super::bootstrap::{WorthUiCapabilityRegistrationFreezeCore, WorthUiFacadeLifecycleBootstrap};
use super::declaration_freeze::{lower_declaration_artifacts, lower_graph_handoffs};

impl WorthUiCapabilityRegistrationFreezeCore {
    pub(crate) fn freeze_from_registration(
        capability_snapshot: CapabilitySnapshot,
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        graph_world_profile: UiGraphWorldProfile,
        measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
        source_backed_declaration_witness: Option<WorthUiSourceBackedDeclarationWitness>,
    ) -> Self {
        Self::freeze_from_registration_with_inspection_scope_inventory(
            capability_snapshot,
            dsl_package,
            host_contract,
            graph_world_profile,
            measurement_inspection_evidence,
            source_backed_declaration_witness,
            worth_ui_inspection::RUNTIME_INSPECTION_SCOPE_INVENTORY,
        )
    }

    pub(crate) fn freeze_from_registration_with_inspection_scope_inventory(
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
        let lifecycle = WorthUiFacadeLifecycleBootstrap::bootstrap_with_inspection_scope_inventory(
            dsl_package,
            host_contract,
            &declaration_artifacts,
            measurement_inspection_evidence,
            inspection_scope_inventory,
        );
        Self::assemble(capability_snapshot, declaration_artifacts, graph_snapshot, lifecycle)
    }
}