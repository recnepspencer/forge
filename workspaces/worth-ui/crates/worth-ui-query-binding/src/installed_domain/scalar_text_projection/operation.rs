use worth_query::facade::{domain, read};

use crate::WorthUiDomainEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiScalarTextProjection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiScalarTextProjectionFamily;

impl
    domain::WorthQueryExecutableDomainOperation<
        WorthUiDomainEntry,
        WorthUiScalarTextProjectionFamily,
    > for WorthUiScalarTextProjection
{
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}
