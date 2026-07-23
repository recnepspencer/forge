use worth_query::facade::{domain, read};

pub(crate) fn measurement_allocation_operation(
) -> domain::WorthQueryDomainGraphReadOperationDefinition {
    domain::WorthQueryDomainGraphReadOperationDefinition::new(
        domain::WorthQueryDomainIdentityName::new("measurement-allocation")
            .expect("static Worth UI operation name must admit"),
        1,
    )
    .accepts_relation(
        read::RelationName::new("measurement.allocation")
            .expect("static Worth UI relation must admit"),
    )
}
