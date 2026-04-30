use crate::identity::hash_parts;

use super::{
    ForgeQueryMutationApiCompatibilityReport, ForgeQueryMutationCompatibilityPosture,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimePublicApiNamingContract,
    ForgeQueryRuntimePublicSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAspectApiFinalizationCloseout {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    mutation_compatibility_digest: String,
    naming_contract_digest: String,
    preferred_stable_surfaces: Vec<String>,
    stable_compatibility_surfaces: Vec<String>,
    deprecated_compatibility_surfaces: Vec<String>,
    support_gated_surfaces: Vec<String>,
    safe_to_build_now: Vec<String>,
    must_not_assume_yet: Vec<String>,
    migration_guidance: Vec<String>,
    required_verification_commands: Vec<String>,
    closeout_self_check_answers: Vec<String>,
    closeout_digest: String,
}

impl ForgeQueryAspectApiFinalizationCloseout {
    pub fn derive(
        backend_posture: ForgeQueryRuntimeBackendPosture,
        support_matrix: &ForgeQueryRuntimePublicSupportMatrix,
        mutation_compatibility: &ForgeQueryMutationApiCompatibilityReport,
        naming_contract: &ForgeQueryRuntimePublicApiNamingContract,
    ) -> Self {
        let preferred_stable_surfaces = mutation_compatibility
            .rows()
            .iter()
            .filter(|row| row.posture() == ForgeQueryMutationCompatibilityPosture::PreferredStable)
            .map(|row| row.surface().to_string())
            .collect::<Vec<_>>();
        let stable_compatibility_surfaces = mutation_compatibility
            .rows()
            .iter()
            .filter(|row| {
                row.posture() == ForgeQueryMutationCompatibilityPosture::StableCompatibility
            })
            .map(|row| {
                row.preferred_replacement()
                    .map(|replacement| format!("{}=>{}", row.surface(), replacement))
                    .unwrap_or_else(|| row.surface().to_string())
            })
            .collect::<Vec<_>>();
        let deprecated_compatibility_surfaces = mutation_compatibility
            .rows()
            .iter()
            .filter(|row| {
                row.posture() == ForgeQueryMutationCompatibilityPosture::DeprecatedCompatibility
            })
            .map(|row| {
                row.preferred_replacement()
                    .map(|replacement| format!("{}=>{}", row.surface(), replacement))
                    .unwrap_or_else(|| row.surface().to_string())
            })
            .collect::<Vec<_>>();
        let support_gated_surfaces = mutation_compatibility
            .rows()
            .iter()
            .filter(|row| row.posture() == ForgeQueryMutationCompatibilityPosture::SupportGated)
            .map(|row| row.surface().to_string())
            .collect::<Vec<_>>();

        let safe_to_build_now = vec![
            "aspect-native authoritative CRUD through workspace.insert/update/delete/batch"
                .to_string(),
            "preview-local aspect-native CRUD through preview.insert/update/delete/batch"
                .to_string(),
            "runtime receipts, state snapshots, and inspection for aspect-authored mutation"
                .to_string(),
            "domain runtimes that keep async execution, store durability, and substrate ownership behind their own adapter boundary"
                .to_string(),
            "wasm-facing and deployed runtime APIs that compile against ForgeQueryWorkspace without depending on payload-shaped internals"
                .to_string(),
        ];
        let must_not_assume_yet = vec![
            "JSON has already been removed from forge-query, forge-relational, forge-store, or the runtime bridge internally"
                .to_string(),
            "payload-first compatibility commands are the preferred ordinary public story"
                .to_string(),
            "intent authority, effect-intent consumption, temporal execution, async/resource execution, or mixed-cause delivery are admitted stable mutation families"
                .to_string(),
            "store-backed parity, durable restart/reload, or cross-process replay semantics are closed and certified"
                .to_string(),
            "downstream runtimes may reach into lower-crate mutation/storage internals instead of staying on the Forge Query facade"
                .to_string(),
        ];
        let migration_guidance = vec![
            "author new runtime code against workspace.insert/update/delete/batch and preview.insert/update/delete/batch"
                .to_string(),
            "treat workspace.write(...) and ForgeQueryWriteCommand::* as compatibility or lower-level seams, not the daily-driver API"
                .to_string(),
            "keep workspace.write(...) available as an expert compatibility seam during the substrate rewrite, but do not require it in ordinary downstream runtime APIs"
                .to_string(),
            "keep mutation receipts, state snapshots, and inspect output as the downstream explanation contract"
                .to_string(),
            "gate intent-shaped authority crossings through support admission until that family is explicitly stabilized"
                .to_string(),
            "move JSON removal work underneath this facade instead of teaching new code to depend on payload lowering"
                .to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt -p forge-query".to_string(),
            "cargo check -p forge-query --tests".to_string(),
            "cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "cargo test -p forge-query".to_string(),
            "cargo test -p forge-query runtime_public_mutation_compatibility_report_marks_payload_insert_deprecated".to_string(),
            "cargo test -p forge-query runtime_public_aspect_api_finalization_closeout_answers_substrate_handoff_questions".to_string(),
            "git diff --check".to_string(),
        ];
        let closeout_self_check_answers = vec![
            format!(
                "preferred public mutation DX is aspect-native: {} preferred stable surfaces",
                preferred_stable_surfaces.len()
            ),
            format!(
                "payload-first ordinary authoring is closed off: {} deprecated compatibility surfaces",
                deprecated_compatibility_surfaces.len()
            ),
            format!(
                "support-gated mutation neighbors stay fail-closed: {} gated surfaces",
                support_gated_surfaces.len()
            ),
            format!(
                "write-family support remains synchronized with the public matrix: {} support rows",
                support_matrix.rows().len()
            ),
            format!(
                "compatibility seams stay explicit rather than co-equal: {} stable compatibility surfaces",
                stable_compatibility_surfaces.len()
            ),
            "downstream runtimes may build on the facade now, while lower-crate JSON removal remains an internal rewrite".to_string(),
        ];

        let mut parts = vec![
            "forge_query_aspect_api_finalization_closeout_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("support:{}", support_matrix.matrix_digest()),
            format!("mutation:{}", mutation_compatibility.report_digest()),
            format!("naming:{}", naming_contract.contract_digest()),
        ];
        parts.extend(
            preferred_stable_surfaces
                .iter()
                .map(|item| format!("preferred:{item}")),
        );
        parts.extend(
            stable_compatibility_surfaces
                .iter()
                .map(|item| format!("compat:{item}")),
        );
        parts.extend(
            deprecated_compatibility_surfaces
                .iter()
                .map(|item| format!("deprecated:{item}")),
        );
        parts.extend(
            support_gated_surfaces
                .iter()
                .map(|item| format!("gated:{item}")),
        );
        parts.extend(
            closeout_self_check_answers
                .iter()
                .map(|item| format!("self_check:{item}")),
        );
        let closeout_digest = hash_parts(&parts);

        Self {
            backend_posture,
            support_matrix_digest: support_matrix.matrix_digest().to_string(),
            mutation_compatibility_digest: mutation_compatibility.report_digest().to_string(),
            naming_contract_digest: naming_contract.contract_digest().to_string(),
            preferred_stable_surfaces,
            stable_compatibility_surfaces,
            deprecated_compatibility_surfaces,
            support_gated_surfaces,
            safe_to_build_now,
            must_not_assume_yet,
            migration_guidance,
            required_verification_commands,
            closeout_self_check_answers,
            closeout_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn mutation_compatibility_digest(&self) -> &str {
        &self.mutation_compatibility_digest
    }

    pub fn naming_contract_digest(&self) -> &str {
        &self.naming_contract_digest
    }

    pub fn preferred_stable_surfaces(&self) -> &[String] {
        &self.preferred_stable_surfaces
    }

    pub fn stable_compatibility_surfaces(&self) -> &[String] {
        &self.stable_compatibility_surfaces
    }

    pub fn deprecated_compatibility_surfaces(&self) -> &[String] {
        &self.deprecated_compatibility_surfaces
    }

    pub fn support_gated_surfaces(&self) -> &[String] {
        &self.support_gated_surfaces
    }

    pub fn safe_to_build_now(&self) -> &[String] {
        &self.safe_to_build_now
    }

    pub fn must_not_assume_yet(&self) -> &[String] {
        &self.must_not_assume_yet
    }

    pub fn migration_guidance(&self) -> &[String] {
        &self.migration_guidance
    }

    pub fn required_verification_commands(&self) -> &[String] {
        &self.required_verification_commands
    }

    pub fn closeout_self_check_answers(&self) -> &[String] {
        &self.closeout_self_check_answers
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
