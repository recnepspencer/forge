use super::certification::compose_certification_row_digest;
use super::scope::{certification_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

pub(crate) fn compose_closeout_support_matrix_surface_row_digest(
    inventory_digest: &str,
    matrix_digest: &str,
    traceability_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "support_matrix_surface"),
            ("inventory", inventory_digest),
            ("matrix", matrix_digest),
            ("traceability", traceability_digest),
        ],
    )
}

pub(crate) fn compose_closeout_public_boundary_surface_row_digest(
    public_surface_digest: &str,
    negative_dx_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "public_boundary_surface"),
            ("public_surface", public_surface_digest),
            ("negative_dx", negative_dx_digest),
        ],
    )
}

pub(crate) fn compose_closeout_proof_shape_surface_row_digest(
    proof_shape_digest: &str,
    phase_progression_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "proof_shape_surface"),
            ("proof_shape", proof_shape_digest),
            ("phase_progression", phase_progression_digest),
        ],
    )
}

pub(crate) fn compose_closeout_forbidden_fallback_surface_row_digest(
    forbidden_fallback_digest: &str,
    total_occurrences: usize,
) -> String {
    seal(
        certification_scope_encoder("projection_consumption_certification_row_v1")
            .field_shape(
                ForgeQueryEvidenceTag::new("lane"),
                "forbidden_fallback_surface",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("forbidden_fallback"),
                forbidden_fallback_digest,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("total_occurrences"),
                total_occurrences,
            ),
    )
}

pub(crate) fn compose_closeout_dx_transcript_surface_row_digest(
    target_dx_digest: &str,
    golden_transcript_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "dx_transcript_surface"),
            ("target_dx", target_dx_digest),
            ("golden", golden_transcript_digest),
        ],
    )
}

pub(crate) fn compose_closeout_compile_fail_boundary_row_digest(
    compile_fail_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "compile_fail_boundary"),
            ("compile_fail", compile_fail_digest),
        ],
    )
}

pub(crate) fn compose_closeout_oracle_surface_row_digest(
    oracle_digest: &str,
    manifest_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "oracle_surface"),
            ("oracle", oracle_digest),
            ("manifest", manifest_digest),
        ],
    )
}

pub(crate) fn compose_closeout_seeded_replay_surface_row_digest(
    seeded_digest: &str,
    replay_digest: &str,
    generator_classes_digest: &str,
) -> String {
    compose_certification_row_digest(
        "projection_consumption_certification_row_v1",
        &[
            ("lane", "seeded_replay_surface"),
            ("seeded", seeded_digest),
            ("replay", replay_digest),
            ("classes", generator_classes_digest),
        ],
    )
}
