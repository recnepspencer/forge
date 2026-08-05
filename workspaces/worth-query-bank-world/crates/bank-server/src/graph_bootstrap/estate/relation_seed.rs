use bank_domain::schema::BankSchema;
use worth_query_host::facade::{
    declaration::application_schema::ApplicationRelationRef,
    primary_graph::{
        WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphBootstrap,
        WorthQueryPrimaryGraphInstallationDenial,
    },
};

use super::super::entity_key;

pub(super) fn bind<Relation, From, To>(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    relation: ApplicationRelationRef<BankSchema, Relation, From, To>,
    key: String,
    from: String,
    to: String,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    graph.bind_relation(WorthQueryApplicationRelationSeed::new(
        relation,
        key,
        entity_key(from),
        entity_key(to),
    ))
}
