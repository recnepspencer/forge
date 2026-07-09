use worth_query::facade::{
    WORTHQueryCanonicalDeclarationArtifact, WORTHQueryDeclaredFamilyChecked,
};

use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::domain_declarations::{
    declare_research_request_checked, MotifSeedDeclaration, TerminalForcingStudyDeclaration,
};
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};

use super::motif_artifacts::MotifArtifact;
use super::motif_errors::MotifLanguageError;
use super::terminal_relation_certificates::TerminalForcingRelationCertificate;
use super::terminal_relations::TerminalForcingRelation;

pub fn build_motif_from_seed_declaration_checked(
    _handle: &HadwigerResearchHandle,
    source_declaration: WORTHQueryCanonicalDeclarationArtifact<
        HadwigerResearchDomainEntry,
        MotifSeedDeclaration,
    >,
    builder: crate::motif_language::MotifArtifactBuilder,
) -> Result<MotifArtifact, MotifLanguageError> {
    let source_reference: crate::domain_artifacts::HadwigerQueryDeclarationReference =
        source_declaration.into();
    if builder.source_declaration() != &source_reference {
        return Err(MotifLanguageError::MotifSourceDeclarationMismatch);
    }
    builder.finish()
}

pub fn certify_terminal_forcing_relation_checked(
    handle: &HadwigerResearchHandle,
    study_declaration: TerminalForcingStudyDeclaration,
    motif: &MotifArtifact,
    certificate: TerminalForcingRelationCertificate,
) -> Result<TerminalForcingRelation, MotifLanguageError> {
    validate_terminal_study_request(&study_declaration, motif, &certificate)?;
    let source_declaration = match declare_research_request_checked(handle, study_declaration) {
        WORTHQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => return Err(MotifLanguageError::TerminalStudyDeclarationNotAdmitted),
    };
    let source_reference = source_declaration.into();
    TerminalForcingRelation::checked(source_reference, motif, certificate)
}

fn validate_terminal_study_request(
    study_declaration: &TerminalForcingStudyDeclaration,
    motif: &MotifArtifact,
    certificate: &TerminalForcingRelationCertificate,
) -> Result<(), MotifLanguageError> {
    if study_declaration.motif_ref() != motif.reference().stable_token() {
        return Err(MotifLanguageError::TerminalStudyMotifMismatch);
    }
    if study_declaration.terminals() != certificate.terminal_labels() {
        return Err(MotifLanguageError::TerminalStudyTerminalMismatch);
    }
    if let Some(relation_goal) = study_declaration.relation_goal() {
        let certificate_goal = certificate.relation_kind().as_str();
        if relation_goal != certificate_goal {
            return Err(MotifLanguageError::TerminalStudyRelationGoalMismatch {
                expected: relation_goal.to_string(),
                actual: certificate_goal.to_string(),
            });
        }
    }
    Ok(())
}
