use super::authority_kind::WorthValidationAuthorityKind;
use super::discovery::WorthValidationAuthorityDiscoveredSource;
use super::disposition::WorthValidationAuthorityDisposition;
use super::source_authority::WorthValidationAuthoritySource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityInventoryRow {
    source: WorthValidationAuthoritySource,
    source_path: &'static str,
    source_symbol: &'static str,
    authority_kind: WorthValidationAuthorityKind,
    owner: &'static str,
    disposition: WorthValidationAuthorityDisposition,
    removal_trigger: &'static str,
    query_access_dependency: Option<&'static str>,
    certification_only_comparison_allowed: bool,
    note: &'static str,
}

pub(super) struct WorthValidationAuthorityInventoryRowInput {
    pub source: WorthValidationAuthoritySource,
    pub source_path: &'static str,
    pub source_symbol: &'static str,
    pub authority_kind: WorthValidationAuthorityKind,
    pub owner: &'static str,
    pub disposition: WorthValidationAuthorityDisposition,
    pub removal_trigger: &'static str,
    pub query_access_dependency: Option<&'static str>,
    pub certification_only_comparison_allowed: bool,
    pub note: &'static str,
}

impl WorthValidationAuthorityInventoryRow {
    pub(super) fn from_input(input: WorthValidationAuthorityInventoryRowInput) -> Self {
        Self {
            source: input.source,
            source_path: input.source_path,
            source_symbol: input.source_symbol,
            authority_kind: input.authority_kind,
            owner: input.owner,
            disposition: input.disposition,
            removal_trigger: input.removal_trigger,
            query_access_dependency: input.query_access_dependency,
            certification_only_comparison_allowed: input.certification_only_comparison_allowed,
            note: input.note,
        }
    }

    pub const fn source(&self) -> WorthValidationAuthoritySource {
        self.source
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn source_symbol(&self) -> &'static str {
        self.source_symbol
    }

    pub const fn authority_kind(&self) -> WorthValidationAuthorityKind {
        self.authority_kind
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn disposition(&self) -> WorthValidationAuthorityDisposition {
        self.disposition
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn query_access_dependency(&self) -> Option<&'static str> {
        self.query_access_dependency
    }

    pub const fn certification_only_comparison_allowed(&self) -> bool {
        self.certification_only_comparison_allowed
    }

    pub const fn note(&self) -> &'static str {
        self.note
    }

    pub(crate) fn matches_discovered_source(
        &self,
        discovered: &WorthValidationAuthorityDiscoveredSource,
    ) -> bool {
        let discovered_path = discovered.normalized_path();
        let source_path = self.source_path.replace('\\', "/");
        (discovered_path.ends_with(&source_path) || discovered_path.contains(&source_path))
            && self.source_symbol_matches(discovered.pattern().pattern())
    }

    fn source_symbol_matches(&self, pattern: &str) -> bool {
        self.source_symbol == pattern
            || self.source_symbol.contains(pattern)
            || pattern.contains(self.source_symbol)
            || self.note.contains(pattern)
    }
}
