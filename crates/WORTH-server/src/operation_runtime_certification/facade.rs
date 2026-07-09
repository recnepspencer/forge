use crate::{
    WorthServerOperationFamily, WorthServerOperationInventory,
    WorthServerProductAdapterRegistrationReceipt, WorthServerQueryDependencyAuditFacade,
    WorthServerRouteInventory, WorthServerSurfaceFamily,
};

use super::{
    phase_artifact_rows::{
        authority_footprint_requirement_row, authorization_posture_requirement_row,
        precondition_posture_requirement_row, requirement_row as build_requirement_row,
        support_posture_requirement_row,
    },
    WorthServerEditorLikeOperationFixture, WorthServerNoProductSemanticsCertification,
    WorthServerProductEditorReadinessCertification,
    WorthServerProductOperationRuntimeArtifactRequirements,
    WorthServerProductOperationRuntimeCertification,
    WorthServerProductOperationRuntimeRequirementRow, WorthServerProductOperationRuntimeSupportRow,
};

#[derive(Clone, Debug)]
pub struct WorthServerProductOperationRuntimeCertificationFacade {
    query_dependency_audit: WorthServerQueryDependencyAuditFacade,
    operation_inventory: WorthServerOperationInventory,
    route_inventory: WorthServerRouteInventory,
    product_adapter_receipts: Vec<WorthServerProductAdapterRegistrationReceipt>,
}

impl WorthServerProductOperationRuntimeCertificationFacade {
    pub(crate) fn new(
        query_dependency_audit: WorthServerQueryDependencyAuditFacade,
        operation_inventory: WorthServerOperationInventory,
        route_inventory: WorthServerRouteInventory,
        product_adapter_receipts: Vec<WorthServerProductAdapterRegistrationReceipt>,
    ) -> Self {
        Self {
            query_dependency_audit,
            operation_inventory,
            route_inventory,
            product_adapter_receipts,
        }
    }

    pub fn certify_product_editor_readiness(
        &self,
        fixture: WorthServerEditorLikeOperationFixture,
    ) -> WorthServerProductOperationRuntimeCertification {
        let editor_readiness = WorthServerProductEditorReadinessCertification::new(fixture);
        let no_product_semantics = WorthServerNoProductSemanticsCertification::derive(
            &self.operation_inventory,
            &self.route_inventory,
        );
        let requirements = WorthServerProductOperationRuntimeArtifactRequirements::new(vec![
            self.query_dependency_requirement_row(),
            self.operation_registry_requirement_row(),
            self.request_contract_requirement_row(),
            authority_footprint_requirement_row(&self.route_inventory),
            authorization_posture_requirement_row(&self.route_inventory),
            support_posture_requirement_row(&self.route_inventory),
            precondition_posture_requirement_row(&editor_readiness),
            self.planner_requirement_row(&editor_readiness),
            self.scheduler_requirement_row(&editor_readiness),
            self.product_adapter_requirement_row(),
            self.product_session_requirement_row(),
            self.route_assembly_requirement_row(),
            self.product_editor_readiness_requirement_row(&editor_readiness),
            self.no_product_semantics_requirement_row(&no_product_semantics),
        ]);
        let support_row = WorthServerProductOperationRuntimeSupportRow::new(requirements);
        WorthServerProductOperationRuntimeCertification::new(
            support_row,
            editor_readiness,
            no_product_semantics,
        )
    }

    fn query_dependency_requirement_row(&self) -> WorthServerProductOperationRuntimeRequirementRow {
        let receipt = self.query_dependency_audit.run();
        let ready = receipt.is_runtime_ready_for_phase_one();
        build_requirement_row(
            "query-dependency-audit",
            ready,
            receipt.audit_digest(),
            format!(
                "ordinary_rows={};blocked={};legacy={};folklore={};scope={}",
                receipt.support_posture().ordinary_row_count(),
                receipt.support_posture().blocked_row_count(),
                receipt.support_posture().legacy_assumption_row_count(),
                receipt.support_posture().local_folklore_row_count(),
                receipt.support_posture().unclassified_scope_row_count(),
            ),
        )
    }

    fn operation_registry_requirement_row(
        &self,
    ) -> WorthServerProductOperationRuntimeRequirementRow {
        let ready = self
            .operation_inventory
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.family(),
                    WorthServerOperationFamily::ProductApplicationRead
                        | WorthServerOperationFamily::ProductApplicationMutation
                        | WorthServerOperationFamily::ProductSessionCoordination
                )
            })
            .all(|row| {
                row.enabled()
                    && row
                        .exposed_surfaces()
                        .contains(&WorthServerSurfaceFamily::WorthNative)
                    && row
                        .exposed_surfaces()
                        .contains(&WorthServerSurfaceFamily::CompatHttp)
            });
        let digest = self
            .operation_inventory
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{:?}:{}:{:?}",
                    row.family(),
                    row.enabled(),
                    row.exposed_surfaces()
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        build_requirement_row(
            "operation-registry",
            ready,
            digest,
            "product families stay generic and surface-exposed",
        )
    }

    fn request_contract_requirement_row(&self) -> WorthServerProductOperationRuntimeRequirementRow {
        let product_rows = self
            .route_inventory
            .rows()
            .iter()
            .filter(|row| row.operation_name().is_some())
            .collect::<Vec<_>>();
        let ready = product_rows
            .iter()
            .all(|row| row.payload_schema_identity().is_some() && row.support_row().is_some());
        let digest = product_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.method(),
                    row.path(),
                    row.payload_schema_identity().unwrap_or("missing"),
                    row.support_row().unwrap_or("missing")
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        build_requirement_row(
            "request-contract",
            ready,
            digest,
            "semantic routes preserve payload schema and support-row pass-through",
        )
    }

    fn planner_requirement_row(
        &self,
        editor_readiness: &WorthServerProductEditorReadinessCertification,
    ) -> WorthServerProductOperationRuntimeRequirementRow {
        let ready = !editor_readiness
            .missing_proof_labels()
            .iter()
            .any(|label| label == "route-parity");
        build_requirement_row(
            "operation-planner",
            ready,
            editor_readiness.canonical_digest(),
            "route/direct lowering parity must remain canonical",
        )
    }

    fn scheduler_requirement_row(
        &self,
        editor_readiness: &WorthServerProductEditorReadinessCertification,
    ) -> WorthServerProductOperationRuntimeRequirementRow {
        let ready = !editor_readiness
            .missing_proof_labels()
            .iter()
            .any(|label| label == "shared-read-certification" || label == "mutation-certification");
        build_requirement_row(
            "operation-scheduler",
            ready,
            editor_readiness.canonical_digest(),
            "shared-read and mutation hostility proofs must both be present",
        )
    }

    fn product_adapter_requirement_row(&self) -> WorthServerProductOperationRuntimeRequirementRow {
        let ready = self
            .product_adapter_receipts
            .iter()
            .any(|receipt| has_editor_like_operation_set(receipt.operation_names()));
        let digest = self
            .product_adapter_receipts
            .iter()
            .map(WorthServerProductAdapterRegistrationReceipt::canonical_digest)
            .collect::<Vec<_>>()
            .join("|");
        build_requirement_row(
            "product-adapter",
            ready,
            digest,
            "editor-like adapter declarations must be registered through the server boundary",
        )
    }

    fn product_session_requirement_row(&self) -> WorthServerProductOperationRuntimeRequirementRow {
        let required = [
            "product_session.open_mutation",
            "product_session.open_preview",
            "product_session.close",
        ];
        let ready = required.iter().all(|operation_name| {
            self.route_inventory.rows().iter().any(|row| {
                row.operation_name() == Some(*operation_name)
                    && row.operation_family()
                        == Some(WorthServerOperationFamily::ProductSessionCoordination)
            })
        });
        let digest = self
            .route_inventory
            .rows()
            .iter()
            .filter_map(|row| {
                row.operation_name()
                    .filter(|name| name.starts_with("product_session."))
                    .map(|name| format!("{}:{}", row.path(), name))
            })
            .collect::<Vec<_>>()
            .join("|");
        build_requirement_row(
            "product-session",
            ready,
            digest,
            "server-owned session coordination routes must remain declared and generic",
        )
    }

    fn route_assembly_requirement_row(&self) -> WorthServerProductOperationRuntimeRequirementRow {
        let required = [
            "product_editor.render",
            "product_editor.select",
            "product_editor.available_actions",
            "product_editor.apply",
            "product_editor.finalize",
        ];
        let ready = required.iter().all(|operation_name| {
            self.route_inventory
                .rows()
                .iter()
                .any(|row| row.operation_name() == Some(*operation_name))
        });
        let digest = self
            .route_inventory
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}",
                    row.method(),
                    row.path(),
                    row.operation_family()
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        build_requirement_row(
            "route-assembly",
            ready,
            digest,
            "operation-declared editor routes must be present without route-local semantics",
        )
    }

    fn product_editor_readiness_requirement_row(
        &self,
        editor_readiness: &WorthServerProductEditorReadinessCertification,
    ) -> WorthServerProductOperationRuntimeRequirementRow {
        build_requirement_row(
            "product-editor-readiness",
            editor_readiness.is_ready(),
            editor_readiness.canonical_digest(),
            format!(
                "missing={}",
                editor_readiness.missing_proof_labels().join(",")
            ),
        )
    }

    fn no_product_semantics_requirement_row(
        &self,
        no_product_semantics: &WorthServerNoProductSemanticsCertification,
    ) -> WorthServerProductOperationRuntimeRequirementRow {
        build_requirement_row(
            "no-product-semantics",
            no_product_semantics.is_ready(),
            no_product_semantics.canonical_digest(),
            no_product_semantics.summary(),
        )
    }
}

fn has_editor_like_operation_set(operation_names: &[String]) -> bool {
    let required = [
        "product_editor.render",
        "product_editor.select",
        "product_editor.available_actions",
        "product_editor.apply",
        "product_editor.finalize",
    ];
    required
        .iter()
        .all(|required_name| operation_names.iter().any(|name| name == required_name))
}
