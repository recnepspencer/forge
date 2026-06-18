use super::{hard_prohibition_registry, ForgeQueryProhibitionEnforcementTier};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryHardProhibitionDocumentationRow {
    seam_key: &'static str,
    public_symbol: &'static str,
    enforcement_tier: ForgeQueryProhibitionEnforcementTier,
    replacement_lane: &'static str,
    rationale: &'static str,
}

impl ForgeQueryHardProhibitionDocumentationRow {
    pub(crate) const fn new(
        seam_key: &'static str,
        public_symbol: &'static str,
        enforcement_tier: ForgeQueryProhibitionEnforcementTier,
        replacement_lane: &'static str,
        rationale: &'static str,
    ) -> Self {
        Self {
            seam_key,
            public_symbol,
            enforcement_tier,
            replacement_lane,
            rationale,
        }
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam_key
    }

    pub fn public_symbol(&self) -> &'static str {
        self.public_symbol
    }

    pub fn enforcement_tier(&self) -> ForgeQueryProhibitionEnforcementTier {
        self.enforcement_tier
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub fn rationale(&self) -> &'static str {
        self.rationale
    }
}

pub fn hard_prohibition_documentation_rows() -> Vec<ForgeQueryHardProhibitionDocumentationRow> {
    hard_prohibition_registry()
        .rows()
        .iter()
        .map(|row| {
            ForgeQueryHardProhibitionDocumentationRow::new(
                row.seam_key(),
                row.public_symbol(),
                row.enforcement_tier(),
                row.replacement_lane(),
                row.rationale(),
            )
        })
        .collect()
}

pub fn hard_prohibition_documented_seam_keys() -> Vec<&'static str> {
    hard_prohibition_documentation_rows()
        .into_iter()
        .map(|row| row.seam_key())
        .collect()
}

pub fn render_hard_prohibition_reference() -> String {
    let mut reference = String::from("# Forge Query Hard Prohibitions\n\n");
    reference.push_str(
        "This reference is generated from the hard prohibition registry. \
Do not edit it without updating the registry-owned projection test.\n\n",
    );
    reference
        .push_str("| Seam | Forbidden symbol | Enforcement | Replacement lane | Rationale |\n");
    reference.push_str("| --- | --- | --- | --- | --- |\n");

    for row in hard_prohibition_documentation_rows() {
        reference.push_str("| ");
        reference.push_str(row.seam_key());
        reference.push_str(" | `");
        reference.push_str(row.public_symbol());
        reference.push_str("` | ");
        reference.push_str(row.enforcement_tier().as_str());
        reference.push_str(" | ");
        reference.push_str(row.replacement_lane());
        reference.push_str(" | ");
        reference.push_str(row.rationale());
        reference.push_str(" |\n");
    }

    reference
}
