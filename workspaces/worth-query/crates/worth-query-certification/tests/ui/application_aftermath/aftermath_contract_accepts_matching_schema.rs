use worth_query_decl::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract,
    DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand,
    DeclaredPreImageLocus, DeclaredRecordedInverse,
};
use worth_query_decl::facade::{
    worth_query_aspect, worth_query_entity, worth_query_field, worth_query_operation,
};

struct Schema;
struct Input;
worth_query_entity!(Account in Schema);
worth_query_aspect!(State in Schema, Account; identity = AspectIdentity(0x91611024), revision = AspectContractRevision(1),);
worth_query_field!(Status in Schema, Account, State: u64, read_only, no_equality);
worth_query_operation!(Operation(Input) in Schema);

fn main() {
    let demand =
        DeclaredPreImageDemand::new([DeclaredPreImageLocus::from_field(Status::reference())], 64)
            .unwrap();
    let inverse = DeclaredRecordedInverse::new(
        "restore-status",
        DeclaredLoweringCorrespondenceRef::new("status-inverse").unwrap(),
        DeclaredAftermathPostcondition::ExactPriorTruth,
        demand,
    )
    .unwrap();
    let contract = DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(inverse),
    );
    let _ = Operation::reference()
        .definition()
        .no_external_effect()
        .aftermath(contract)
        .finish();
}
