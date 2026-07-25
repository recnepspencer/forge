use super::sequence_coverage::UiSequenceCoveredObservationBatch;
use super::{UiHostObservationFrameRelation, UiHostObservationReportDenial};

pub(super) struct UiBasisAdmittedObservationBatch {
    batch: UiSequenceCoveredObservationBatch,
    relation: UiHostObservationFrameRelation,
}

struct UiPresentedBasisCoordinates {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    instance: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    receipt: Option<worth_ui_host_contract::UiMountedNodeReceiptIdentity>,
}

impl UiBasisAdmittedObservationBatch {
    pub(super) fn admit(
        batch: UiSequenceCoveredObservationBatch,
        retention: &crate::mounting::UiMountedFrameRetentionCoordinator,
        host_session: u64,
    ) -> Result<Self, UiHostObservationReportDenial> {
        if batch.core().host_session() != host_session {
            return Err(UiHostObservationReportDenial::ForeignHostSession);
        }
        let relation = classify_basis(&batch, retention)?;
        Ok(Self { batch, relation })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        UiSequenceCoveredObservationBatch,
        UiHostObservationFrameRelation,
    ) {
        (self.batch, self.relation)
    }
}

fn classify_basis(
    batch: &UiSequenceCoveredObservationBatch,
    retention: &crate::mounting::UiMountedFrameRetentionCoordinator,
) -> Result<UiHostObservationFrameRelation, UiHostObservationReportDenial> {
    let core = batch.core();
    let mut relation = None;
    for report in batch.reports() {
        let mounted = report.mounted_basis();
        let next = classify_presented_basis(
            retention,
            UiPresentedBasisCoordinates {
                frame: core.frame(),
                binding: core.binding(),
                instance: mounted.map(|basis| basis.instance()),
                receipt: mounted.map(|basis| basis.node_receipt()),
            },
        )?;
        if relation.is_some_and(|current| current != next) {
            return Err(UiHostObservationReportDenial::MalformedBatch);
        }
        relation = Some(next);
    }
    match relation {
        Some(relation) => Ok(relation),
        None => classify_presented_basis(
            retention,
            UiPresentedBasisCoordinates {
                frame: core.frame(),
                binding: core.binding(),
                instance: None,
                receipt: None,
            },
        ),
    }
}

fn classify_presented_basis(
    retention: &crate::mounting::UiMountedFrameRetentionCoordinator,
    coordinates: UiPresentedBasisCoordinates,
) -> Result<UiHostObservationFrameRelation, UiHostObservationReportDenial> {
    match retention.classify(
        coordinates.frame,
        coordinates.binding,
        coordinates.instance,
        coordinates.receipt,
    ) {
        Ok(crate::mounting::UiPresentedFrameBasisRelation::Current) => {
            Ok(UiHostObservationFrameRelation::CurrentPresented)
        }
        Ok(crate::mounting::UiPresentedFrameBasisRelation::Retained) => {
            Ok(UiHostObservationFrameRelation::SupersededPresented)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::Expired) => {
            Err(UiHostObservationReportDenial::ExpiredFrame)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::Unknown) => {
            Err(UiHostObservationReportDenial::UnknownFrame)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::BindingNotPresented) => {
            Err(UiHostObservationReportDenial::BindingNotPresented)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::InstanceNotPresented) => {
            Err(UiHostObservationReportDenial::MountedInstanceNotPresented)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::NodeReceiptMismatch) => {
            Err(UiHostObservationReportDenial::NodeReceiptMismatch)
        }
    }
}
