use super::{
    prepare_primitive_construction_policy_pressure_report_bundle,
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureDeltaCase,
};

#[test]
fn policy_pressure_bundle_binds_direct_and_same_setup_delta_truth() {
    let bundle = prepare_primitive_construction_policy_pressure_report_bundle().expect("bundle");

    assert_eq!(
        bundle.required_direct_cases(),
        &[
            PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
            PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity,
            PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
            PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity,
            PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst,
            PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
            PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst,
        ]
    );
    assert_eq!(
        bundle.required_delta_cases(),
        &[
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
        ]
    );
    assert_eq!(
        bundle.direct_report().report_digest(),
        bundle.delta_report().direct_report().report_digest()
    );
    assert_eq!(
        bundle.truth().required_direct_cases(),
        bundle.required_direct_cases()
    );
    assert_eq!(
        bundle.truth().required_delta_cases(),
        bundle.required_delta_cases()
    );
    assert_eq!(
        bundle
            .direct_report()
            .row(PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap)
            .expect("direct aggressive row")
            .row_digest(),
        bundle
            .delta_report()
            .row(PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity)
            .expect("delta high fidelity row")
            .left_row()
            .row_digest()
    );
    assert_ne!(
        bundle.report_digest(),
        bundle.direct_report().report_digest()
    );
    assert_ne!(
        bundle.report_digest(),
        bundle.delta_report().report_digest()
    );
}
