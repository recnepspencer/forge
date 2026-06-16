use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiNamingRow {
    concept: String,
    preferred_name: String,
    alternate_names: Vec<String>,
    boundary_crossing: bool,
    naming_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimePublicApiNamingRow {
    pub(crate) fn new(
        concept: impl Into<String>,
        preferred_name: impl Into<String>,
        alternate_names: impl IntoIterator<Item = impl Into<String>>,
        boundary_crossing: bool,
    ) -> Self {
        let concept = concept.into();
        let preferred_name = preferred_name.into();
        let alternate_names = alternate_names
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let naming_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicApiNamingRow)
                .field_shape(ForgeQueryEvidenceTag::new("concept"), concept.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("preferred_name"),
                    preferred_name.as_str(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("boundary_crossing"),
                    boundary_crossing,
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("alternate_name"),
                    alternate_names.iter().map(String::as_str),
                )
                .seal();
        Self {
            concept,
            preferred_name,
            alternate_names,
            boundary_crossing,
            naming_identity,
        }
    }

    pub fn concept(&self) -> &str {
        &self.concept
    }

    pub fn preferred_name(&self) -> &str {
        &self.preferred_name
    }

    pub fn alternate_names(&self) -> &[String] {
        &self.alternate_names
    }

    pub fn boundary_crossing(&self) -> bool {
        self.boundary_crossing
    }

    pub fn naming_digest(&self) -> &str {
        self.naming_identity.as_str()
    }

    pub fn naming_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.naming_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiNamingContract {
    rows: Vec<ForgeQueryRuntimePublicApiNamingRow>,
    preferred_entrypoint_count: usize,
    alternate_name_count: usize,
    boundary_crossing_name_count: usize,
    contract_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimePublicApiNamingContract {
    pub fn standard() -> Self {
        let rows = vec![
            ForgeQueryRuntimePublicApiNamingRow::new(
                "workspace",
                "workspace",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "live-view",
                "live_view",
                ["live_view_request", "declare_live_view"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "live-view-builder",
                "live_view closure",
                std::iter::empty::<&str>(),
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "computed",
                "computed",
                [
                    "computed_view",
                    "computed_definition",
                    "declare_maintained_derived_view",
                    "declare_derived_view",
                ],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "computed-builder",
                "computed closure",
                std::iter::empty::<&str>(),
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new("effect", "effect", ["declare_effect"], true),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "preview",
                "preview",
                ["preview_with_options"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "branch",
                "branch",
                ["branch_with_options"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "state",
                "state",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "inspect",
                "inspect",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "read",
                "read",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new("observe", "observe", ["drain_patches"], true),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "materialize",
                "materialize",
                ["snapshot_rows"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "write",
                "write",
                ["ForgeQueryWriteCommand"],
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "insert",
                "insert",
                ["ForgeQueryWriteCommand::InsertAspects"],
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "update",
                "update",
                [
                    "ForgeQueryWriteCommand::UpdateAspect",
                    "ForgeQueryWriteCommand::UpdateAspects",
                ],
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "delete",
                "delete",
                ["ForgeQueryWriteCommand::Delete"],
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "batch",
                "batch",
                ["workspace.write(...)"],
                false,
            ),
        ];
        let preferred_entrypoint_count = rows.len();
        let alternate_name_count = rows.iter().map(|row| row.alternate_names().len()).sum();
        let boundary_crossing_name_count =
            rows.iter().filter(|row| row.boundary_crossing()).count();
        let contract_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicApiNamingContract)
                .field_usize(
                    ForgeQueryEvidenceTag::new("preferred_entrypoint_count"),
                    preferred_entrypoint_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("alternate_name_count"),
                    alternate_name_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("boundary_crossing_name_count"),
                    boundary_crossing_name_count,
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("row_digest"),
                    rows.iter()
                        .map(ForgeQueryRuntimePublicApiNamingRow::naming_digest),
                )
                .seal();
        Self {
            rows,
            preferred_entrypoint_count,
            alternate_name_count,
            boundary_crossing_name_count,
            contract_identity,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryRuntimePublicApiNamingRow] {
        &self.rows
    }

    pub fn preferred_name_for(&self, concept: &str) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.concept() == concept)
            .map(ForgeQueryRuntimePublicApiNamingRow::preferred_name)
    }

    pub fn preferred_entrypoint_count(&self) -> usize {
        self.preferred_entrypoint_count
    }

    pub fn alternate_name_count(&self) -> usize {
        self.alternate_name_count
    }

    pub fn boundary_crossing_name_count(&self) -> usize {
        self.boundary_crossing_name_count
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_identity.as_str()
    }

    pub fn contract_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.contract_identity
    }
}

#[allow(dead_code)]
pub(crate) fn compose_public_api_naming_row_identity(
    row: &ForgeQueryRuntimePublicApiNamingRow,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicApiNamingRow)
        .field_shape(ForgeQueryEvidenceTag::new("concept"), row.concept())
        .field_shape(
            ForgeQueryEvidenceTag::new("preferred_name"),
            row.preferred_name(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("boundary_crossing"),
            row.boundary_crossing(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("alternate_name"),
            row.alternate_names().iter().map(String::as_str),
        )
        .seal()
}

#[allow(dead_code)]
pub(crate) fn compose_public_api_naming_contract_identity(
    contract: &ForgeQueryRuntimePublicApiNamingContract,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicApiNamingContract)
        .field_usize(
            ForgeQueryEvidenceTag::new("preferred_entrypoint_count"),
            contract.preferred_entrypoint_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("alternate_name_count"),
            contract.alternate_name_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("boundary_crossing_name_count"),
            contract.boundary_crossing_name_count(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("row_digest"),
            contract
                .rows()
                .iter()
                .map(ForgeQueryRuntimePublicApiNamingRow::naming_digest),
        )
        .seal()
}
