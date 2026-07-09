use crate::s2_acceptance_suite_transcript::transcript_supports_acceptance_suite;
use crate::{
    BoundedMemoryResidencySuiteDenial, PhysicalProofOracleKind, PhysicalScenarioDriverKind,
    PhysicalScenarioObserverKind, PhysicalScenarioPlan, PhysicalStoryTranscript, RoadmapLaneFamily,
    S2AcceptanceSuiteKind,
};
use worth_store_test_support::LargeStorePressureClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessCloseoutEvidenceReport {
    lane_families: Vec<RoadmapLaneFamily>,
    pressure_classes: Vec<LargeStorePressureClass>,
    acceptance_suites: Vec<S2AcceptanceSuiteKind>,
    suite_transcripts: Vec<HarnessCloseoutTranscriptEvidence>,
    driver_families: Vec<PhysicalScenarioDriverKind>,
    observer_families: Vec<PhysicalScenarioObserverKind>,
    oracle_families: Vec<PhysicalProofOracleKind>,
    transcript_families: u32,
}

impl HarnessCloseoutEvidenceReport {
    pub fn from_harness_transcripts(
        transcripts: &[HarnessCloseoutTranscriptEvidence],
    ) -> Result<Self, BoundedMemoryResidencySuiteDenial> {
        if transcripts.is_empty() {
            return Err(BoundedMemoryResidencySuiteDenial::MissingHarnessEvidence);
        }
        let mut report = HarnessCloseoutEvidenceAccumulator::default();
        for transcript in transcripts {
            report.observe(transcript);
        }
        report.finish(transcripts.len() as u32)
    }

    pub const fn transcript_families(&self) -> u32 {
        self.transcript_families
    }

    pub fn lane_families(&self) -> &[RoadmapLaneFamily] {
        &self.lane_families
    }

    pub fn pressure_classes(&self) -> &[LargeStorePressureClass] {
        &self.pressure_classes
    }

    pub fn acceptance_suites(&self) -> &[S2AcceptanceSuiteKind] {
        &self.acceptance_suites
    }

    pub fn suite_transcripts(&self) -> &[HarnessCloseoutTranscriptEvidence] {
        &self.suite_transcripts
    }

    pub fn transcript_for_acceptance_suite(
        &self,
        suite: S2AcceptanceSuiteKind,
    ) -> Option<&HarnessCloseoutTranscriptEvidence> {
        self.suite_transcripts
            .iter()
            .find(|transcript| transcript.acceptance_suite() == suite)
    }

    pub fn contains_pressure_class(&self, pressure_class: LargeStorePressureClass) -> bool {
        self.pressure_classes.contains(&pressure_class)
    }

    pub fn contains_acceptance_suite(&self, suite: S2AcceptanceSuiteKind) -> bool {
        self.transcript_for_acceptance_suite(suite).is_some()
    }

    #[cfg(test)]
    pub(crate) fn without_acceptance_suite_for_test(self, suite: S2AcceptanceSuiteKind) -> Self {
        let transcripts: Vec<_> = self
            .suite_transcripts
            .into_iter()
            .filter(|transcript| transcript.acceptance_suite() != suite)
            .collect();
        Self::from_harness_transcripts(&transcripts).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessCloseoutTranscriptEvidence {
    acceptance_suite: S2AcceptanceSuiteKind,
    lane_family: RoadmapLaneFamily,
    pressure_class: Option<LargeStorePressureClass>,
    driver_families: Vec<PhysicalScenarioDriverKind>,
    observer_families: Vec<PhysicalScenarioObserverKind>,
    oracle_families: Vec<PhysicalProofOracleKind>,
}

impl HarnessCloseoutTranscriptEvidence {
    pub fn from_suite_plan_and_transcript(
        acceptance_suite: S2AcceptanceSuiteKind,
        plan: &PhysicalScenarioPlan,
        transcript: &PhysicalStoryTranscript,
    ) -> Result<Self, BoundedMemoryResidencySuiteDenial> {
        require_transcript_identity(plan, transcript)?;
        if !transcript_supports_acceptance_suite(acceptance_suite, plan, transcript) {
            return Err(BoundedMemoryResidencySuiteDenial::MissingHarnessEvidence);
        }
        Ok(Self {
            acceptance_suite,
            lane_family: transcript.plan_identity().lane_family(),
            pressure_class: plan
                .large_store_pressure_fixture()
                .map(|fixture| fixture.class()),
            driver_families: plan
                .driver_requirements()
                .iter()
                .map(|requirement| requirement.kind())
                .collect(),
            observer_families: transcript.observer_trace().observed_observers().to_vec(),
            oracle_families: transcript
                .judgments()
                .iter()
                .map(|judgment| judgment.oracle())
                .collect(),
        })
    }

    pub const fn acceptance_suite(&self) -> S2AcceptanceSuiteKind {
        self.acceptance_suite
    }

    pub const fn lane_family(&self) -> RoadmapLaneFamily {
        self.lane_family
    }

    pub const fn pressure_class(&self) -> Option<LargeStorePressureClass> {
        self.pressure_class
    }

    pub fn driver_families(&self) -> &[PhysicalScenarioDriverKind] {
        &self.driver_families
    }

    pub fn observer_families(&self) -> &[PhysicalScenarioObserverKind] {
        &self.observer_families
    }

    pub fn oracle_families(&self) -> &[PhysicalProofOracleKind] {
        &self.oracle_families
    }

    pub fn names_required_families(&self) -> bool {
        !self.driver_families.is_empty()
            && !self.observer_families.is_empty()
            && !self.oracle_families.is_empty()
    }
}

#[derive(Default)]
struct HarnessCloseoutEvidenceAccumulator {
    lane_families: Vec<RoadmapLaneFamily>,
    pressure_classes: Vec<LargeStorePressureClass>,
    acceptance_suites: Vec<S2AcceptanceSuiteKind>,
    suite_transcripts: Vec<HarnessCloseoutTranscriptEvidence>,
    driver_families: Vec<PhysicalScenarioDriverKind>,
    observer_families: Vec<PhysicalScenarioObserverKind>,
    oracle_families: Vec<PhysicalProofOracleKind>,
}

impl HarnessCloseoutEvidenceAccumulator {
    fn observe(&mut self, transcript: &HarnessCloseoutTranscriptEvidence) {
        push_unique(&mut self.lane_families, transcript.lane_family);
        push_unique(&mut self.acceptance_suites, transcript.acceptance_suite);
        self.suite_transcripts.push(transcript.clone());
        if let Some(pressure_class) = transcript.pressure_class {
            push_unique(&mut self.pressure_classes, pressure_class);
        }
        extend_unique(&mut self.driver_families, &transcript.driver_families);
        extend_unique(&mut self.observer_families, &transcript.observer_families);
        extend_unique(&mut self.oracle_families, &transcript.oracle_families);
    }

    fn finish(
        self,
        transcript_families: u32,
    ) -> Result<HarnessCloseoutEvidenceReport, BoundedMemoryResidencySuiteDenial> {
        if self.lane_families.is_empty()
            || self.driver_families.is_empty()
            || self.observer_families.is_empty()
            || self.oracle_families.is_empty()
            || self.acceptance_suites.is_empty()
            || self.suite_transcripts.is_empty()
            || self
                .suite_transcripts
                .iter()
                .any(|transcript| !transcript.names_required_families())
        {
            return Err(BoundedMemoryResidencySuiteDenial::MissingHarnessEvidence);
        }
        Ok(HarnessCloseoutEvidenceReport {
            lane_families: self.lane_families,
            pressure_classes: self.pressure_classes,
            acceptance_suites: self.acceptance_suites,
            suite_transcripts: self.suite_transcripts,
            driver_families: self.driver_families,
            observer_families: self.observer_families,
            oracle_families: self.oracle_families,
            transcript_families,
        })
    }
}

fn require_transcript_identity(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> Result<(), BoundedMemoryResidencySuiteDenial> {
    if transcript.plan_identity() == plan.identity()
        && !plan.driver_requirements().is_empty()
        && !transcript.observer_trace().observed_observers().is_empty()
        && !transcript.judgments().is_empty()
    {
        Ok(())
    } else {
        Err(BoundedMemoryResidencySuiteDenial::MissingHarnessEvidence)
    }
}

fn extend_unique<T: Copy + PartialEq>(values: &mut Vec<T>, candidates: &[T]) {
    for candidate in candidates {
        push_unique(values, *candidate);
    }
}

fn push_unique<T: Copy + PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}
