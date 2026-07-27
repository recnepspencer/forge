use worth_query_decl::facade::application_schema::{
    ApplicationEntityRef, ApplicationRelationRef, TypedReadDeclarationBuilder,
};

struct Schema;
struct Principal;
struct Account;
struct PersonalOwner;

fn main() {
    let account = ApplicationEntityRef::<Schema, Account>::from_schema_identifier("Account");
    let personal_owner =
        ApplicationRelationRef::<Schema, PersonalOwner, Principal, Account>::from_schema_identifiers(
            "PersonalOwner",
            "Principal",
            "Account",
        );
    let _ = TypedReadDeclarationBuilder::new(account).traverse(personal_owner);
}
