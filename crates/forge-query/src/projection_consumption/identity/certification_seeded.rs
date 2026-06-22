use super::scope::{certification_scope_encoder, compose_certification_sequence_digest, seal};
use crate::ForgeQueryEvidenceTag;

pub(crate) fn compose_seeded_replay_digest(
    seed: u64,
    scenario: &str,
    executed_artifact_digest: &str,
) -> String {
    seal(
        certification_scope_encoder("projection_consumption_seeded_replay_v1")
            .field_shape(ForgeQueryEvidenceTag::new("seed"), seed.to_string())
            .field_shape(ForgeQueryEvidenceTag::new("scenario"), scenario)
            .field_shape(
                ForgeQueryEvidenceTag::new("artifact"),
                executed_artifact_digest,
            ),
    )
}

fn compose_seeded_generator_choice_entry_digest(class: &str, choice: &str) -> String {
    seal(
        certification_scope_encoder("projection_consumption_seeded_generator_choice_entry_v1")
            .field_shape(ForgeQueryEvidenceTag::new("class"), class)
            .field_shape(ForgeQueryEvidenceTag::new("choice"), choice),
    )
}

pub(crate) fn compose_seeded_row_digest(
    seed: u64,
    replay_digest: &str,
    generator_choices: &[(&str, &str)],
) -> String {
    let choices = generator_choices
        .iter()
        .map(|(class, choice)| compose_seeded_generator_choice_entry_digest(class, choice))
        .collect::<Vec<_>>();
    seal(
        certification_scope_encoder("projection_consumption_seeded_row_v2")
            .field_shape(ForgeQueryEvidenceTag::new("seed"), seed.to_string())
            .field_shape(ForgeQueryEvidenceTag::new("replay"), replay_digest)
            .field_value_sequence(ForgeQueryEvidenceTag::new("generator_choice"), choices),
    )
}

pub(crate) fn compose_seeded_sequence_replay_check_digest(
    first_replay_digest: &str,
    stable_replay_digest: &str,
    stable_match: bool,
    changed_match: bool,
) -> String {
    seal(
        certification_scope_encoder("projection_consumption_seed_replay_v2")
            .field_shape(
                ForgeQueryEvidenceTag::new("first_replay"),
                first_replay_digest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("stable_replay"),
                stable_replay_digest,
            )
            .field_bool(ForgeQueryEvidenceTag::new("stable"), stable_match)
            .field_bool(ForgeQueryEvidenceTag::new("changed"), changed_match),
    )
}

pub(crate) fn compose_executed_lifecycle_artifact_digest(
    declaration_digest: &str,
    contract_digest: &str,
    fact_set_digest: &str,
    receipt_digest: &str,
    envelope_digest: &str,
) -> String {
    compose_certification_sequence_digest(
        "projection_consumption_executed_lifecycle_artifact_v1",
        "artifact",
        [
            declaration_digest,
            contract_digest,
            fact_set_digest,
            receipt_digest,
            envelope_digest,
        ],
    )
}
