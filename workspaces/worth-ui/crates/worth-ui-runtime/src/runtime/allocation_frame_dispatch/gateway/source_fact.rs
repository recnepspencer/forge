#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFrameQuerySettlementPosture {
    Settled,
    Partial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFrameQueryWarningPosture {
    None,
    QueryContextRowBound,
    PreviewDerivedContext,
    QueryContextRowBoundAndPreviewDerivedContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFrameSourceFactPosture {
    HostMeasurement,
    QueryProjection {
        settlement: UiAllocationFrameQuerySettlementPosture,
        warnings: UiAllocationFrameQueryWarningPosture,
    },
    Interaction,
    DurableResize,
}

/// Move-only source truth retained in the sealed frame for Phase 5 consumption.
///
/// ```compile_fail
/// use worth_ui_runtime::runtime::UiAllocationFrameSourceFact;
///
/// fn source_truth_cannot_be_duplicated(fact: UiAllocationFrameSourceFact) {
///     let _duplicate = fact.clone();
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum UiAllocationFrameSourceFact {
    HostMeasurement(crate::host::UiAdmittedHostMeasurement),
    QueryProjection {
        source: worth_ui_query_binding::WorthUiQueryMeasurementFactSettlement,
        posture: UiAllocationFrameQuerySettlementPosture,
        warnings: UiAllocationFrameQueryWarningPosture,
    },
    Interaction(crate::runtime::WorthUiAdmittedTransientInteraction),
    DurableResize(crate::runtime::WorthUiAdmittedDurableResizeSourceFact),
}

impl UiAllocationFrameSourceFact {
    pub(in crate::runtime::allocation_frame_dispatch) fn posture(
        &self,
    ) -> UiAllocationFrameSourceFactPosture {
        match self {
            Self::HostMeasurement(_) => UiAllocationFrameSourceFactPosture::HostMeasurement,
            Self::QueryProjection {
                posture, warnings, ..
            } => UiAllocationFrameSourceFactPosture::QueryProjection {
                settlement: *posture,
                warnings: *warnings,
            },
            Self::Interaction(_) => UiAllocationFrameSourceFactPosture::Interaction,
            Self::DurableResize(_) => UiAllocationFrameSourceFactPosture::DurableResize,
        }
    }
}

#[cfg(test)]
impl Clone for UiAllocationFrameSourceFact {
    fn clone(&self) -> Self {
        match self {
            Self::HostMeasurement(source) => Self::HostMeasurement(source.clone()),
            Self::QueryProjection {
                source,
                posture,
                warnings,
            } => Self::QueryProjection {
                source: source.clone(),
                posture: *posture,
                warnings: *warnings,
            },
            Self::Interaction(source) => Self::Interaction(*source),
            Self::DurableResize(source) => Self::DurableResize(source.clone()),
        }
    }
}
