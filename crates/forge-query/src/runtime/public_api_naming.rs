use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiNamingRow {
    concept: String,
    preferred_name: String,
    alternate_names: Vec<String>,
    boundary_crossing: bool,
    naming_digest: String,
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
        let mut parts = vec![
            format!("concept:{concept}"),
            format!("preferred:{preferred_name}"),
            format!("boundary:{boundary_crossing}"),
        ];
        parts.extend(
            alternate_names
                .iter()
                .map(|name| format!("alternate:{name}")),
        );
        let naming_digest = hash_parts(&parts);
        Self {
            concept,
            preferred_name,
            alternate_names,
            boundary_crossing,
            naming_digest,
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
        &self.naming_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiNamingContract {
    rows: Vec<ForgeQueryRuntimePublicApiNamingRow>,
    preferred_entrypoint_count: usize,
    alternate_name_count: usize,
    boundary_crossing_name_count: usize,
    contract_digest: String,
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
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "insert",
                "insert",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "update",
                "update",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "delete",
                "delete",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "batch",
                "batch",
                std::iter::empty::<&str>(),
                false,
            ),
        ];
        let preferred_entrypoint_count = rows.len();
        let alternate_name_count = rows.iter().map(|row| row.alternate_names().len()).sum();
        let boundary_crossing_name_count =
            rows.iter().filter(|row| row.boundary_crossing()).count();
        let mut parts = vec![
            "forge_query_runtime_public_api_naming_contract_v1".to_string(),
            format!("preferred:{preferred_entrypoint_count}"),
            format!("alternate:{alternate_name_count}"),
            format!("boundary:{boundary_crossing_name_count}"),
        ];
        parts.extend(
            rows.iter()
                .map(|row| format!("row:{}", row.naming_digest())),
        );
        let contract_digest = hash_parts(&parts);
        Self {
            rows,
            preferred_entrypoint_count,
            alternate_name_count,
            boundary_crossing_name_count,
            contract_digest,
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
        &self.contract_digest
    }
}
