use crate::application::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFoundationalEvidenceInput,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDomainEntryMarker,
};

use super::super::checked::{
    ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationFailed,
    ForgeQueryDeclarationEntryOrchestrationRebindRequired,
    ForgeQueryDeclarationEntryOrchestrationStale,
};
use super::super::proof::{
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
};
use super::super::refusal::{
    ForgeQueryDeclarationEntryOrchestrationRefusal,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
};
use super::canonical_digest_token;
use super::reason::{canonicalization_reason, declare_row_reason, legality_denial_reason};
use super::route::lower_from_route_checked;

pub(super) fn lower_from_declaration_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    stage_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    input: I,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    match handle.declare_checked(input) {
        crate::application::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                Some(canonical_digest_token(declaration.declaration_digest())),
            ));
            lower_from_legality_checked(
                handle,
                stage_records,
                handle.review_legality_checked(declaration),
            )
        }
        crate::application::ForgeQueryDeclaredFamilyChecked::Deferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    declare_row_reason(denial.support_report()),
                    digest,
                ),
            )
        }
        crate::application::ForgeQueryDeclaredFamilyChecked::Unsupported(denial)
        | crate::application::ForgeQueryDeclaredFamilyChecked::InvalidContext(denial) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                Some(denial.support_report().support_digest().to_string()),
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::Refused(
                ForgeQueryDeclarationEntryOrchestrationRefusal::new(
                    denial.support_report().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation,
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    declare_row_reason(denial.support_report()),
                ),
            )
        }
        crate::application::ForgeQueryDeclaredFamilyChecked::Canonicalization(error) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                None,
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    I::Family::semantic_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    canonicalization_reason(&error),
                    None,
                ),
            )
        }
    }
}

fn lower_from_legality_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    stage_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationLegalityChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    match checked {
        ForgeQueryDeclarationLegalityChecked::Legal(legal) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                Some(legal.legality_digest().to_string()),
            ));
            lower_from_progression_checked(
                handle,
                stage_records,
                handle.progress_declaration_checked(legal),
            )
        }
        ForgeQueryDeclarationLegalityChecked::Illegal(denial) => {
            let retained = Some(denial.declaration_digest());
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                retained.clone(),
            ));
            lower_from_legality_denial(denial, retained)
        }
    }
}

fn lower_from_legality_denial<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    denial: ForgeQueryDeclarationLegalityDenial<D, I>,
    retained: Option<String>,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    let family = denial.declaration_family_key();
    let reason = legality_denial_reason(&denial);
    match denial {
        ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    family,
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
        ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            ForgeQueryDeclarationEntryOrchestrationChecked::Refused(
                ForgeQueryDeclarationEntryOrchestrationRefusal::new(
                    family,
                    ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation,
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                ),
            )
        }
        _ => ForgeQueryDeclarationEntryOrchestrationChecked::Denied(
            ForgeQueryDeclarationEntryOrchestrationDenied::new(
                family,
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                reason,
                retained,
            ),
        ),
    }
}

fn lower_from_progression_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    stage_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: crate::application::ForgeQueryDeclarationProgressionChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    match checked {
        crate::application::ForgeQueryDeclarationProgressionChecked::Admitted(progressed) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                Some(progressed.progression_digest().to_string()),
            ));
            let evidence = match handle.describe_foundational(
                ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                    progressed.clone(),
                ),
            ) {
                Ok(evidence) => evidence,
                Err(_) => panic!(
                    "same-handle admitted progression should always describe foundational evidence"
                ),
            };
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
                Some(canonical_digest_token(evidence.attachment_bundle_digest())),
            ));
            let route_checked = handle.plan_routes_checked(
                crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(
                    progressed, evidence,
                ),
            );
            lower_from_route_checked(handle, stage_records, route_checked)
        }
        crate::application::ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
            progression_deferred(stage_records, progress)
        }
        crate::application::ForgeQueryDeclarationProgressionChecked::Denied(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::Denied(
                ForgeQueryDeclarationEntryOrchestrationDenied::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    progress.progression_contract().reason(),
                    digest,
                ),
            )
        }
        crate::application::ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::Stale(
                ForgeQueryDeclarationEntryOrchestrationStale::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression requires stale-readable review before admission",
                    digest,
                ),
            )
        }
        crate::application::ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::RebindRequired(
                ForgeQueryDeclarationEntryOrchestrationRebindRequired::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression requires explicit rebind before lowering",
                    digest,
                ),
            )
        }
        crate::application::ForgeQueryDeclarationProgressionChecked::Failed(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
            ));
            ForgeQueryDeclarationEntryOrchestrationChecked::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    progress.progression_contract().reason(),
                    digest,
                ),
            )
        }
    }
}

fn progression_deferred<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    stage_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    progress: crate::application::ForgeQueryDeclarationProgressionDeferred<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    let digest = Some(progress.progression_digest().to_string());
    stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
        ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
        digest.clone(),
    ));
    ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(
        ForgeQueryDeclarationEntryOrchestrationDeferred::new(
            progress.declaration_family_key(),
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            progress.progression_contract().reason(),
            digest,
        ),
    )
}
