use worth_query_decl::facade::application_aftermath::{
    DeclaredPreImageDemand, DeclaredPreImageLocus,
};
use worth_query_decl::facade::{worth_query_aspect, worth_query_entity, worth_query_field};

struct Schema;
worth_query_entity!(Account in Schema);
worth_query_aspect!(State in Schema, Account; identity = AspectIdentity(0x91611027), revision = AspectContractRevision(1),);
worth_query_field!(Status in Schema, Account, State: u64, read_only, no_equality);

fn main() {
    let demand =
        DeclaredPreImageDemand::new([DeclaredPreImageLocus::from_field(Status::reference())], 64)
            .unwrap();
    assert_eq!(demand.loci()[0].entity(), "Account");
}
