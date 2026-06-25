mod construction_proof;
mod counters;
mod obligation_posture;
mod query_posture;

pub use construction_proof::WorthUiPrimitiveConstructionGraphProof;
pub(crate) use construction_proof::WorthUiPrimitiveFamilyAdmissionDigests;
pub use counters::WorthUiPrimitiveGraphCounters;
pub use obligation_posture::{
    WorthUiPrimitiveConstructionObligationKind, WorthUiPrimitiveConstructionObligationPosture,
    WorthUiPrimitiveConstructionObligationRow,
};
pub use query_posture::WorthUiPrimitiveQueryPosture;

use crate::runtime::{
    WorthUiQueryGraphExecutionReceipt, WorthUiValidatedProjectionDependencyContract,
};

pub(crate) fn prove_primitive_construction_graph(
    surface_id: &str,
    component_id: &str,
    dependency_contract: WorthUiValidatedProjectionDependencyContract,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    digests: WorthUiPrimitiveFamilyAdmissionDigests,
) -> WorthUiPrimitiveConstructionGraphProof {
    WorthUiPrimitiveConstructionGraphProof::prove(
        surface_id,
        component_id,
        dependency_contract,
        query_graph_execution,
        digests,
    )
}

#[cfg(test)]
mod tests {
    use crate::capability::SurfaceId;
    use crate::runtime::{
        WorthUiPrimitiveConstructionRequest, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
        WorthUiRuntimeGraphAuthority,
    };

    use super::{
        WorthUiPrimitiveConstructionObligationPosture, WorthUiPrimitiveFamilyAdmissionDigests,
    };

    #[test]
    fn construction_graph_proof_publishes_primitive_construction_fact() {
        let surface_id =
            SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
        let plan = WorthUiRuntimeGraphAuthority::new()
            .plan_primitive_construction(WorthUiPrimitiveConstructionRequest::for_surface(
                surface_id.clone(),
            ))
            .expect("primitive construction plan");
        let proof = super::prove_primitive_construction_graph(
            surface_id.as_str(),
            "worth.component.primitive_proof",
            plan.dependency_contract().clone(),
            plan.query_graph_execution().clone(),
            WorthUiPrimitiveFamilyAdmissionDigests {
                primitive: 1,
                flow: 2,
                content: 3,
                appearance_state: 4,
                interaction: 5,
                event_geometry: 6,
            },
        );

        assert!(proof
            .published_facts()
            .contains(&WorthUiRuntimeFactId::primitive_construction(
                surface_id.as_str()
            )));
        assert!(proof
            .dependency_contract()
            .dependencies()
            .facts()
            .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PrimitiveConstruction));
        assert_eq!(proof.counters().source_reparse_count(), 0);
        assert_eq!(proof.counters().renderer_prop_parse_count(), 0);
    }

    #[test]
    fn construction_graph_proof_records_query_selected_obligations() {
        let surface_id =
            SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
        let plan = WorthUiRuntimeGraphAuthority::new()
            .plan_primitive_construction(WorthUiPrimitiveConstructionRequest::for_surface(
                surface_id.clone(),
            ))
            .expect("primitive construction plan");
        let proof = super::prove_primitive_construction_graph(
            surface_id.as_str(),
            "worth.component.primitive_proof",
            plan.dependency_contract().clone(),
            plan.query_graph_execution().clone(),
            WorthUiPrimitiveFamilyAdmissionDigests {
                primitive: 10,
                flow: 20,
                content: 30,
                appearance_state: 40,
                interaction: 50,
                event_geometry: 60,
            },
        );

        assert!(proof.obligation_rows().iter().any(|row| {
            row.posture() == WorthUiPrimitiveConstructionObligationPosture::Selected
        }));
        assert_eq!(
            proof.obligation_rows().len(),
            proof.query_graph_execution().selected_obligation_count()
        );
        assert_eq!(proof.query_posture().token(), "projection_facts_required");
    }
}
