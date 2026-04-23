use forge_query::facade::{
    admit_query_subscription, QuerySubscriptionAdmissionBudget,
    QuerySubscriptionDeclarationArtifact,
};

fn main() {
    let declaration: Option<QuerySubscriptionDeclarationArtifact> = None;
    let budget = QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1);
    let _admission = admit_query_subscription(declaration.unwrap(), budget);
}
