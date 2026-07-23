use worth_query::facade::domain;

use crate::WorthUiDomainEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiMeasurementRecording;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiMeasurementRecordingFamily;

impl
    domain::WorthQueryExecutableDomainOperation<
        WorthUiDomainEntry,
        WorthUiMeasurementRecordingFamily,
    > for WorthUiMeasurementRecording
{
    type Input = ();
    type Output = ();
    type Publication = domain::WorthQueryTerminalOperation;
    type Execution = domain::WorthQueryWorkflowOperation;
}
