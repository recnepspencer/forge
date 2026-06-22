use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_motion_posture::authoring::PlanarMotionPostureEntry;
use crate::bindings::query_native_planar_motion_posture::domain::PlanarMotionPostureQueryDomain;
use crate::bindings::query_native_planar_motion_posture::inspection::{
    PlanarMotionPostureInspectionKind, PlanarMotionPostureInspectionRow,
};
use crate::planar_contracts::motion_posture::{
    PlanarMotionPostureCounters, PlanarMotionPostureReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarMotionPostureFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PlanarMotionPostureFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_motion_posture_facts<WC>(
    entry: &PlanarMotionPostureEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarMotionPostureQueryDomain, WC>,
) -> Result<PlanarMotionPostureReceipt, PlanarMotionPostureFactError>
where
    WC: ForgeQueryDomainOperatingContext<PlanarMotionPostureQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let rows = PlanarMotionPostureInspectionRow::from_basis(&basis);
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let retained_motion_digest =
                PlanarMotionPostureReceipt::retained_motion_digest_for(&basis);
            Ok(PlanarMotionPostureReceipt::new(
                basis,
                declaration_digest,
                envelope_digest,
                retained_motion_digest,
                counters(&rows),
            ))
        }
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(PlanarMotionPostureFactError::outcome_not_bound(&posture))
        }
    }
}

fn counters(rows: &[PlanarMotionPostureInspectionRow]) -> PlanarMotionPostureCounters {
    let motion = rows
        .iter()
        .filter(|row| row.kind() == PlanarMotionPostureInspectionKind::MotionStep)
        .count();
    let rotation = rows
        .iter()
        .filter(|row| row.kind() == PlanarMotionPostureInspectionKind::Rotation)
        .count();
    let cancellation = rows
        .iter()
        .filter(|row| row.kind() == PlanarMotionPostureInspectionKind::Cancellation)
        .count();
    let signal = rows
        .iter()
        .filter(|row| row.kind() == PlanarMotionPostureInspectionKind::SignalCompatibility)
        .count();
    PlanarMotionPostureCounters::certified(motion, rotation, cancellation, signal)
}
