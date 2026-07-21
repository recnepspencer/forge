use worth_query::facade::{domain, runtime};

#[path = "aftermath/executors.rs"]
mod executors;
use executors::{CandidateExecutor, OriginalExecutor, ProvisionalExecutor};

use super::{
    canonical_bundle, configured_runtime_without_executors, semantic_closure, GeometryDomain,
};

#[derive(Clone, Copy, Debug)]
pub struct AftermathOriginal;

#[derive(Clone, Copy, Debug)]
pub struct AftermathCandidate;

#[derive(Clone, Copy, Debug)]
pub struct ProvisionalWorkflow;

#[derive(Clone, Copy, Debug)]
pub struct AftermathFamily;

#[derive(Clone, Copy, Debug)]
pub enum AftermathContract {
    ExactInverse,
    Compensation,
    FalsePostcondition,
    CandidateFailureAfterEffect,
    WrongInverseTarget,
    Irreversible,
    IncompleteCompensation,
    RebuildRequired,
}

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, AftermathFamily>
    for AftermathOriginal
{
    type Input = ();
    type Output = ();
    type Publication = domain::WorthQueryTerminalOperation;
    type Execution = domain::WorthQueryWorkflowOperation;
}

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, AftermathFamily>
    for AftermathCandidate
{
    type Input = ();
    type Output = ();
    type Publication = domain::WorthQueryTerminalOperation;
    type Execution = domain::WorthQueryWorkflowOperation;
}

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, AftermathFamily>
    for ProvisionalWorkflow
{
    type Input = ();
    type Output = ();
    type Publication = domain::WorthQueryTerminalOperation;
    type Execution = domain::WorthQueryWorkflowOperation;
}

pub fn aftermath_workspace(
    name: &str,
    contract: AftermathContract,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let original = operation_semantics(aftermath_reversal(contract));
    let candidate = operation_semantics(domain::WorthQueryOperationReversalContract::Irreversible);
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        AftermathOriginal,
        AftermathFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("aftermath-original", 1),
        original,
    ))
    .operation(domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        AftermathCandidate,
        AftermathFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("aftermath-candidate", 1),
        candidate,
    ));
    configured_runtime_without_executors(package)
        .workflow_stage_executor(
            GeometryDomain,
            AftermathOriginal,
            AftermathFamily,
            OriginalExecutor,
        )
        .workflow_stage_executor(
            GeometryDomain,
            AftermathCandidate,
            AftermathFamily,
            CandidateExecutor::new(
                matches!(contract, AftermathContract::WrongInverseTarget),
                matches!(contract, AftermathContract::CandidateFailureAfterEffect),
            ),
        )
        .workspace(name)
}

pub fn provisional_workflow_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::NotRequired,
        false,
    );
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::NotRequired;
    semantics.reversal = domain::WorthQueryOperationReversalContract::ProvisionalDiscard;
    semantics.lowering.family = "provisional-workflow-v1".into();
    semantics.terminal.failure_classes = vec![
        domain::WorthQueryOperationFailureClass::InvalidInput,
        domain::WorthQueryOperationFailureClass::Dependency,
    ];
    semantics.workflow =
        domain::WorthQueryOperationWorkflowContract::Declared(one_stage_workflow(Vec::new()));
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        ProvisionalWorkflow,
        AftermathFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("provisional-workflow", 1),
        semantics,
    ));
    configured_runtime_without_executors(package)
        .workflow_stage_executor(
            GeometryDomain,
            ProvisionalWorkflow,
            AftermathFamily,
            ProvisionalExecutor,
        )
        .workspace(name)
}

fn operation_semantics(
    reversal: domain::WorthQueryOperationReversalContract,
) -> domain::WorthQueryDomainOperationSemanticClosure {
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::NotRequired,
        false,
    );
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::NotRequired;
    semantics.effects = domain::WorthQueryOperationEffectContract::Declared {
        effect_families: vec![domain::WorthQueryOperationEffectFamily::Mutation],
    };
    semantics.reversal = reversal;
    semantics.lowering.family = "aftermath-mutation-v1".into();
    semantics.terminal.failure_classes = vec![
        domain::WorthQueryOperationFailureClass::InvalidInput,
        domain::WorthQueryOperationFailureClass::Dependency,
    ];
    semantics.workflow =
        domain::WorthQueryOperationWorkflowContract::Declared(one_stage_workflow(vec![
            domain::WorthQueryOperationEffectFamily::Mutation,
        ]));
    semantics
}

fn one_stage_workflow(
    effect_roles: Vec<domain::WorthQueryOperationEffectFamily>,
) -> domain::WorthQueryPortableWorkflowDefinition {
    let mut cost_roles = vec![
        domain::WorthQueryWorkflowCostRole::Admission,
        domain::WorthQueryWorkflowCostRole::Execution,
        domain::WorthQueryWorkflowCostRole::ResultValidation,
    ];
    if !effect_roles.is_empty() {
        cost_roles.push(domain::WorthQueryWorkflowCostRole::Effect);
    }
    domain::WorthQueryPortableWorkflowDefinition::new(
        "apply",
        [domain::WorthQueryPortableWorkflowStage::new(
            "apply",
            std::iter::empty::<&str>(),
            true,
            false,
            std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
        )
        .with_semantics(domain::WorthQueryWorkflowStageSemantics {
            input: domain::WorthQueryWorkflowValueContract::EntityIdentity,
            output: domain::WorthQueryWorkflowValueContract::Text,
            effect_roles,
            cost_roles,
            terminal_result_states: vec![domain::WorthQueryOperationResultState::Ready],
            failure_classes: vec![
                domain::WorthQueryOperationFailureClass::InvalidInput,
                domain::WorthQueryOperationFailureClass::Dependency,
            ],
            ..Default::default()
        })],
    )
}

fn aftermath_reversal(contract: AftermathContract) -> domain::WorthQueryOperationReversalContract {
    match contract {
        AftermathContract::ExactInverse | AftermathContract::WrongInverseTarget => {
            domain::WorthQueryOperationReversalContract::ExactInverseWithPostcondition {
                operation: domain::WorthQueryDomainOperationIdentity::new("aftermath-candidate", 1),
                lowering_family: "aftermath-mutation-v1".into(),
                postcondition: domain::WorthQueryAftermathPostcondition::ExactPriorTruth,
            }
        }
        AftermathContract::Compensation
        | AftermathContract::FalsePostcondition
        | AftermathContract::CandidateFailureAfterEffect => {
            domain::WorthQueryOperationReversalContract::CompensationWithPostcondition {
                operation: domain::WorthQueryDomainOperationIdentity::new("aftermath-candidate", 1),
                postcondition: domain::WorthQueryAftermathPostcondition::BusinessPostcondition {
                    identity: if matches!(contract, AftermathContract::FalsePostcondition) {
                        "unestablished-postcondition".into()
                    } else {
                        "original-obligation-settled".into()
                    },
                },
            }
        }
        AftermathContract::Irreversible => {
            domain::WorthQueryOperationReversalContract::Irreversible
        }
        AftermathContract::IncompleteCompensation => {
            domain::WorthQueryOperationReversalContract::Compensation {
                operation: domain::WorthQueryDomainOperationIdentity::new("aftermath-candidate", 1),
            }
        }
        AftermathContract::RebuildRequired => {
            domain::WorthQueryOperationReversalContract::RebuildRequired {
                recovery_family: "geometry-rebuild-v1".into(),
            }
        }
    }
}
