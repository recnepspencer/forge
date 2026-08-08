//! Receipt-derived evidence required to admit an undo correction.

use worth_query_installation::facade::{
    InstalledCorrectionMechanism, WorthQueryInstalledAftermathContract,
};

use crate::domain_computation::primary_graph::{
    WorthQueryRetainedGovernedInput, WorthQueryTouchedRecordIdentity,
};

use super::governed_input::require_original_governed_input;
use super::recovery_handle::WorthQueryRecoveryHandleBinding;
use super::{
    WorthQueryRetainedPreImage, WorthQueryUndoDenial, WorthQueryUndoDenialKind,
    WorthQueryUndoDerivedRequest,
};

pub(super) struct WorthQueryPreparedUndoEvidence {
    pub(super) touched_records: Vec<WorthQueryTouchedRecordIdentity>,
    pub(super) retained_preimage: Option<WorthQueryRetainedPreImage>,
    pub(super) retained_governed_input: WorthQueryRetainedGovernedInput,
}

pub(super) fn prepare_undo_evidence(
    binding: &WorthQueryRecoveryHandleBinding,
    aftermath: &WorthQueryInstalledAftermathContract,
    request: WorthQueryUndoDerivedRequest,
) -> Result<WorthQueryPreparedUndoEvidence, WorthQueryUndoDenial> {
    let mutation_work = binding.mutation_work().ok_or_else(|| {
        WorthQueryUndoDenial::new(match request {
            WorthQueryUndoDerivedRequest::RecordedInverse
            | WorthQueryUndoDerivedRequest::Compensation => {
                WorthQueryUndoDenialKind::TouchedRecordsRequired
            }
            WorthQueryUndoDerivedRequest::Reconciliation => WorthQueryUndoDenialKind::Stale,
        })
    })?;
    let touched_records = mutation_work.touched_records().to_vec();
    require_touched_records(request, &touched_records)?;
    let retained_preimage =
        retain_required_preimage(binding, aftermath, request, &touched_records)?;
    let retained_governed_input =
        require_original_governed_input(binding.retained_governed_input())?.clone();
    Ok(WorthQueryPreparedUndoEvidence {
        touched_records,
        retained_preimage,
        retained_governed_input,
    })
}

fn require_touched_records(
    request: WorthQueryUndoDerivedRequest,
    touched_records: &[WorthQueryTouchedRecordIdentity],
) -> Result<(), WorthQueryUndoDenial> {
    if matches!(
        request,
        WorthQueryUndoDerivedRequest::RecordedInverse | WorthQueryUndoDerivedRequest::Compensation
    ) && touched_records.is_empty()
    {
        return Err(WorthQueryUndoDenial::new(
            WorthQueryUndoDenialKind::TouchedRecordsRequired,
        ));
    }
    Ok(())
}

fn retain_required_preimage(
    binding: &WorthQueryRecoveryHandleBinding,
    aftermath: &WorthQueryInstalledAftermathContract,
    request: WorthQueryUndoDerivedRequest,
    touched_records: &[WorthQueryTouchedRecordIdentity],
) -> Result<Option<WorthQueryRetainedPreImage>, WorthQueryUndoDenial> {
    if request != WorthQueryUndoDerivedRequest::RecordedInverse {
        return Ok(None);
    }
    let preimage = binding.retained_preimage().cloned().ok_or_else(|| {
        WorthQueryUndoDenial::new(WorthQueryUndoDenialKind::RetainedPreImageRequired)
    })?;
    let Some(InstalledCorrectionMechanism::RecordedInverse(inverse)) = aftermath.mechanism() else {
        return Err(WorthQueryUndoDenial::new(
            WorthQueryUndoDenialKind::LoweringCorrespondenceRequired,
        ));
    };
    if inverse
        .preimage_demand()
        .field_slots()
        .iter()
        .any(|slot| preimage.field(slot).is_none())
    {
        return Err(WorthQueryUndoDenial::new(
            WorthQueryUndoDenialKind::RetainedPreImageRequired,
        ));
    }
    let _resolved = inverse.lowering_correspondence().resolved();
    require_exact_preimage_target(&preimage, touched_records)?;
    Ok(Some(preimage))
}

pub(crate) fn require_exact_preimage_target(
    preimage: &WorthQueryRetainedPreImage,
    touched_records: &[WorthQueryTouchedRecordIdentity],
) -> Result<(), WorthQueryUndoDenial> {
    let Some(target) = preimage.target_record() else {
        return Err(WorthQueryUndoDenial::new(
            WorthQueryUndoDenialKind::RetainedPreImageRequired,
        ));
    };
    if !touched_records
        .iter()
        .any(|touched| touched.record() == target)
    {
        return Err(WorthQueryUndoDenial::new(
            WorthQueryUndoDenialKind::TouchedRecordsRequired,
        ));
    }
    Ok(())
}
