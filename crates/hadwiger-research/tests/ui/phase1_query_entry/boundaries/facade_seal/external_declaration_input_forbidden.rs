use worth_query::facade::domain::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationInput,
};
use hadwiger_research::facade::{
    declare_research_request_checked, CandidateGraphDeclarationFamily,
    HadwigerResearchDomainEntry, HadwigerResearchHandle,
};

struct ExternalDeclaration;

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for ExternalDeclaration {
    type Family = CandidateGraphDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "declaration_kind",
            "external",
        )]
    }
}

fn attempt(handle: &HadwigerResearchHandle) {
    let _ = declare_research_request_checked(handle, ExternalDeclaration);
}

fn main() {}
