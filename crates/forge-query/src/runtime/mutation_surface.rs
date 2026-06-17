use crate::identity::hash_parts;

use super::{
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicSupportMatrix,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryMutationSurfacePosture {
    PreferredStable,
    LowerLevelStable,
    SupportGated,
}

impl ForgeQueryMutationSurfacePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreferredStable => "preferred-stable",
            Self::LowerLevelStable => "lower-level-stable",
            Self::SupportGated => "support-gated",
        }
    }
}

impl std::fmt::Display for ForgeQueryMutationSurfacePosture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationSurfaceRow {
    concept: String,
    surface: String,
    posture: ForgeQueryMutationSurfacePosture,
    ordinary_public_story: bool,
    preferred_replacement: Option<String>,
    reason: String,
    row_digest: String,
}

impl ForgeQueryMutationSurfaceRow {
    fn new(
        concept: impl Into<String>,
        surface: impl Into<String>,
        posture: ForgeQueryMutationSurfacePosture,
        ordinary_public_story: bool,
        preferred_replacement: Option<impl Into<String>>,
        reason: impl Into<String>,
    ) -> Self {
        let concept = concept.into();
        let surface = surface.into();
        let preferred_replacement = preferred_replacement.map(Into::into);
        let reason = reason.into();
        let mut parts = vec![
            format!("concept:{concept}"),
            format!("surface:{surface}"),
            format!("posture:{}", posture.as_str()),
            format!("ordinary:{ordinary_public_story}"),
            format!("reason:{reason}"),
        ];
        if let Some(replacement) = &preferred_replacement {
            parts.push(format!("replacement:{replacement}"));
        }
        let row_digest = hash_parts(&parts);
        Self {
            concept,
            surface,
            posture,
            ordinary_public_story,
            preferred_replacement,
            reason,
            row_digest,
        }
    }

    pub fn concept(&self) -> &str {
        &self.concept
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn posture(&self) -> ForgeQueryMutationSurfacePosture {
        self.posture
    }

    pub fn ordinary_public_story(&self) -> bool {
        self.ordinary_public_story
    }

    pub fn preferred_replacement(&self) -> Option<&str> {
        self.preferred_replacement.as_deref()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationSurfaceReport {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    naming_contract_digest: String,
    rows: Vec<ForgeQueryMutationSurfaceRow>,
    preferred_stable_count: usize,
    lower_level_stable_count: usize,
    support_gated_count: usize,
    report_digest: String,
}

impl ForgeQueryMutationSurfaceReport {
    pub fn derive(
        backend_posture: ForgeQueryRuntimeBackendPosture,
        support_matrix: &ForgeQueryRuntimePublicSupportMatrix,
        naming_contract: &ForgeQueryRuntimePublicApiNamingContract,
    ) -> Self {
        let rows = vec![
            ForgeQueryMutationSurfaceRow::new(
                "authoritative-insert",
                "workspace.insert(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred aspect-native authoritative create path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "authoritative-update",
                "workspace.update(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred aspect-native authoritative update path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "authoritative-delete",
                "workspace.delete(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred authoritative delete path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "authoritative-batch",
                "workspace.batch(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred ordered multi-mutation path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "preview-insert",
                "preview.insert(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred preview-local aspect-native create path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "preview-update",
                "preview.update(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred preview-local aspect-native update path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "preview-delete",
                "preview.delete(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred preview-local delete path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "preview-batch",
                "preview.batch(...)",
                ForgeQueryMutationSurfacePosture::PreferredStable,
                true,
                None::<String>,
                "preferred preview-local ordered multi-mutation path",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "direct-write-lower-level",
                "workspace.write(...)",
                ForgeQueryMutationSurfacePosture::LowerLevelStable,
                false,
                Some("workspace.insert/update/delete/batch"),
                "stable lower-level expert seam for command-shaped runtime mutation; ordinary runtime code should prefer workspace.insert/update/delete/batch",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "aspect-insert-command",
                "ForgeQueryWriteCommand::InsertAspects",
                ForgeQueryMutationSurfacePosture::LowerLevelStable,
                false,
                Some("workspace.insert(...)"),
                "lower-level command path for aspect-native insert authoring",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "command-update-aspect",
                "ForgeQueryWriteCommand::UpdateAspect",
                ForgeQueryMutationSurfacePosture::LowerLevelStable,
                false,
                Some("workspace.update(...)"),
                "lower-level command path for single-aspect update authoring",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "command-update-aspects",
                "ForgeQueryWriteCommand::UpdateAspects",
                ForgeQueryMutationSurfacePosture::LowerLevelStable,
                false,
                Some("workspace.update(...)"),
                "lower-level command path for multi-aspect update authoring",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "command-delete",
                "ForgeQueryWriteCommand::Delete",
                ForgeQueryMutationSurfacePosture::LowerLevelStable,
                false,
                Some("workspace.delete(...)"),
                "lower-level command path for delete authoring",
            ),
            ForgeQueryMutationSurfaceRow::new(
                "intent-commit",
                "workspace.intent(...)",
                ForgeQueryMutationSurfacePosture::SupportGated,
                false,
                None::<String>,
                support_matrix
                    .row_for_family(ForgeQueryRuntimeFacadeFamily::Intent)
                    .and_then(|row| row.support_contract_digest().map(|_| row.owner_milestone()))
                    .map(|owner| format!("public vocabulary only; support gate owned by {owner}"))
                    .unwrap_or_else(|| {
                        "public vocabulary only; support-gated intent authority path".to_string()
                    }),
            ),
            ForgeQueryMutationSurfaceRow::new(
                "effect-intent-consumption",
                "workspace.next_effect_intent(...)",
                ForgeQueryMutationSurfacePosture::SupportGated,
                false,
                None::<String>,
                "consumes staged effect intent residue only when the runtime admits intent support",
            ),
        ];

        let preferred_stable_count = rows
            .iter()
            .filter(|row| row.posture() == ForgeQueryMutationSurfacePosture::PreferredStable)
            .count();
        let lower_level_stable_count = rows
            .iter()
            .filter(|row| row.posture() == ForgeQueryMutationSurfacePosture::LowerLevelStable)
            .count();
        let support_gated_count = rows
            .iter()
            .filter(|row| row.posture() == ForgeQueryMutationSurfacePosture::SupportGated)
            .count();
        let mut parts = vec![
            "forge_query_mutation_surface_report_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!(
                "support_matrix:{}",
                support_matrix
                    .matrix_digest()
                    .terminal_projection_for_reporting()
            ),
            format!("naming_contract:{}", naming_contract.contract_digest()),
            format!("preferred:{preferred_stable_count}"),
            format!("lower_level:{lower_level_stable_count}"),
            format!("support_gated:{support_gated_count}"),
        ];
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        let report_digest = hash_parts(&parts);
        Self {
            backend_posture,
            support_matrix_digest: support_matrix
                .matrix_digest()
                .terminal_projection_for_reporting()
                .to_string(),
            naming_contract_digest: naming_contract.contract_digest().to_string(),
            rows,
            preferred_stable_count,
            lower_level_stable_count,
            support_gated_count,
            report_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn naming_contract_digest(&self) -> &str {
        &self.naming_contract_digest
    }

    pub fn rows(&self) -> &[ForgeQueryMutationSurfaceRow] {
        &self.rows
    }

    pub fn preferred_stable_count(&self) -> usize {
        self.preferred_stable_count
    }

    pub fn lower_level_stable_count(&self) -> usize {
        self.lower_level_stable_count
    }

    pub fn support_gated_count(&self) -> usize {
        self.support_gated_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn row_by_surface(&self, surface: &str) -> Option<&ForgeQueryMutationSurfaceRow> {
        self.rows.iter().find(|row| row.surface() == surface)
    }

    pub fn row_by_concept(&self, concept: &str) -> Option<&ForgeQueryMutationSurfaceRow> {
        self.rows.iter().find(|row| row.concept() == concept)
    }
}
