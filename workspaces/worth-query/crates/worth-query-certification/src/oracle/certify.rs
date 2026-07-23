use super::{
    WorthQueryCertificationProvider, WorthQueryCertificationReport,
    WorthQueryCertificationScenarioReport, WorthQueryHostileCertificationProvider,
    WorthQueryHostileCertificationReport,
};
use crate::evidence::{WorthQueryCertificationDenialEvidence, WorthQueryCertificationObservation};
use crate::scenario::{
    canonical_hostile_matrix, WorthQueryCertificationHostileAttack, WorthQueryCertificationSuite,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryCertificationFailure {
    InvalidProviderIdentity(String),
    SameProviderIdentity,
    ProviderExecution {
        provider: String,
        scenario: String,
        detail: String,
    },
    OracleMismatch {
        provider: String,
        scenario: String,
        expected: WorthQueryCertificationObservation,
        observed: WorthQueryCertificationObservation,
    },
    HostileExecution {
        provider: String,
        attack: WorthQueryCertificationHostileAttack,
        detail: String,
    },
    HostileEvidenceMismatch {
        provider: String,
        attack: WorthQueryCertificationHostileAttack,
        expected: WorthQueryCertificationDenialEvidence,
        observed: WorthQueryCertificationDenialEvidence,
    },
}

pub fn certify_provider_pair(
    suite: &WorthQueryCertificationSuite,
    first: &mut impl WorthQueryCertificationProvider,
    second: &mut impl WorthQueryCertificationProvider,
) -> Result<WorthQueryCertificationReport, WorthQueryCertificationFailure> {
    let identities = [
        first.provider_identity().to_owned(),
        second.provider_identity().to_owned(),
    ];
    for identity in &identities {
        validate_identity(identity)?;
    }
    if identities[0] == identities[1] {
        return Err(WorthQueryCertificationFailure::SameProviderIdentity);
    }

    let mut reports = Vec::with_capacity(suite.scenarios().len());
    for scenario in suite.scenarios() {
        let first_observation = execute(&identities[0], first, scenario)?;
        let second_observation = execute(&identities[1], second, scenario)?;
        for (provider, observation) in [
            (identities[0].as_str(), &first_observation),
            (identities[1].as_str(), &second_observation),
        ] {
            if observation != scenario.oracle() {
                return Err(WorthQueryCertificationFailure::OracleMismatch {
                    provider: provider.to_owned(),
                    scenario: scenario.id().to_owned(),
                    expected: scenario.oracle().clone(),
                    observed: observation.clone(),
                });
            }
        }
        reports.push(WorthQueryCertificationScenarioReport::new(
            scenario.id().to_owned(),
            scenario.kind(),
            scenario.journey_checkpoints().clone(),
            first_observation.counters().clone(),
        ));
    }
    Ok(WorthQueryCertificationReport::new(identities, reports))
}

pub fn certify_hostile_provider(
    provider: &mut impl WorthQueryHostileCertificationProvider,
) -> Result<WorthQueryHostileCertificationReport, WorthQueryCertificationFailure> {
    let identity = provider.provider_identity().to_owned();
    validate_identity(&identity)?;
    let hostile = canonical_hostile_matrix();
    for case in &hostile {
        attack(&identity, provider, case.attack(), case.expected())?;
    }
    Ok(WorthQueryHostileCertificationReport::new(
        identity,
        hostile.len(),
    ))
}

fn execute(
    provider_identity: &str,
    provider: &mut impl WorthQueryCertificationProvider,
    scenario: &crate::scenario::WorthQueryCertificationScenario,
) -> Result<WorthQueryCertificationObservation, WorthQueryCertificationFailure> {
    provider
        .execute(scenario)
        .map_err(|detail| WorthQueryCertificationFailure::ProviderExecution {
            provider: provider_identity.to_owned(),
            scenario: scenario.id().to_owned(),
            detail,
        })
}

fn attack(
    provider_identity: &str,
    provider: &mut impl WorthQueryHostileCertificationProvider,
    hostile: WorthQueryCertificationHostileAttack,
    expected: &WorthQueryCertificationDenialEvidence,
) -> Result<(), WorthQueryCertificationFailure> {
    let observed = provider.attack(hostile).map_err(|detail| {
        WorthQueryCertificationFailure::HostileExecution {
            provider: provider_identity.to_owned(),
            attack: hostile,
            detail,
        }
    })?;
    if &observed != expected {
        return Err(WorthQueryCertificationFailure::HostileEvidenceMismatch {
            provider: provider_identity.to_owned(),
            attack: hostile,
            expected: expected.clone(),
            observed,
        });
    }
    Ok(())
}

fn validate_identity(identity: &str) -> Result<(), WorthQueryCertificationFailure> {
    if identity.is_empty()
        || !identity
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-._".contains(&byte))
    {
        return Err(WorthQueryCertificationFailure::InvalidProviderIdentity(
            identity.to_owned(),
        ));
    }
    Ok(())
}
