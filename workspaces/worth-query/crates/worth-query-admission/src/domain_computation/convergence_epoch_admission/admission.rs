use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryCandidateSearchPosture, WorthQueryInstalledArtifactContractAuthority,
    WorthQueryInstalledDomainOperationAuthority, WorthQueryOperationWorkflowContract,
};

use super::evidence::WorthQueryConvergenceContractBinding;
use super::{
    WorthQueryAdmittedConvergenceContract, WorthQueryConvergenceAdmissionCounters,
    WorthQueryConvergenceAdmissionDenial, WorthQueryConvergenceAdmissionDenialKind as Kind,
    WorthQueryConvergenceAdmissionRejection,
};

pub fn admit_convergence_epoch_contract(
    operation: &WorthQueryInstalledDomainOperationAuthority,
    artifact: WorthQueryInstalledArtifactContractAuthority,
) -> Result<WorthQueryAdmittedConvergenceContract, WorthQueryConvergenceAdmissionRejection> {
    let mut counters = WorthQueryConvergenceAdmissionCounters::default();
    counters.checked_installed_authority();
    if !artifact.belongs_to_operation_installation(operation) {
        return Err(rejection(
            denial(
                Kind::ForeignInstalledAuthorities,
                "operation and convergence artifact do not share one installed authority",
                counters,
            ),
            artifact,
        ));
    }

    counters.checked_operation_evidence();
    let evidence_stage_identity =
        match resolve_evidence_stage(operation, &artifact.contract().reference(), counters) {
            Ok(stage) => stage,
            Err(denial) => return Err(rejection(denial, artifact)),
        };

    counters.checked_convergence_contract();
    let binding =
        match bind_convergence_contract(operation, &artifact, evidence_stage_identity, counters) {
            Ok(binding) => binding,
            Err(denial) => return Err(rejection(denial, artifact)),
        };
    let identity = convergence_admission_identity(operation, &artifact, &binding);

    Ok(WorthQueryAdmittedConvergenceContract::new(
        identity,
        Arc::from(operation.definition().canonical_identity()),
        Arc::from(operation.owner()),
        operation.runtime_ordinal(),
        operation.generation(),
        artifact,
        binding,
        counters,
    ))
}

fn bind_convergence_contract(
    operation: &WorthQueryInstalledDomainOperationAuthority,
    artifact: &WorthQueryInstalledArtifactContractAuthority,
    evidence_stage_identity: Option<String>,
    counters: WorthQueryConvergenceAdmissionCounters,
) -> Result<WorthQueryConvergenceContractBinding, WorthQueryConvergenceAdmissionDenial> {
    let contract = artifact.contract();
    let convergence = contract.convergence();
    let Some(iteration_bound) = convergence.iteration_bound() else {
        return Err(denial(
            Kind::NonIterativeContract,
            "installed artifact is not an iterative convergence contract",
            counters,
        ));
    };
    if matches!(
        contract.search().search_posture(),
        WorthQueryCandidateSearchPosture::NotApplicable
    ) {
        return Err(denial(
            Kind::MissingCandidateSearch,
            "iterative convergence requires an installed candidate-search contract",
            counters,
        ));
    }

    let resources = &operation.definition().semantics().resources;

    Ok(WorthQueryConvergenceContractBinding {
        artifact_contract_identity: Arc::from(contract.identity().as_str()),
        evidence_stage_identity: evidence_stage_identity.map(Arc::from),
        resource_contract_identity: Arc::from(resources.canonical_identity()),
        universe_family: Arc::from(
            contract
                .search()
                .universe_family()
                .expect("validated search"),
        ),
        termination_family: Arc::from(
            contract
                .search()
                .termination_family()
                .expect("validated search"),
        ),
        feasibility_family: Arc::from(
            contract
                .search()
                .feasibility_family()
                .expect("validated search"),
        ),
        comparison_family: Arc::from(
            contract
                .search()
                .comparison_family()
                .expect("validated search"),
        ),
        incumbent_family: Arc::from(
            contract
                .search()
                .incumbent_family()
                .expect("validated search"),
        ),
        progress_measure_family: Arc::from(
            convergence
                .progress_measure_family()
                .expect("validated convergence"),
        ),
        comparator_family: Arc::from(
            convergence
                .comparator_family()
                .expect("validated convergence"),
        ),
        repeated_state_family: Arc::from(
            convergence
                .repeated_state_family()
                .expect("validated convergence"),
        ),
        search_posture: contract.search().search_posture().clone(),
        optimality_posture: contract.search().optimality_posture().clone(),
        incumbent_posture: convergence
            .incumbent_posture()
            .expect("validated convergence"),
        oscillation_posture: convergence
            .oscillation_posture()
            .expect("validated convergence"),
        iteration_bound,
    })
}

fn convergence_admission_identity(
    operation: &WorthQueryInstalledDomainOperationAuthority,
    artifact: &WorthQueryInstalledArtifactContractAuthority,
    binding: &WorthQueryConvergenceContractBinding,
) -> Arc<str> {
    Arc::from(crate::admission_digest::hash_parts_with_digests(
        &[
            "worth_query_admitted_convergence_contract_v1".into(),
            format!("runtime:{}", operation.runtime_ordinal()),
            format!("generation:{}", operation.generation().ordinal()),
            format!("owner:{}", operation.owner()),
            format!("operation:{}", operation.definition().canonical_identity()),
            format!("artifact-contract:{}", binding.artifact_contract_identity),
            format!(
                "evidence-stage:{}",
                binding
                    .evidence_stage_identity
                    .as_deref()
                    .unwrap_or("direct")
            ),
            format!("resources:{}", binding.resource_contract_identity),
            format!("universe:{}", binding.universe_family),
            format!("termination:{}", binding.termination_family),
            format!("feasibility:{}", binding.feasibility_family),
            format!("comparison:{}", binding.comparison_family),
            format!("incumbent:{}", binding.incumbent_family),
            format!("progress:{}", binding.progress_measure_family),
            format!("comparator:{}", binding.comparator_family),
            format!("repeated-state:{}", binding.repeated_state_family),
            format!("iterations:{}", binding.iteration_bound),
        ],
        &[artifact.admission_identity().digest()],
    ))
}

fn resolve_evidence_stage(
    operation: &WorthQueryInstalledDomainOperationAuthority,
    artifact: &worth_query_installation::facade::WorthQueryArtifactContractReference,
    counters: WorthQueryConvergenceAdmissionCounters,
) -> Result<Option<String>, WorthQueryConvergenceAdmissionDenial> {
    match &operation.definition().semantics().workflow {
        WorthQueryOperationWorkflowContract::NotRequired => {
            let Some(reference) = operation
                .definition()
                .semantics()
                .evidence
                .artifact_reference()
            else {
                return Err(denial(
                    Kind::OperationEvidenceNotInstalled,
                    "direct operation does not declare an installed convergence artifact",
                    counters,
                ));
            };
            if reference != artifact {
                return Err(denial(
                    Kind::ArtifactContractMismatch,
                    "direct operation evidence does not name the supplied convergence artifact",
                    counters,
                ));
            }
            Ok(None)
        }
        WorthQueryOperationWorkflowContract::Declared(workflow) => {
            let stages = workflow
                .stages()
                .iter()
                .filter(|stage| stage.semantics().evidence.artifact_reference() == Some(artifact))
                .map(|stage| stage.identity().to_owned())
                .collect::<Vec<_>>();
            match stages.as_slice() {
                [stage] => Ok(Some(stage.clone())),
                [] => Err(denial(
                    Kind::OperationEvidenceNotInstalled,
                    "workflow has no stage that declares the supplied convergence artifact",
                    counters,
                )),
                _ => Err(denial(
                    Kind::AmbiguousWorkflowEvidence,
                    "workflow convergence artifact must belong to one unique evidence stage",
                    counters,
                )),
            }
        }
    }
}

fn denial(
    kind: Kind,
    detail: &'static str,
    counters: WorthQueryConvergenceAdmissionCounters,
) -> WorthQueryConvergenceAdmissionDenial {
    WorthQueryConvergenceAdmissionDenial::new(kind, detail, counters)
}

fn rejection(
    denial: WorthQueryConvergenceAdmissionDenial,
    artifact: WorthQueryInstalledArtifactContractAuthority,
) -> WorthQueryConvergenceAdmissionRejection {
    WorthQueryConvergenceAdmissionRejection::new(denial, artifact)
}
