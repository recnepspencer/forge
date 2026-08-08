use worth_query_declaration::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract,
    DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand,
    DeclaredRecordedInverse,
};

pub(in super::super) fn aftermath() -> DeclaredApplicationAftermathContract {
    DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(
            DeclaredRecordedInverse::new(
                "test-wrong-field-inverse",
                DeclaredLoweringCorrespondenceRef::new("test-wrong-field-lowering").unwrap(),
                DeclaredAftermathPostcondition::ExactPriorTruth,
                DeclaredPreImageDemand::new(["AccountStatus"], 256).unwrap(),
            )
            .unwrap(),
        ),
    )
}
