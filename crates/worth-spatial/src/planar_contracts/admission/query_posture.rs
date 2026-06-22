use forge_query::facade::ForgeQueryCapabilityFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarQueryPosture {
    configured_domain_handle_required: bool,
    canonical_declaration_required: bool,
    declaration_family_taxonomy_required: bool,
    declaration_family_capability_matrix_required: bool,
    broad_public_dx_helper_gated: bool,
    required_capability_families: &'static [ForgeQueryCapabilityFamily],
}

impl PlanarQueryPosture {
    pub(crate) const fn required_now(
        required_capability_families: &'static [ForgeQueryCapabilityFamily],
    ) -> Self {
        Self {
            configured_domain_handle_required: true,
            canonical_declaration_required: true,
            declaration_family_taxonomy_required: true,
            declaration_family_capability_matrix_required: true,
            broad_public_dx_helper_gated: true,
            required_capability_families,
        }
    }

    pub(crate) const fn support_gated() -> Self {
        Self {
            configured_domain_handle_required: true,
            canonical_declaration_required: false,
            declaration_family_taxonomy_required: true,
            declaration_family_capability_matrix_required: true,
            broad_public_dx_helper_gated: true,
            required_capability_families: &[ForgeQueryCapabilityFamily::QueryComposition],
        }
    }

    pub fn configured_domain_handle_required(&self) -> bool {
        self.configured_domain_handle_required
    }

    pub fn canonical_declaration_required(&self) -> bool {
        self.canonical_declaration_required
    }

    pub fn declaration_family_taxonomy_required(&self) -> bool {
        self.declaration_family_taxonomy_required
    }

    pub fn declaration_family_capability_matrix_required(&self) -> bool {
        self.declaration_family_capability_matrix_required
    }

    pub fn broad_public_dx_helper_gated(&self) -> bool {
        self.broad_public_dx_helper_gated
    }

    pub fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        self.required_capability_families
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let capabilities = self
            .required_capability_families
            .iter()
            .map(ForgeQueryCapabilityFamily::as_str)
            .collect::<Vec<_>>()
            .join(",");
        vec![
            format!("domain:{}", self.configured_domain_handle_required),
            format!("canonical:{}", self.canonical_declaration_required),
            format!("taxonomy:{}", self.declaration_family_taxonomy_required),
            format!(
                "capability-matrix:{}",
                self.declaration_family_capability_matrix_required
            ),
            format!("dx-gated:{}", self.broad_public_dx_helper_gated),
            format!("capabilities:{capabilities}"),
        ]
    }
}
