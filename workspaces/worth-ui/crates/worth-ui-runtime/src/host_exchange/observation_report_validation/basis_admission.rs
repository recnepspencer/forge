use super::sequence_coverage::UiSequenceCoveredObservationBatch;
use super::{UiHostObservationFrameRelation, UiHostObservationReportDenial};

pub(super) struct UiBasisAdmittedObservationBatch {
    batch: UiSequenceCoveredObservationBatch,
    relation: UiHostObservationFrameRelation,
    observation_basis: crate::mounting::UiMountedObservationBasisLease,
}

struct UiPresentedBasisCoordinates {
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    instance: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    receipt: Option<worth_ui_host_contract::UiMountedNodeReceiptIdentity>,
}

impl UiBasisAdmittedObservationBatch {
    pub(super) fn admit(
        batch: UiSequenceCoveredObservationBatch,
        retention: &crate::mounting::UiMountedFrameRetentionCoordinator,
        host_session: u64,
        existing_basis: Option<crate::mounting::UiMountedObservationBasisLease>,
    ) -> Result<Self, UiHostObservationReportDenial> {
        if batch.core().host_session() != host_session {
            return Err(UiHostObservationReportDenial::ForeignHostSession);
        }
        let relation = classify_basis(&batch, retention)?;
        let observation_basis = match existing_basis {
            Some(existing) => existing,
            None => retention
                .acquire_observation_basis(batch.core().frame())
                .map_err(map_retention_denial)?,
        };
        Ok(Self {
            batch,
            relation,
            observation_basis,
        })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        UiSequenceCoveredObservationBatch,
        UiHostObservationFrameRelation,
        crate::mounting::UiMountedObservationBasisLease,
    ) {
        (self.batch, self.relation, self.observation_basis)
    }
}

fn map_retention_denial(
    denial: crate::mounting::UiMountedObservationBasisRetentionDenial,
) -> UiHostObservationReportDenial {
    match denial {
        crate::mounting::UiMountedObservationBasisRetentionDenial::FrameTransitionInFlight => {
            UiHostObservationReportDenial::FrameTransitionInFlight
        }
        crate::mounting::UiMountedObservationBasisRetentionDenial::UnknownFrame => {
            UiHostObservationReportDenial::UnknownFrame
        }
        crate::mounting::UiMountedObservationBasisRetentionDenial::ExpiredFrame => {
            UiHostObservationReportDenial::ExpiredFrame
        }
        crate::mounting::UiMountedObservationBasisRetentionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        } => UiHostObservationReportDenial::ObservationBasisCapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        },
        crate::mounting::UiMountedObservationBasisRetentionDenial::AccountingOverflow => {
            UiHostObservationReportDenial::ObservationBasisAccountingOverflow
        }
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
                presentation: core.presentation(),
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
                presentation: core.presentation(),
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
        coordinates.presentation,
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
        Err(crate::mounting::UiPresentedFrameBasisDenial::PresentationEpochMismatch) => {
            Err(UiHostObservationReportDenial::PresentationEpochMismatch)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::InstanceNotPresented) => {
            Err(UiHostObservationReportDenial::MountedInstanceNotPresented)
        }
        Err(crate::mounting::UiPresentedFrameBasisDenial::NodeReceiptMismatch) => {
            Err(UiHostObservationReportDenial::NodeReceiptMismatch)
        }
    }
}
