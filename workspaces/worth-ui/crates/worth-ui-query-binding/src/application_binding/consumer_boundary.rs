use worth_query::facade::foundation::ObservationLaneWitness;
use worth_query::facade::installed::operation::{
    WorthQueryConsumerBoundaryRequirements, WorthQueryConsumerProjectionContractDenial,
};
use worth_query::facade::installed::WorthQueryOperationBindingDenial;

use super::{WorthUiBoundSnapshotMeasurement, WorthUiQueryOperatingWorldGateway};
use super::{WorthUiSnapshotNativeRequest, WorthUiSnapshotNativeRequestDenial};
use crate::WorthUiQueryViewShape;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiQueryDenialPresentation {
    Hidden,
    AdvisoryText,
    StructuredStatus,
}

impl WorthUiQueryDenialPresentation {
    pub const fn hidden() -> Self {
        Self::Hidden
    }

    pub const fn advisory_text() -> Self {
        Self::AdvisoryText
    }

    pub const fn structured_status() -> Self {
        Self::StructuredStatus
    }

    pub const fn digest_basis(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::AdvisoryText => "advisory_text",
            Self::StructuredStatus => "structured_status",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryAllocationDetail {
    BorrowedFactSlice,
    OwnedMaterialization,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryInspectionRelevance {
    Omitted,
    Relevant,
}

/// UI-owned requirements adjacent to, but never merged into, Query's
/// consumer boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryConsumerRequirements {
    query_boundary: WorthQueryConsumerBoundaryRequirements,
    allocation_detail: WorthUiQueryAllocationDetail,
    result_shape: WorthUiQueryViewShape,
    denial_presentation: WorthUiQueryDenialPresentation,
    inspection_relevance: WorthUiQueryInspectionRelevance,
}

impl WorthUiQueryConsumerRequirements {
    pub const fn new(
        query_boundary: WorthQueryConsumerBoundaryRequirements,
        allocation_detail: WorthUiQueryAllocationDetail,
        result_shape: WorthUiQueryViewShape,
        denial_presentation: WorthUiQueryDenialPresentation,
        inspection_relevance: WorthUiQueryInspectionRelevance,
    ) -> Self {
        Self {
            query_boundary,
            allocation_detail,
            result_shape,
            denial_presentation,
            inspection_relevance,
        }
    }

    pub const fn query_boundary(self) -> WorthQueryConsumerBoundaryRequirements {
        self.query_boundary
    }

    pub const fn allocation_detail(self) -> WorthUiQueryAllocationDetail {
        self.allocation_detail
    }

    pub const fn result_shape(self) -> WorthUiQueryViewShape {
        self.result_shape
    }

    pub const fn denial_presentation(self) -> WorthUiQueryDenialPresentation {
        self.denial_presentation
    }

    pub const fn inspection_relevance(self) -> WorthUiQueryInspectionRelevance {
        self.inspection_relevance
    }
}

#[derive(Debug)]
pub enum WorthUiSnapshotConsumerPreparationDenial {
    ResultShapeMismatch {
        installed: WorthUiQueryViewShape,
        requested: WorthUiQueryViewShape,
    },
    Binding(Box<WorthQueryOperationBindingDenial>),
    ConsumerContract(WorthQueryConsumerProjectionContractDenial),
    NativeRequest(WorthUiSnapshotNativeRequestDenial),
}

/// One bound snapshot operation with its one Query-minted consumer contract.
///
/// Query's boundary owns Query support and coarse downstream posture. The UI
/// requirements remain an adjacent artifact and cannot rewrite that contract.
pub struct WorthUiPreparedSnapshotConsumer {
    reference: crate::WorthUiInstalledQueryBindingReference,
    bound: WorthUiBoundSnapshotMeasurement<ObservationLaneWitness>,
    native_request: WorthUiSnapshotNativeRequest,
    requirements: WorthUiQueryConsumerRequirements,
}

impl WorthUiQueryOperatingWorldGateway<'_> {
    pub fn prepare_snapshot_consumer(
        self,
        requirements: WorthUiQueryConsumerRequirements,
    ) -> Result<WorthUiPreparedSnapshotConsumer, WorthUiSnapshotConsumerPreparationDenial> {
        let installed_shape = self.reference.definition().shape();
        if installed_shape != requirements.result_shape() {
            return Err(
                WorthUiSnapshotConsumerPreparationDenial::ResultShapeMismatch {
                    installed: installed_shape,
                    requested: requirements.result_shape(),
                },
            );
        }
        let reference = self.reference.clone();
        let bound = self
            .bind_snapshot()
            .map_err(WorthUiSnapshotConsumerPreparationDenial::Binding)?;
        let consumer = bound
            .consumer_projection_contract()
            .map_err(WorthUiSnapshotConsumerPreparationDenial::ConsumerContract)?
            .with_downstream_requirements(requirements.query_boundary());
        let native_request = WorthUiSnapshotNativeRequest::from_consumer(consumer)
            .map_err(WorthUiSnapshotConsumerPreparationDenial::NativeRequest)?;
        Ok(WorthUiPreparedSnapshotConsumer {
            reference,
            bound,
            native_request,
            requirements,
        })
    }
}

impl WorthUiPreparedSnapshotConsumer {
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.reference
    }

    pub fn query_boundary_requirements(&self) -> WorthQueryConsumerBoundaryRequirements {
        self.requirements.query_boundary()
    }

    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.requirements
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::WorthUiInstalledQueryBindingReference,
        WorthUiBoundSnapshotMeasurement<ObservationLaneWitness>,
        WorthUiSnapshotNativeRequest,
        WorthUiQueryConsumerRequirements,
    ) {
        (
            self.reference,
            self.bound,
            self.native_request,
            self.requirements,
        )
    }
}
