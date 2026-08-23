use std::collections::BTreeSet;

use worth_store::physical_runtime::{PhysicalWorkHostileCurrentTruth, PhysicalWorkRerunEvidence};

use super::super::{
    c7_crash_campaign::{C7CrashCampaignEvidence, C7CrashSeamEvidence},
    schedule::{C7DurabilityCrashSeam, SchedulePerturbationPlan},
};
use super::artifact_policy::{
    verify_durability_artifact_manifest, DurabilityArtifactManifestStage,
};
use super::c8_recovery;

pub(super) fn verify(
    campaign: &C7CrashCampaignEvidence,
    schedule: &SchedulePerturbationPlan,
) -> Result<(), String> {
    if campaign.cases().is_empty() {
        return Err("C.7 durability campaign omitted every boundary".to_owned());
    }
    let mut seams = BTreeSet::new();
    for case in campaign.cases() {
        if !seams.insert(case.seam()) {
            return Err(format!(
                "C.7 durability campaign duplicated `{}`",
                case.seam().label()
            ));
        }
        verify_case(case, schedule)?;
    }
    Ok(())
}

fn verify_case(
    case: &C7CrashSeamEvidence,
    schedule: &SchedulePerturbationPlan,
) -> Result<(), String> {
    if case.checkpoint_order() != schedule.durability_checkpoint_order() {
        return Err(format!(
            "C.7 boundary `{}` executed a foreign checkpoint schedule choice",
            case.seam().label()
        ));
    }
    verify_durability_artifact_manifest(
        case.baseline(),
        DurabilityArtifactManifestStage::CleanBaseline,
        0,
    )?;
    verify_durability_artifact_manifest(
        case.observed(),
        DurabilityArtifactManifestStage::PostBoundary,
        expected_recovery_obligations(case.seam()),
    )?;
    verify_truth(
        case.seam(),
        case.baseline().current(),
        case.observed().current(),
        case.observed().recovery_obligations(),
        case.reopen(),
    )?;
    c8_recovery::verify(case)?;
    verify_rerun(case.rerun(), schedule.seed().value(), case.seam())
}

fn expected_recovery_obligations(seam: C7DurabilityCrashSeam) -> u64 {
    if seam.interrupts_unsettled_media_effect() {
        1
    } else {
        0
    }
}

fn verify_rerun(
    rerun: &PhysicalWorkRerunEvidence,
    schedule_seed: u64,
    seam: C7DurabilityCrashSeam,
) -> Result<(), String> {
    require_argument_value(rerun, "--schedule-seed", &schedule_seed.to_string())?;
    require_argument_value(rerun, "--crash-seam", seam.label())
}

fn require_argument_value(
    rerun: &PhysicalWorkRerunEvidence,
    name: &str,
    expected: &str,
) -> Result<(), String> {
    let arguments = rerun.arguments();
    let matches = arguments
        .windows(2)
        .filter(|pair| pair[0].as_ref() == name && pair[1].as_ref() == expected)
        .count();
    if matches == 1
        && arguments
            .iter()
            .filter(|argument| argument.as_ref() == name)
            .count()
            == 1
    {
        Ok(())
    } else {
        Err(format!(
            "C.7 exact rerun omitted unique `{name} {expected}` arguments"
        ))
    }
}

fn verify_truth(
    seam: C7DurabilityCrashSeam,
    baseline: PhysicalWorkHostileCurrentTruth,
    observed: PhysicalWorkHostileCurrentTruth,
    observed_recovery_obligations: u64,
    reopen: worth_store::physical_runtime::PhysicalWorkFreshReopenEvidence,
) -> Result<(), String> {
    if baseline.store() != observed.store() || baseline.store() != reopen.identity().store() {
        return Err("C.7 boundary observations crossed Store identity".to_owned());
    }
    let successor = baseline.generation().saturating_add(1);
    let must_remain_baseline = !matches!(
        seam,
        C7DurabilityCrashSeam::AfterRootReplacementBeforeNamespaceDurability
            | C7DurabilityCrashSeam::AfterPhysicalDurabilityBeforeAcknowledgment
    );
    if must_remain_baseline && observed != baseline {
        return Err(format!(
            "{} exposed partially settled current truth",
            seam.label()
        ));
    }
    if seam == C7DurabilityCrashSeam::AfterPhysicalDurabilityBeforeAcknowledgment
        && (observed.generation() != successor
            || observed.records() != baseline.records().saturating_add(1))
    {
        return Err(
            "post-durability caller loss did not retain the completed physical effect".into(),
        );
    }
    if seam == C7DurabilityCrashSeam::AfterRootReplacementBeforeNamespaceDurability
        && ![baseline.generation(), successor].contains(&observed.generation())
    {
        return Err("root replacement exposed an impossible generation".into());
    }
    if reopen.posture().recovery_obligations() != observed_recovery_obligations
        || (observed_recovery_obligations != 0 && !reopen.posture().inspection_required())
    {
        return Err("fresh reopener did not preserve exact recovery-obligation posture".into());
    }
    if !reopen.posture().inspection_required()
        && (reopen.identity().generation() != observed.generation()
            || reopen.identity().records() != observed.records())
    {
        return Err("fresh reopener disagreed with independent current truth".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::expected_recovery_obligations;
    use crate::courtroom_campaign::bounded_residency_siege::schedule::C7DurabilityCrashSeam;

    #[test]
    fn unsettled_media_points_require_one_recovery_obligation() {
        for seam in C7DurabilityCrashSeam::ALL {
            if seam.interrupts_unsettled_media_effect() && expected_recovery_obligations(seam) != 1
            {
                panic!("MUTANT_PREDICATE:c7-post-boundary-recovery-residue-rejected");
            }
        }
    }

    #[test]
    fn settled_between_effect_points_require_zero_recovery_obligations() {
        for seam in C7DurabilityCrashSeam::ALL {
            if !seam.interrupts_unsettled_media_effect() && expected_recovery_obligations(seam) != 0
            {
                panic!("MUTANT_PREDICATE:c7-settled-boundary-residue-required");
            }
        }
    }
}
