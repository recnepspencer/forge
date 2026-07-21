use worth_query::facade::{domain, read};

use crate::WorthUiDomainEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSnapshotMeasurement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSnapshotMeasurementFamily;

impl
    domain::WorthQueryExecutableDomainOperation<
        WorthUiDomainEntry,
        WorthUiSnapshotMeasurementFamily,
    > for WorthUiSnapshotMeasurement
{
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}
