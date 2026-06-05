use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput,
};
use hadwiger_research::facade::{
    CandidateGraphDeclarationFamily, HadwigerResearchDomainEntry, HadwigerResearchHandle,
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
    let _ = handle.declare_checked(ExternalDeclaration);
}

fn main() {}
