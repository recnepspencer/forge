use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput,
};
use hadwiger_research::facade::{
    declare_research_request_checked, CandidateGraphDeclarationFamily,
    HadwigerResearchDomainEntry, HadwigerResearchHandle,
};

struct ExternalDeclaration;

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for ExternalDeclaration {
    type Family = CandidateGraphDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "declaration_kind",
            "external",
        )]
    }
}

fn attempt(handle: &HadwigerResearchHandle) {
    let _ = declare_research_request_checked(handle, ExternalDeclaration);
}

fn main() {}
