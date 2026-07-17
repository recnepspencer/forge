use worth_foundational::canonicalization_api::lower_lane::basis::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalizationRuleVersion,
};
use worth_foundational::InternedString;

use crate::{
    OracleVerdictBasis, PhysicalOracleNonClaim, PhysicalProofOracleVerdict,
    PhysicalProofOracleVerdictKind,
};

use super::super::ExecutedTranscriptParts;
use super::observation_entries::observation_entries;

pub(super) const TRANSCRIPT_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.simulation.transcript");
const TRANSCRIPT_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-simulation-transcript-field");

pub(super) fn transcript_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    let mut entries = replay_basis_entries(parts);
    entries.extend(
        parts
            .oracle_verdicts()
            .iter()
            .enumerate()
            .flat_map(|(index, verdict)| oracle_verdict_entries(index, verdict)),
    );
    entries
}

pub(super) fn replay_basis_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            "transcript.scenario.identity",
            hex(parts.plan().scenario_identity().digest_bytes()),
        ),
        text_entry(
            "transcript.plan.identity",
            hex(parts.plan().identity().digest_bytes()),
        ),
        text_entry(
            "transcript.schedule.identity",
            hex(parts.schedule().identity().digest_bytes()),
        ),
        text_entry(
            "transcript.seed",
            parts.schedule().seed().value().to_string(),
        ),
        text_entry(
            "transcript.profile",
            format!("{:?}", parts.plan().profile()),
        ),
        text_entry("transcript.fixture.name", parts.fixture_manifest().name()),
        text_entry(
            "transcript.fixture.digest",
            parts.fixture_manifest().semantic_digest(),
        ),
        text_entry(
            "transcript.fixture.source.root",
            parts
                .fixture_manifest()
                .source()
                .root_reference()
                .to_string(),
        ),
        text_entry(
            "transcript.fixture.profile",
            format!("{:?}", parts.fixture_manifest().profile()),
        ),
        text_entry(
            "transcript.fixture.scale.bytes",
            parts
                .fixture_manifest()
                .scale()
                .declared_store_bytes()
                .to_string(),
        ),
        text_entry(
            "transcript.fixture.scale.blob_bytes",
            parts.fixture_manifest().scale().blob_bytes().to_string(),
        ),
        text_entry(
            "transcript.fixture.scale.wal_tail_bytes",
            parts
                .fixture_manifest()
                .scale()
                .wal_tail_bytes()
                .to_string(),
        ),
        text_entry(
            "transcript.fixture.scale.damaged_region_bytes",
            parts
                .fixture_manifest()
                .scale()
                .damaged_region_bytes()
                .to_string(),
        ),
        text_entry(
            "transcript.fixture.identity",
            hex(&parts.fixture_manifest().evidence_identity()),
        ),
        text_entry(
            "transcript.trace.observer",
            format!("{:?}", parts.trace().observer()),
        ),
    ];
    entries.extend(fixture_boundary_entries(parts));
    entries.extend(runtime_trace_entries(parts));
    entries.extend(observation_entries(parts));
    entries.extend(driver_profile_entries(parts));
    entries.extend(actor_step_entries(parts));
    entries.extend(fault_entries(parts));
    entries.extend(counter_entries(parts));
    entries
}

pub(super) fn transcript_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.physical.simulation.transcript.v2")
        .expect("transcript canonicalization version is static")
}

fn fixture_boundary_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    let capability_entries = parts
        .fixture_manifest()
        .capability_declarations()
        .iter()
        .enumerate()
        .map(|(index, capability)| {
            text_entry(
                format!("transcript.fixture.capability.{index:04}"),
                format!("{:?}", capability.mutation_boundary()),
            )
        });
    let mutation_entries = parts
        .fixture_manifest()
        .mutation_boundaries()
        .iter()
        .enumerate()
        .map(|(index, boundary)| {
            text_entry(
                format!("transcript.fixture.mutation_boundary.{index:04}"),
                format!("{boundary:?}"),
            )
        });
    capability_entries.chain(mutation_entries).collect()
}

fn runtime_trace_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    let trace = parts.trace().runtime_trace();
    let mut entries = vec![
        text_entry(
            "transcript.runtime_trace.driver",
            format!("{:?}", trace.driver()),
        ),
        text_entry(
            "transcript.runtime_trace.boundary",
            format!("{:?}", trace.boundary()),
        ),
        text_entry(
            "transcript.runtime_trace.backend_profile",
            format!("{:?}", trace.backend_profile()),
        ),
    ];
    entries.extend(
        trace
            .yieldpoints()
            .iter()
            .enumerate()
            .map(|(index, yieldpoint)| {
                text_entry(
                    format!("transcript.runtime_trace.yieldpoint.{index:04}"),
                    format!("{}:{:?}", yieldpoint.name(), yieldpoint.seam()),
                )
            }),
    );
    entries
}

fn driver_profile_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    parts
        .driver_profile_rows()
        .into_iter()
        .enumerate()
        .map(|(index, row)| text_entry(format!("transcript.driver_profile.{index:04}"), row))
        .collect()
}

fn actor_step_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    parts
        .schedule()
        .actor_steps()
        .iter()
        .enumerate()
        .map(|(index, step)| {
            text_entry(
                format!("transcript.actor_step.{index:04}"),
                format!(
                    "{}:{}:{}",
                    step.step_index(),
                    step.actor_id(),
                    step.yieldpoint()
                ),
            )
        })
        .collect()
}

fn fault_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    parts
        .fault_events()
        .iter()
        .enumerate()
        .map(|(index, fault)| {
            text_entry(
                format!("transcript.fault.{index:04}"),
                format!(
                    "{:?}:{:?}:{:?}:{:?}",
                    fault.kind(),
                    fault.required_seam(),
                    fault.locus(),
                    fault
                ),
            )
        })
        .collect()
}

fn counter_entries(parts: &ExecutedTranscriptParts) -> Vec<CanonicalBasisEntry> {
    parts
        .counter_receipt()
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            text_entry(
                format!("transcript.counter.{index:04}"),
                format!(
                    "{:?}:{:?}:{}",
                    row.kind(),
                    row.strength(),
                    row.observed_count()
                ),
            )
        })
        .collect()
}

fn oracle_verdict_entries(
    index: usize,
    verdict: &PhysicalProofOracleVerdict,
) -> Vec<CanonicalBasisEntry> {
    let basis = verdict.basis();
    let mut entries = vec![
        text_entry(
            format!("transcript.oracle.{index:04}.family"),
            format!("{:?}", verdict.family()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.oracle"),
            format!("{:?}", verdict.oracle()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.kind"),
            verdict_kind_token(verdict.kind()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.non_claims"),
            non_claims_token(verdict.non_claims()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.replay_basis_digest"),
            verdict
                .transcript_replay_basis_digest()
                .map(hex)
                .unwrap_or_else(|| "none".to_owned()),
        ),
    ];
    entries.extend(oracle_basis_entries(index, basis));
    entries
}

fn oracle_basis_entries(index: usize, basis: &OracleVerdictBasis) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            format!("transcript.oracle.{index:04}.basis.scenario"),
            hex(basis.scenario_identity().digest_bytes()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.basis.plan"),
            hex(basis.plan_identity().digest_bytes()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.basis.observer"),
            format!("{:?}", basis.observer()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.basis.runtime_trace_present"),
            basis.runtime_trace_present().to_string(),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.basis.independent_verifier"),
            basis
                .independent_verifier()
                .map(|observation| format!("{:?}:{:?}", observation.seam(), observation.kind()))
                .unwrap_or_else(|| "none".to_owned()),
        ),
        text_entry(
            format!("transcript.oracle.{index:04}.basis.recovery_outcome"),
            basis
                .recovery_outcome()
                .map(|observation| format!("{:?}", observation.kind()))
                .unwrap_or_else(|| "none".to_owned()),
        ),
    ];
    entries.extend(basis.shortcut_rejections().iter().enumerate().map(
        |(shortcut_index, observation)| {
            text_entry(
                format!("transcript.oracle.{index:04}.basis.shortcut.{shortcut_index:04}"),
                format!("{:?}", observation.kind()),
            )
        },
    ));
    entries
}

pub(super) fn text_entry(
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        TRANSCRIPT_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        TRANSCRIPT_FIELD_KIND,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn verdict_kind_token(kind: PhysicalProofOracleVerdictKind) -> &'static str {
    match kind {
        PhysicalProofOracleVerdictKind::Satisfied => "satisfied",
        PhysicalProofOracleVerdictKind::Denied => "denied",
        PhysicalProofOracleVerdictKind::Deferred => "deferred",
        PhysicalProofOracleVerdictKind::Stale => "stale",
        PhysicalProofOracleVerdictKind::RebindRequired => "rebind-required",
        PhysicalProofOracleVerdictKind::Failed => "failed",
    }
}

fn non_claims_token(non_claims: &[PhysicalOracleNonClaim]) -> String {
    non_claims
        .iter()
        .map(|non_claim| format!("{non_claim:?}"))
        .collect::<Vec<_>>()
        .join("|")
}
