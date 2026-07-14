use super::{
    InterferenceAttribution, InterferenceCounterDenial, InterferenceCounterRow,
    LatencyEnvelopeClaim,
};

pub(super) fn require_claim_rows(
    claim: &LatencyEnvelopeClaim,
    rows: &[InterferenceCounterRow],
) -> Result<(), InterferenceCounterDenial> {
    for requirement in claim.requirements() {
        let row = rows
            .iter()
            .find(|row| row.name() == requirement.name())
            .ok_or(InterferenceCounterDenial::MissingCounter(
                requirement.name(),
            ))?;
        if row.profile_scope() != claim.profile_scope() {
            return Err(InterferenceCounterDenial::ProfileScopeMismatch);
        }
        if row.lane() != claim.lane() {
            return Err(InterferenceCounterDenial::LaneMismatch);
        }
        if !row.strength().satisfies(requirement.required_strength()) {
            return Err(InterferenceCounterDenial::InsufficientCounterStrength {
                counter: row.name(),
                required: requirement.required_strength(),
                actual: row.strength(),
            });
        }
        if requirement.attribution_required() && row.attribution().is_none() {
            return Err(InterferenceCounterDenial::MissingCausalAttribution(
                row.name(),
            ));
        }
    }
    Ok(())
}

pub(super) fn require_any_attribution(
    rows: &[InterferenceCounterRow],
) -> Result<(), InterferenceCounterDenial> {
    if rows
        .iter()
        .any(|row| row.value() > 0 && row.attribution().is_some())
    {
        Ok(())
    } else {
        Err(InterferenceCounterDenial::MissingPostAdmissionViolationAttribution)
    }
}

pub(super) fn require_violation_attribution(
    rows: &[InterferenceCounterRow],
) -> Result<(), InterferenceCounterDenial> {
    if rows.iter().any(|row| {
        row.value() > 0
            && matches!(
                row.attribution(),
                Some(
                    InterferenceAttribution::ExecutionViolation
                        | InterferenceAttribution::Backpressure(_)
                        | InterferenceAttribution::ForegroundWait
                        | InterferenceAttribution::FlushDelay
                        | InterferenceAttribution::SyncDebt
                        | InterferenceAttribution::PageCacheWait
                        | InterferenceAttribution::WorkerHandoffWait
                        | InterferenceAttribution::BackendContradictedWitness
                        | InterferenceAttribution::EnvelopeExceeded
                        | InterferenceAttribution::PolicyDebt
                        | InterferenceAttribution::BackgroundYield
                        | InterferenceAttribution::BackgroundDebt(_)
                )
            )
    }) {
        Ok(())
    } else {
        Err(InterferenceCounterDenial::MissingPostAdmissionViolationAttribution)
    }
}
