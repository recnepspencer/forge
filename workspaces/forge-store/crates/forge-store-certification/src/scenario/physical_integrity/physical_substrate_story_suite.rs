use crate::{
    PhysicalScenarioDefinition, PhysicalScenarioQualityHarness, PhysicalStoryStep,
    PhysicalSubstrateCertificationDenial, PhysicalSubstrateCloseoutStoryReport,
    PhysicalSubstrateCloseoutStoryRow, PhysicalSubstrateLane,
};

pub(crate) fn story_reports(
) -> Result<Vec<PhysicalSubstrateCloseoutStoryReport>, PhysicalSubstrateCertificationDenial> {
    Ok(vec![
        PhysicalSubstrateCloseoutStoryReport::from_transcript(
            PhysicalSubstrateCloseoutStoryRow::PhysicalSubstrateStoryTranscript,
            &story_transcript(physical_substrate_story_definition()?)?,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::StoryEvidenceRejected)?,
        PhysicalSubstrateCloseoutStoryReport::from_transcript(
            PhysicalSubstrateCloseoutStoryRow::LegacyOverclaimRejected,
            &story_transcript(legacy_overclaim_definition()?)?,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::StoryEvidenceRejected)?,
    ])
}

fn story_transcript(
    definition: PhysicalScenarioDefinition,
) -> Result<crate::PhysicalStoryTranscript, PhysicalSubstrateCertificationDenial> {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let plan = harness
        .lower(definition)
        .map_err(|_| PhysicalSubstrateCertificationDenial::StoryPlanRejected)?;
    Ok(harness.transcribe(harness.judge(harness.observe(harness.execute(plan)))))
}

fn physical_substrate_story_definition(
) -> Result<PhysicalScenarioDefinition, PhysicalSubstrateCertificationDenial> {
    PhysicalScenarioDefinition::story("physical_substrate_closeout_story")
        .physical_substrate_lane(PhysicalSubstrateLane::HappyAuthority)
        .proves_law("page segment extent substrate survives physical closeout")
        .step(PhysicalStoryStep::GivenCleanPhysicalStore)
        .step(PhysicalStoryStep::WhenAuthoritativeRecordIsAppended)
        .step(PhysicalStoryStep::WhenStoreClosesAndReopensFromBytes)
        .step(PhysicalStoryStep::ThenRecordLocatesByPhysicalReference)
        .step(PhysicalStoryStep::ThenShortcutCertificationFails)
        .define()
        .map_err(|_| PhysicalSubstrateCertificationDenial::StoryDefinitionRejected)
}

fn legacy_overclaim_definition(
) -> Result<PhysicalScenarioDefinition, PhysicalSubstrateCertificationDenial> {
    PhysicalScenarioDefinition::story("legacy_backend_overclaim_denial")
        .physical_substrate_lane(PhysicalSubstrateLane::LegacyOverclaim)
        .proves_law("legacy heap file and sqlite claims cannot become platform grade")
        .step(PhysicalStoryStep::GivenLegacyBackendClaim)
        .step(PhysicalStoryStep::WhenLegacyClaimAsksForPlatformGrade)
        .step(PhysicalStoryStep::ThenForbiddenClaimIsDenied)
        .define()
        .map_err(|_| PhysicalSubstrateCertificationDenial::StoryDefinitionRejected)
}
