use crate::identity::hash_parts;

use super::{
    WorthQueryMutationSurfacePosture, WorthQueryMutationSurfaceReport,
    WorthQueryRuntimeBackendPosture, WorthQueryRuntimePublicApiNamingContract,
    WorthQueryRuntimePublicSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAspectApiFinalizationCloseout {
    backend_posture: WorthQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    mutation_surface_digest: String,
    naming_contract_digest: String,
    preferred_stable_surfaces: Vec<String>,
    lower_level_stable_surfaces: Vec<String>,
    support_gated_surfaces: Vec<String>,
    safe_to_build_now: Vec<String>,
    must_not_assume_yet: Vec<String>,
    migration_guidance: Vec<String>,
    required_verification_commands: Vec<String>,
    closeout_self_check_answers: Vec<String>,
    closeout_digest: String,
}

impl WorthQueryAspectApiFinalizationCloseout {
    pub fn derive(
        backend_posture: WorthQueryRuntimeBackendPosture,
        support_matrix: &WorthQueryRuntimePublicSupportMatrix,
        mutation_surface: &WorthQueryMutationSurfaceReport,
        naming_contract: &WorthQueryRuntimePublicApiNamingContract,
    ) -> Self {
        let preferred_stable_surfaces = mutation_surface
            .rows()
            .iter()
            .filter(|row| row.posture() == WorthQueryMutationSurfacePosture::PreferredStable)
            .map(|row| row.surface().to_string())
            .collect::<Vec<_>>();
        let lower_level_stable_surfaces = mutation_surface
            .rows()
            .iter()
            .filter(|row| row.posture() == WorthQueryMutationSurfacePosture::LowerLevelStable)
            .map(|row| {
                row.preferred_replacement()
                    .map(|replacement| format!("{}=>{}", row.surface(), replacement))
                    .unwrap_or_else(|| row.surface().to_string())
            })
            .collect::<Vec<_>>();
        let support_gated_surfaces = mutation_surface
            .rows()
            .iter()
            .filter(|row| row.posture() == WorthQueryMutationSurfacePosture::SupportGated)
            .map(|row| row.surface().to_string())
            .collect::<Vec<_>>();

        let safe_to_build_now = vec![
            "aspect-native authoritative CRUD through workspace.insert/update/delete plus explicit submission batches"
                .to_string(),
            "preview-local aspect-native CRUD through preview.insert/update/delete/batch"
                .to_string(),
            "runtime receipts, state snapshots, and inspection for aspect-authored mutation"
                .to_string(),
            "domain runtimes that keep async execution, store durability, and substrate ownership behind their own adapter boundary"
                .to_string(),
            "wasm-facing and deployed runtime APIs that compile against WorthQueryWorkspace without depending on payload-shaped internals"
                .to_string(),
        ];
        let must_not_assume_yet = vec![
            "terminal document JSON or external reference artifacts are native authority carriers"
                .to_string(),
            "lower-level write commands are the preferred ordinary public story"
                .to_string(),
            "intent authority, effect-intent consumption, temporal execution, async/resource execution, or mixed-cause delivery are admitted stable mutation families"
                .to_string(),
            "store-backed parity, durable restart/reload, or cross-process replay semantics are closed and certified"
                .to_string(),
            "downstream runtimes may reach into lower-crate mutation/storage internals instead of staying on the WorthQuery facade"
                .to_string(),
        ];
        let migration_guidance = vec![
            "author new runtime code against workspace.insert/update/delete, workspace.submissions()?.submit_batch(commands), and preview.insert/update/delete/batch"
                .to_string(),
            "treat WorthQueryWriteCommand::* as lower-level command artifacts owned by explicit intent or submission lanes, not the daily-driver API"
                .to_string(),
            "use `workspace.public_mutation_surface_report()` when a runtime or doc needs the exact preferred-versus-lower-level-versus-support-gated mutation posture"
                .to_string(),
            "keep direct workspace write and batch helpers sealed; publish command-shaped mutation through workspace.write_intent(...) or workspace.submissions()"
                .to_string(),
            "keep mutation receipts, state snapshots, and inspect output as the downstream explanation contract"
                .to_string(),
            "gate intent-shaped authority crossings through support admission until that family is explicitly stabilized"
                .to_string(),
            "treat terminal JSON documents as external import/export only; keep runtime mutation and read authority in aspect-native proof carriers"
                .to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt -p worth-query".to_string(),
            "cargo check -p worth-query --tests".to_string(),
            "cargo test --manifest-path crates/worth-query/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "cargo test -p worth-query".to_string(),
            "cargo test -p worth-query runtime_public_mutation_surface_report_lists_only_live_lower_level_command_surfaces".to_string(),
            "cargo test -p worth-query runtime_public_aspect_api_finalization_closeout_answers_substrate_handoff_questions".to_string(),
            "git diff --check".to_string(),
        ];
        let closeout_self_check_answers = vec![
            format!(
                "preferred public mutation DX is aspect-native: {} preferred stable surfaces",
                preferred_stable_surfaces.len()
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
                "lower-level seams stay explicit rather than co-equal: {} lower-level stable surfaces",
                lower_level_stable_surfaces.len()
            ),
            "downstream runtimes may build on the facade now, while JSON-shaped authority remains forbidden".to_string(),
        ];

        let mut parts = vec![
            "worth_query_aspect_api_finalization_closeout_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!(
                "support:{}",
                support_matrix
                    .matrix_digest()
                    .terminal_projection_for_reporting()
            ),
            format!("mutation:{}", mutation_surface.report_digest()),
            format!("naming:{}", naming_contract.contract_digest()),
        ];
        parts.extend(
            preferred_stable_surfaces
                .iter()
                .map(|item| format!("preferred:{item}")),
        );
        parts.extend(
            lower_level_stable_surfaces
                .iter()
                .map(|item| format!("lower_level:{item}")),
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
            support_matrix_digest: support_matrix
                .matrix_digest()
                .terminal_projection_for_reporting()
                .to_string(),
            mutation_surface_digest: mutation_surface.report_digest().to_string(),
            naming_contract_digest: naming_contract.contract_digest().to_string(),
            preferred_stable_surfaces,
            lower_level_stable_surfaces,
            support_gated_surfaces,
            safe_to_build_now,
            must_not_assume_yet,
            migration_guidance,
            required_verification_commands,
            closeout_self_check_answers,
            closeout_digest,
        }
    }

    pub fn backend_posture(&self) -> WorthQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn mutation_surface_digest(&self) -> &str {
        &self.mutation_surface_digest
    }

    pub fn naming_contract_digest(&self) -> &str {
        &self.naming_contract_digest
    }

    pub fn preferred_stable_surfaces(&self) -> &[String] {
        &self.preferred_stable_surfaces
    }

    pub fn lower_level_stable_surfaces(&self) -> &[String] {
        &self.lower_level_stable_surfaces
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
