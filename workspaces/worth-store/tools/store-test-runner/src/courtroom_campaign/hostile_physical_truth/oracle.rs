use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkEvidenceDigest, PhysicalWorkHostileCurrentTruth,
    PhysicalWorkHostileTruthComparison, PhysicalWorkHostileTruthScenario,
    PhysicalWorkOracleEvidence,
};

use super::{
    artifact_oracle,
    offline_protocol::OfflineObservation,
    writer_protocol::{CheckpointObservation, SeedObservation},
};

#[derive(Clone, Copy)]
pub(super) struct OraclePayloads<'payload> {
    pub(super) seed: &'payload [u8],
    pub(super) mutation: &'payload [u8],
}

pub(super) struct TruthOracleInput<'observation> {
    pub(super) scenario: PhysicalWorkHostileTruthScenario,
    pub(super) seed: &'observation SeedObservation,
    pub(super) baseline: &'observation OfflineObservation,
    pub(super) observed: &'observation OfflineObservation,
    pub(super) checkpoint: &'observation CheckpointObservation,
    pub(super) payloads: OraclePayloads<'observation>,
}

pub(super) fn compare(
    input: TruthOracleInput<'_>,
) -> Result<
    (
        PhysicalWorkHostileTruthComparison,
        PhysicalWorkOracleEvidence,
    ),
    String,
> {
    let baseline_expected = current_truth(
        input.baseline.current().store(),
        input.seed.generation(),
        input.seed.records(),
        [input.payloads.seed],
    )?;
    if input.seed.records() != 1 || input.baseline.current() != baseline_expected {
        return Err(format!(
            "{} baseline offline truth disagrees with the independent seed oracle",
            input.scenario.label()
        ));
    }
    let expected = select_expected_current(
        input.scenario,
        baseline_expected,
        input.observed.current(),
        input.payloads,
    )?;
    artifact_oracle::validate_mutation_coordination(
        input.baseline.artifacts(),
        input.seed.process(),
        "seeded baseline",
    )?;
    artifact_oracle::validate_mutation_coordination(
        input.observed.artifacts(),
        input.checkpoint.process(),
        input.scenario.label(),
    )?;
    artifact_oracle::validate_transition(input.scenario, input.baseline, input.observed)?;
    let comparison = PhysicalWorkHostileTruthComparison::new(
        baseline_expected,
        expected,
        input.observed.current(),
    );
    let oracle = PhysicalWorkOracleEvidence::new(
        format!(
            "courtroom-b:{}:independent-physical-truth",
            input.scenario.label()
        ),
        true,
        oracle_digest(&input)?,
    )
    .map_err(|denial| format!("Courtroom B oracle binding denied: {denial:?}"))?;
    Ok((comparison, oracle))
}

fn select_expected_current(
    scenario: PhysicalWorkHostileTruthScenario,
    baseline: PhysicalWorkHostileCurrentTruth,
    observed: PhysicalWorkHostileCurrentTruth,
    payloads: OraclePayloads<'_>,
) -> Result<PhysicalWorkHostileCurrentTruth, String> {
    if scenario != PhysicalWorkHostileTruthScenario::DuringRootPublication {
        return (observed == baseline).then_some(baseline).ok_or_else(|| {
            format!(
                "{} observed current truth does not match its independent oracle: \
                 expected={baseline:?}, observed={observed:?}",
                scenario.label(),
            )
        });
    }
    let generation = baseline.generation().saturating_add(1);
    let records = baseline.records().saturating_add(1);
    let seed_first = current_truth(
        baseline.store(),
        generation,
        records,
        [payloads.seed, payloads.mutation],
    )?;
    let mutation_first = current_truth(
        baseline.store(),
        generation,
        records,
        [payloads.mutation, payloads.seed],
    )?;
    [seed_first, mutation_first]
        .into_iter()
        .find(|candidate| *candidate == observed)
        .ok_or_else(|| {
            format!(
                "{} observed current truth is outside the independent persisted-record-order \
                 candidates: seed_first={seed_first:?}, mutation_first={mutation_first:?}, \
                 observed={observed:?}",
                scenario.label(),
            )
        })
}

fn current_truth<'payload>(
    store: [u8; 16],
    generation: u64,
    records: u64,
    payloads: impl IntoIterator<Item = &'payload [u8]>,
) -> Result<PhysicalWorkHostileCurrentTruth, String> {
    let mut digest = Sha256::new();
    let mut payload_bytes = 0_u64;
    for payload in payloads {
        digest.update((payload.len() as u64).to_le_bytes());
        digest.update(payload);
        payload_bytes = payload_bytes.saturating_add(payload.len() as u64);
    }
    PhysicalWorkHostileCurrentTruth::new(
        store,
        generation,
        records,
        payload_bytes,
        evidence_digest(digest.finalize().into(), "payload oracle")?,
    )
    .map_err(|denial| format!("current-truth oracle denied: {denial:?}"))
}

fn oracle_digest(input: &TruthOracleInput<'_>) -> Result<PhysicalWorkEvidenceDigest, String> {
    let mut digest = Sha256::new();
    digest.update(input.scenario.label().as_bytes());
    digest.update(input.checkpoint.checkpoint().as_bytes());
    digest.update(input.checkpoint.detail().as_bytes());
    digest.update(b"independent-payload-oracles-v1");
    for payload in [input.payloads.seed, input.payloads.mutation] {
        digest.update((payload.len() as u64).to_le_bytes());
        digest.update(payload);
    }
    update_observation(&mut digest, input.baseline);
    update_observation(&mut digest, input.observed);
    evidence_digest(digest.finalize().into(), "Courtroom B oracle")
}

fn update_observation(digest: &mut Sha256, observation: &OfflineObservation) {
    let current = observation.current();
    digest.update(current.store());
    digest.update(current.generation().to_le_bytes());
    digest.update(current.records().to_le_bytes());
    digest.update(current.payload_bytes().to_le_bytes());
    digest.update(current.payload_digest().bytes());
    for artifact in observation.artifacts() {
        digest.update((artifact.path().len() as u64).to_le_bytes());
        digest.update(artifact.path().as_bytes());
        digest.update(artifact.byte_length().to_le_bytes());
        digest.update(artifact.digest());
        digest.update((artifact.prefix().len() as u64).to_le_bytes());
        digest.update(artifact.prefix());
        digest.update([u8::from(artifact.is_recovery_obligation())]);
    }
}

fn evidence_digest(bytes: [u8; 32], label: &str) -> Result<PhysicalWorkEvidenceDigest, String> {
    PhysicalWorkEvidenceDigest::new(bytes)
        .ok_or_else(|| format!("{label} cannot be an all-zero digest"))
}

#[cfg(test)]
mod tests {
    use super::{
        current_truth, select_expected_current, OraclePayloads, PhysicalWorkHostileTruthScenario,
    };

    #[test]
    fn publication_oracle_accepts_both_persisted_record_orders() {
        let baseline = current_truth([7; 16], 2, 1, [b"seed".as_slice()]).unwrap();
        for payloads in [
            [b"seed".as_slice(), b"mutation".as_slice()],
            [b"mutation".as_slice(), b"seed".as_slice()],
        ] {
            let observed = current_truth([7; 16], 3, 2, payloads).unwrap();
            assert_eq!(
                select_expected_current(
                    PhysicalWorkHostileTruthScenario::DuringRootPublication,
                    baseline,
                    observed,
                    OraclePayloads {
                        seed: b"seed",
                        mutation: b"mutation",
                    },
                )
                .unwrap(),
                observed,
            );
        }
    }

    #[test]
    fn publication_oracle_rejects_a_foreign_payload_multiset() {
        let baseline = current_truth([7; 16], 2, 1, [b"seed".as_slice()]).unwrap();
        let foreign =
            current_truth([7; 16], 3, 2, [b"seed".as_slice(), b"foreign".as_slice()]).unwrap();
        assert!(select_expected_current(
            PhysicalWorkHostileTruthScenario::DuringRootPublication,
            baseline,
            foreign,
            OraclePayloads {
                seed: b"seed",
                mutation: b"mutation",
            },
        )
        .is_err());
    }
}
