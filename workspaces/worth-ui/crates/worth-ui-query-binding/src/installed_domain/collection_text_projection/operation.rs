use worth_query::facade::{domain, read};

use crate::WorthUiDomainEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCollectionTextProjection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCollectionTextProjectionFamily;

impl
    domain::WorthQueryExecutableDomainOperation<
        WorthUiDomainEntry,
        WorthUiCollectionTextProjectionFamily,
    > for WorthUiCollectionTextProjection
{
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}
