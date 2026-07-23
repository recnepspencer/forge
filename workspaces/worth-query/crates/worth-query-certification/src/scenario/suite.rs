use super::{
    WorthQueryCertificationJourneyCheckpoint, WorthQueryCertificationScenario,
    WorthQueryCertificationScenarioKind,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryCertificationSuiteDenial {
    DuplicateScenarioIdentity(String),
    MissingScenarioKinds(BTreeSet<WorthQueryCertificationScenarioKind>),
    MissingJourneyCheckpoints(BTreeSet<WorthQueryCertificationJourneyCheckpoint>),
}

#[derive(Clone, Debug)]
pub struct WorthQueryCertificationSuite {
    scenarios: Vec<WorthQueryCertificationScenario>,
}

impl WorthQueryCertificationSuite {
    pub fn complete(
        scenarios: impl IntoIterator<Item = WorthQueryCertificationScenario>,
    ) -> Result<Self, WorthQueryCertificationSuiteDenial> {
        let scenarios = scenarios.into_iter().collect::<Vec<_>>();
        let mut identities = BTreeSet::new();
        for scenario in &scenarios {
            if !identities.insert(scenario.id().to_owned()) {
                return Err(
                    WorthQueryCertificationSuiteDenial::DuplicateScenarioIdentity(
                        scenario.id().to_owned(),
                    ),
                );
            }
        }
        let present = scenarios
            .iter()
            .map(|scenario| scenario.kind())
            .collect::<BTreeSet<_>>();
        let missing = WorthQueryCertificationScenarioKind::ALL
            .into_iter()
            .filter(|kind| !present.contains(kind))
            .collect::<BTreeSet<_>>();
        if !missing.is_empty() {
            return Err(WorthQueryCertificationSuiteDenial::MissingScenarioKinds(
                missing,
            ));
        }
        let covered = scenarios
            .iter()
            .flat_map(|scenario| scenario.required_journey_checkpoints().iter().copied())
            .collect::<BTreeSet<_>>();
        let missing = WorthQueryCertificationJourneyCheckpoint::ALL
            .into_iter()
            .filter(|checkpoint| !covered.contains(checkpoint))
            .collect::<BTreeSet<_>>();
        if !missing.is_empty() {
            return Err(WorthQueryCertificationSuiteDenial::MissingJourneyCheckpoints(missing));
        }
        Ok(Self { scenarios })
    }

    pub fn scenarios(&self) -> &[WorthQueryCertificationScenario] {
        &self.scenarios
    }
}
