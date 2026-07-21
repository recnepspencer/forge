use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis,
};
use crate::domain_installation::operation_identity_basis::{
    canonical_operation_identity, lineage_outcome_material,
};
use crate::domain_installation::{
    WorthQueryCompletedWorkflowTrace, WorthQueryConditionalOutcomeClass,
    WorthQueryOperationLineageContract,
};

use super::report::{lineage_evidence, WorthQueryStageLineageDeclaration};
use super::{WorthQueryTraceLineageCounters, WorthQueryTraceLineageReport};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLineageBindingDenial {
    StaleInstallationGeneration,
    LineageAlreadyBound,
    LineageNotDeclared,
    EmptyDeclarationSet,
    DuplicateStageDeclaration,
    UnknownStage,
    PreserveContractMismatch,
    IdentityEvolutionIsNotAuthoritative,
    ConditionalStageDidNotEstablishFreshLineage,
}

type TraceLineageBindingResult<D, O, F, L> = Result<
    WorthQueryCompletedWorkflowTrace<D, O, F, L>,
    (
        WorthQueryCompletedWorkflowTrace<D, O, F, L>,
        WorthQueryLineageBindingDenial,
    ),
>;

pub(crate) fn bind_execution_lineage<D, O, F, L: BasisOperationLane>(
    trace: WorthQueryCompletedWorkflowTrace<D, O, F, L>,
) -> TraceLineageBindingResult<D, O, F, L> {
    let declarations = trace
        .stage_receipts()
        .iter()
        .filter(|receipt| !receipt.lineage.is_empty())
        .map(|receipt| {
            WorthQueryStageLineageDeclaration::from_execution(
                receipt.stage_identity().to_owned(),
                receipt.lineage.clone(),
            )
        })
        .collect::<Vec<_>>();
    if trace.bound().definition().semantics().lineage
        == WorthQueryOperationLineageContract::NotRequired
        && declarations.is_empty()
    {
        return Ok(trace);
    }
    bind_trace_lineage(trace, declarations)
}

fn bind_trace_lineage<D, O, F, L: BasisOperationLane>(
    mut trace: WorthQueryCompletedWorkflowTrace<D, O, F, L>,
    declarations: Vec<WorthQueryStageLineageDeclaration>,
) -> TraceLineageBindingResult<D, O, F, L> {
    if !trace.bound().installation_is_current() {
        return Err((
            trace,
            WorthQueryLineageBindingDenial::StaleInstallationGeneration,
        ));
    }
    if trace.lineage_report().is_some() {
        return Err((trace, WorthQueryLineageBindingDenial::LineageAlreadyBound));
    }
    let contract = trace.bound().definition().semantics().lineage;
    if matches!(contract, WorthQueryOperationLineageContract::NotRequired) {
        return Err((trace, WorthQueryLineageBindingDenial::LineageNotDeclared));
    }
    if declarations.is_empty() {
        return Err((trace, WorthQueryLineageBindingDenial::EmptyDeclarationSet));
    }
    let mut seen_stages = std::collections::BTreeSet::new();
    let stage_index = trace
        .stage_receipts()
        .iter()
        .map(|receipt| (receipt.stage_identity(), receipt))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut counters = WorthQueryTraceLineageCounters {
        indexed_trace_stages: stage_index.len(),
        indexed_effect_receipts: stage_index
            .values()
            .map(|receipt| receipt.effect_evidence().len())
            .sum(),
        ..Default::default()
    };
    let mut evidence = Vec::new();
    for declaration in declarations {
        if !seen_stages.insert(declaration.stage_identity().to_owned()) {
            return Err((
                trace,
                WorthQueryLineageBindingDenial::DuplicateStageDeclaration,
            ));
        }
        counters.stage_lookups += 1;
        let Some(receipt) = stage_index.get(declaration.stage_identity()).copied() else {
            return Err((trace, WorthQueryLineageBindingDenial::UnknownStage));
        };
        if !receipt.conditional_provenance().is_empty() {
            counters.conditional_path_checks += 1;
            if !receipt
                .conditional_provenance()
                .iter()
                .any(|item| item.class() == WorthQueryConditionalOutcomeClass::ComputedChanged)
            {
                return Err((
                    trace,
                    WorthQueryLineageBindingDenial::ConditionalStageDidNotEstablishFreshLineage,
                ));
            }
        }
        for outcome in declaration.outcomes() {
            counters.outcome_contract_checks += 1;
            counters.outcome_width += outcome.width();
            if contract == WorthQueryOperationLineageContract::Preserve
                && outcome.kind()
                    != crate::identity_evolution::InstalledIdentityEvolutionKind::PreservedIdentity
            {
                return Err((
                    trace,
                    WorthQueryLineageBindingDenial::PreserveContractMismatch,
                ));
            }
            let Some(foundational_lineage) = outcome.foundational_attested_lineage() else {
                return Err((
                    trace,
                    WorthQueryLineageBindingDenial::IdentityEvolutionIsNotAuthoritative,
                ));
            };
            let effect_receipts = outcome
                .establishing_effect_receipt_identity()
                .map(|identity| vec![identity.to_owned()])
                .unwrap_or_default();
            counters.effect_receipt_attachments += effect_receipts.len();
            evidence.push(lineage_evidence(
                receipt.stage_identity().to_owned(),
                receipt.identity().to_owned(),
                effect_receipts,
                outcome.clone(),
                foundational_lineage,
            ));
        }
    }
    let identity = lineage_report_identity(trace.identity(), &evidence);
    let proof = mint_operation_phase_proof(
        identity.clone(),
        Some(trace.phase_proof().payload().identity()),
        operation_phase_basis(trace.phase_proof()).clone(),
    );
    trace.lineage = Some(WorthQueryTraceLineageReport {
        identity,
        trace_identity: trace.identity().to_owned(),
        evidence,
        counters,
        proof,
    });
    trace.refresh_semantic_identity_for_lineage();
    Ok(trace)
}

fn lineage_report_identity(
    trace_identity: &str,
    evidence: &[super::WorthQueryTraceLineageEvidence],
) -> String {
    canonical_operation_identity(
        "trace-lineage-report-v2",
        vec![
            ("lineage.trace", trace_identity.to_owned()),
            (
                "lineage.evidence",
                crate::domain_installation::operation_identity_basis::canonical_indexed_operation_material(
                    "lineage.evidence",
                    evidence.iter().map(|item| {
                        crate::domain_installation::operation_identity_basis::canonical_operation_material(vec![
                            ("lineage.stage", item.stage_identity().to_owned()),
                            ("lineage.stage_receipt", item.stage_receipt_identity().to_owned()),
                            (
                                "lineage.effect_receipts",
                                crate::domain_installation::operation_identity_basis::canonical_indexed_operation_material(
                                    "lineage.effect_receipt",
                                    item.effect_receipt_identities().iter().cloned(),
                                ),
                            ),
                            ("lineage.outcome", lineage_outcome_material(item.outcome())),
                        ])
                    }),
                ),
            ),
        ],
    )
}
