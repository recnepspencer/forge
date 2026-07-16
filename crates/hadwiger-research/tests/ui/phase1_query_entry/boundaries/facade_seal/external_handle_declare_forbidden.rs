use worth_query::facade::domain::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationInput,
};
use hadwiger_research::facade::{
    CandidateGraphDeclarationFamily, HadwigerResearchDomainEntry, HadwigerResearchHandle,
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
    let _ = handle.declare_checked(ExternalDeclaration);
}

fn main() {}
