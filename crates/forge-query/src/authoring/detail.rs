use super::{
    query_family::{AuthoredQuery, QueryAuthoringFamily, QueryBuilder},
    RawAuthoredQuery, RootEntityKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailFamily;

impl QueryAuthoringFamily for DetailFamily {
    fn initialize(root: RootEntityKey) -> RawAuthoredQuery {
        RawAuthoredQuery::detail(root)
    }
}

pub type DetailAuthoredQuery = AuthoredQuery<DetailFamily>;
pub type DetailQueryBuilder = QueryBuilder<DetailFamily>;
