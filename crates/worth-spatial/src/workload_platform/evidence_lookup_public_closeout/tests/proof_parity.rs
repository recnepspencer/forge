use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SpatialCompiledProductConsumer,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout, EvidenceLookupPublicCloseoutDisposition,
};
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;

#[test]
fn receipt_proof_rows_match_real_family_lowering_and_cutover_proof() {
    let family_catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let spatial_catalog = current_spatial_compiled_product_family_catalog();
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");

    for row in closeout.family_stage_rows() {
        let EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
            selected_lookup_plan_digest,
            lookup_execution_receipt_digest,
            lookup_product_output_digest,
        } = row.disposition()
        else {
            continue;
        };

        let family = family_catalog
            .family_by_identity(row.family_identity())
            .expect("declared family for closeout row");
        let path = admit_current_family_stage_cutover_path(&family_catalog, family, row.stage())
            .expect("current cutover path");
        let proof = path
            .prove_for_family(row.family_identity())
            .expect("family cutover proof");
        let lowered_identity = select_spatial_compiled_product_family(
            &spatial_catalog,
            admit_spatial_compiled_product_input(
                &spatial_catalog,
                SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
                    SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout,
                    path.selected_plan(),
                    path.index_product(),
                ),
            )
            .expect("public closeout admitted input")
            .family_admitted_input(),
        )
        .expect("selected spatial family")
        .compile_product_identity()
        .expect("lowered spatial family identity");

        assert_eq!(row.family_declaration_digest(), family.declaration_digest());
        assert_eq!(
            row.stage_receipt_family_identity(),
            family
                .stage_applicability()
                .stage_receipt_family_identity()
                .digest()
        );
        assert_eq!(
            row.query_import_evidence_digest(),
            family.query_posture().imported_evidence_digest()
        );
        assert_eq!(
            row.spatial_touch_digest(),
            Some(proof.spatial_touch_digest())
        );
        assert_eq!(
            row.spatial_compiled_product_identity_digest(),
            Some(
                lowered_identity
                    .compiled_product_identity()
                    .identity_digest()
            )
        );
        assert_eq!(
            row.spatial_equivalence_policy_identity_digest(),
            Some(
                lowered_identity
                    .equivalence_policy_identity()
                    .identity_digest()
            )
        );
        assert_eq!(
            row.selected_lookup_plan_digest(),
            Some(selected_lookup_plan_digest.as_str())
        );
        assert_eq!(
            selected_lookup_plan_digest,
            path.selected_plan().selected_plan_digest()
        );
        assert_eq!(
            row.lookup_execution_receipt_digest(),
            Some(lookup_execution_receipt_digest.as_str())
        );
        assert_eq!(
            lookup_execution_receipt_digest,
            path.execution_receipt().execution_receipt_digest()
        );
        assert_eq!(
            row.lookup_product_output_digest(),
            Some(lookup_product_output_digest.as_str())
        );
        assert_eq!(
            lookup_product_output_digest,
            path.execution_receipt().lookup_product_output_digest()
        );
    }
}
