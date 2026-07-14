use super::{
    query_family::{AuthoredQuery, QueryAuthoringFamily, QueryBuilder},
    RawAuthoredQuery, RootEntityKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionFamily;

impl QueryAuthoringFamily for CollectionFamily {
    fn initialize(root: RootEntityKey) -> RawAuthoredQuery {
        RawAuthoredQuery::collection(root)
    }
}

pub type CollectionAuthoredQuery = AuthoredQuery<CollectionFamily>;
pub type CollectionQueryBuilder = QueryBuilder<CollectionFamily>;
