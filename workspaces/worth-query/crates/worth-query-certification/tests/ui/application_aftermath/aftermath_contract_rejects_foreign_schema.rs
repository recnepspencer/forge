use worth_query_decl::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract,
    DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand,
    DeclaredPreImageLocus, DeclaredRecordedInverse,
};

mod schema_a {
    use worth_query_decl::facade::{
        worth_query_aspect, worth_query_entity, worth_query_field, worth_query_operation,
    };

    pub struct Schema;
    pub struct Input;
    worth_query_entity!(pub Account in Schema);
    worth_query_aspect!(pub State in Schema, Account; identity = AspectIdentity(0x91611025), revision = AspectContractRevision(1),);
    worth_query_field!(pub Status in Schema, Account, State: u64, read_only, no_equality);
    worth_query_operation!(pub Operation(Input) in Schema);
}

mod schema_b {
    use worth_query_decl::facade::{worth_query_aspect, worth_query_entity, worth_query_field};

    pub struct Schema;
    worth_query_entity!(pub Account in Schema);
    worth_query_aspect!(pub State in Schema, Account; identity = AspectIdentity(0x91611026), revision = AspectContractRevision(1),);
    worth_query_field!(pub Status in Schema, Account, State: u64, read_only, no_equality);
}

fn foreign_contract() -> DeclaredApplicationAftermathContract<schema_b::Schema> {
    let demand = DeclaredPreImageDemand::new(
        [DeclaredPreImageLocus::from_field(
            schema_b::Status::reference(),
        )],
        64,
    )
    .unwrap();
    let inverse = DeclaredRecordedInverse::new(
        "restore-status",
        DeclaredLoweringCorrespondenceRef::new("status-inverse").unwrap(),
        DeclaredAftermathPostcondition::ExactPriorTruth,
        demand,
    )
    .unwrap();
    DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(inverse),
    )
}

fn main() {
    let _ = schema_a::Operation::reference()
        .definition()
        .no_external_effect()
        .aftermath(foreign_contract())
        .finish();
}
