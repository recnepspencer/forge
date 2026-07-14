use worth_query::facade::runtime::{QuerySubscriptionFamily, QuerySubscriptionSupportClass, QuerySubscriptionSupportSubject};

fn main() {
    let _ = QuerySubscriptionSupportSubject {
        support_class: QuerySubscriptionSupportClass::Declaration,
        family: QuerySubscriptionFamily::DetailExact,
        declaration_digest: String::new(),
        admission_digest: None,
        source_digest: String::new(),
        digest: String::new(),
    };
}
