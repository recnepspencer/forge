use worth_query_installation::facade::{
    WorthQueryInstalledArtifactContractAuthority, WorthQueryPortableArtifactContract,
    WorthQueryStructuralCounterRequiredness, WorthQueryTransformationEvidenceContract,
};

use super::admitted::WorthQueryAdmittedDomainEvidenceParts;
use super::material::WorthQueryDomainEvidenceMaterialParts;
use super::{
    WorthQueryAdmittedDecisionSummary, WorthQueryAdmittedDomainEvidence,
    WorthQueryAdmittedDomainEvidenceSidecar, WorthQueryAdmittedStructuralCounter,
    WorthQueryCandidateRecord, WorthQueryCandidateSearchSummary, WorthQueryDecisionRecord,
    WorthQueryDomainEvidenceAdmissionDenial, WorthQueryDomainEvidenceAdmissionDenialKind,
    WorthQueryDomainEvidenceAdmissionLedger, WorthQueryDomainEvidenceBinding,
    WorthQueryDomainEvidenceCore, WorthQueryDomainEvidenceGovernance,
    WorthQueryDomainEvidenceMaterial, WorthQueryTransformationRecord,
    WorthQueryTransformationSummary,
};

pub struct WorthQueryDomainEvidenceAdmissionInput<'a> {
    pub material: Option<WorthQueryDomainEvidenceMaterial>,
    pub binding:
        worth_query_execution::facade::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
    pub ledger: Option<&'a mut WorthQueryDomainEvidenceAdmissionLedger>,
}

pub fn admit_domain_evidence(
    input: WorthQueryDomainEvidenceAdmissionInput<'_>,
) -> Result<Option<WorthQueryAdmittedDomainEvidence>, WorthQueryDomainEvidenceAdmissionDenial> {
    validate_binding(&input.binding)?;
    let Some((authority, material)) =
        resolve_evidence_presence(input.binding.contract(), input.material)?
    else {
        return Ok(None);
    };
    let binding = WorthQueryDomainEvidenceBinding::from_execution(&input.binding);
    let contract = authority.contract();
    let contract_identity = contract.identity().as_str().to_owned();
    let prepared = prepare_material(contract, material);
    let admitted = admit_material(
        contract,
        prepared,
        input.binding.output_occurrence_identity(),
    )?;
    if let Some(ledger) = input.ledger {
        ledger.validate_and_retain(&contract_identity, &admitted.counters)?;
    }
    let content = materialize_content(contract, admitted);
    Ok(Some(assemble_evidence(contract_identity, binding, content)))
}

fn resolve_evidence_presence(
    contract: Option<&WorthQueryInstalledArtifactContractAuthority>,
    material: Option<WorthQueryDomainEvidenceMaterial>,
) -> Result<
    Option<(
        &WorthQueryInstalledArtifactContractAuthority,
        WorthQueryDomainEvidenceMaterial,
    )>,
    WorthQueryDomainEvidenceAdmissionDenial,
> {
    match (contract, material) {
        (None, None) => Ok(None),
        (Some(_), None) => Err(WorthQueryDomainEvidenceAdmissionDenial::new(
            WorthQueryDomainEvidenceAdmissionDenialKind::MissingRequiredMaterial,
            "installed-domain-evidence",
        )),
        (None, Some(_)) => Err(WorthQueryDomainEvidenceAdmissionDenial::new(
            WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredMaterial,
            "uninstalled-domain-evidence",
        )),
        (Some(authority), Some(material)) => Ok(Some((authority, material))),
    }
}

struct PreparedMaterial {
    parts: WorthQueryDomainEvidenceMaterialParts,
    decision_records: Option<Vec<WorthQueryDecisionRecord>>,
    candidate_records: Option<Vec<WorthQueryCandidateRecord>>,
    transformation_records: Option<Vec<WorthQueryTransformationRecord>>,
}

fn prepare_material(
    contract: &WorthQueryPortableArtifactContract,
    material: WorthQueryDomainEvidenceMaterial,
) -> PreparedMaterial {
    let mut material = material.into_parts();
    let process_sidecars = super::sidecar_policy::process_supplied_records(contract.governance());
    if !process_sidecars {
        material.counters.retain(|observation| {
            contract
                .counters()
                .row(observation.name())
                .is_none_or(|schema| {
                    schema.requiredness() == WorthQueryStructuralCounterRequiredness::RequiredCore
                })
        });
    }
    let (mut decision_records, mut candidate_records, mut transformation_records) =
        std::mem::take(&mut material.sidecar).into_parts();
    if !process_sidecars {
        decision_records = None;
        candidate_records = None;
        transformation_records = None;
    }
    PreparedMaterial {
        parts: material,
        decision_records,
        candidate_records,
        transformation_records,
    }
}

struct AdmittedMaterial {
    counters: Vec<WorthQueryAdmittedStructuralCounter>,
    decisions: Vec<WorthQueryAdmittedDecisionSummary>,
    candidate_search: Option<WorthQueryCandidateSearchSummary>,
    transformation: Option<WorthQueryTransformationSummary>,
    decision_records: Option<Vec<WorthQueryDecisionRecord>>,
    candidate_records: Option<Vec<WorthQueryCandidateRecord>>,
    transformation_records: Option<Vec<WorthQueryTransformationRecord>>,
}

fn admit_material(
    contract: &WorthQueryPortableArtifactContract,
    prepared: PreparedMaterial,
    output_occurrence_identity: &str,
) -> Result<AdmittedMaterial, WorthQueryDomainEvidenceAdmissionDenial> {
    let counters =
        super::counter_admission::admit_counters(contract.counters(), prepared.parts.counters)?;
    let decisions = super::decision_admission::admit_decisions(
        contract.decisions(),
        prepared.parts.decisions,
        prepared.decision_records.as_deref(),
    )?;
    let candidate_search = super::search_admission::admit_candidate_search(
        contract.search(),
        prepared.parts.candidate_search,
        prepared.candidate_records.as_deref(),
    )?;
    let transformation = super::transformation_admission::admit_transformation(
        contract.transformation(),
        prepared.parts.transformation,
        prepared.transformation_records.as_deref(),
        output_occurrence_identity,
    )?;
    Ok(AdmittedMaterial {
        counters,
        decisions,
        candidate_search,
        transformation,
        decision_records: prepared.decision_records,
        candidate_records: prepared.candidate_records,
        transformation_records: prepared.transformation_records,
    })
}

struct AdmittedContent {
    governance: WorthQueryDomainEvidenceGovernance,
    core: WorthQueryDomainEvidenceCore,
    counter_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryAdmittedStructuralCounter>,
    decision_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryTransformationRecord>,
}

fn materialize_content(
    contract: &WorthQueryPortableArtifactContract,
    admitted: AdmittedMaterial,
) -> AdmittedContent {
    let (counters, optional_counters): (Vec<_>, Vec<_>) =
        admitted.counters.into_iter().partition(|counter| {
            counter.schema().requiredness() == WorthQueryStructuralCounterRequiredness::RequiredCore
        });
    let core = WorthQueryDomainEvidenceCore {
        counters,
        decisions: admitted.decisions,
        candidate_search: admitted.candidate_search,
        transformation: admitted.transformation,
    };
    let counter_sidecar = super::sidecar_policy::materialize_sidecar(
        super::sidecar_policy::WorthQuerySidecarMaterializationPolicy {
            applicable: contract.counters().rows().iter().any(|schema| {
                schema.requiredness() == WorthQueryStructuralCounterRequiredness::OptionalSidecar
            }),
            retention_allows_materialization: true,
        },
        (!optional_counters.is_empty()).then_some(optional_counters),
        contract.governance(),
        super::identity::counter_sidecar_digest,
    );
    let decision_sidecar = materialize_decision_sidecar(contract, admitted.decision_records);
    let candidate_sidecar = super::sidecar_policy::materialize_sidecar(
        super::sidecar_policy::WorthQuerySidecarMaterializationPolicy {
            applicable: contract.search().universe_family().is_some(),
            retention_allows_materialization: true,
        },
        admitted.candidate_records,
        contract.governance(),
        super::identity::candidate_sidecar_digest,
    );
    let transformation_sidecar = super::sidecar_policy::materialize_sidecar(
        super::sidecar_policy::WorthQuerySidecarMaterializationPolicy {
            applicable: matches!(
                contract.transformation(),
                WorthQueryTransformationEvidenceContract::Declared { .. }
            ),
            retention_allows_materialization: true,
        },
        admitted.transformation_records,
        contract.governance(),
        super::identity::transformation_sidecar_digest,
    );
    AdmittedContent {
        governance: WorthQueryDomainEvidenceGovernance::from_contract(contract.governance()),
        core,
        counter_sidecar,
        decision_sidecar,
        candidate_sidecar,
        transformation_sidecar,
    }
}

fn materialize_decision_sidecar(
    contract: &WorthQueryPortableArtifactContract,
    records: Option<Vec<WorthQueryDecisionRecord>>,
) -> WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord> {
    super::sidecar_policy::materialize_sidecar_by_record(
        !contract.decisions().schemas().is_empty(),
        records,
        contract.governance(),
        super::identity::decision_sidecar_digest,
        |record| {
            contract.decisions().schemas().iter().any(|schema| {
                schema.kind() == record.kind()
                    && schema.retention() == contract.governance().retention()
            })
        },
    )
}

fn assemble_evidence(
    contract_identity: String,
    binding: WorthQueryDomainEvidenceBinding,
    content: AdmittedContent,
) -> WorthQueryAdmittedDomainEvidence {
    let identity = super::identity::domain_evidence_identity(
        &contract_identity,
        &binding,
        &content.core,
        &content.counter_sidecar,
        &content.decision_sidecar,
        &content.candidate_sidecar,
        &content.transformation_sidecar,
    );
    WorthQueryAdmittedDomainEvidence::from_parts(WorthQueryAdmittedDomainEvidenceParts {
        contract_identity,
        binding,
        governance: content.governance,
        core: content.core,
        counter_sidecar: content.counter_sidecar,
        decision_sidecar: content.decision_sidecar,
        candidate_sidecar: content.candidate_sidecar,
        transformation_sidecar: content.transformation_sidecar,
        identity,
    })
}

fn validate_binding(
    binding: &worth_query_execution::facade::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    if !binding.is_current_installation_generation() {
        return Err(WorthQueryDomainEvidenceAdmissionDenial::new(
            WorthQueryDomainEvidenceAdmissionDenialKind::StaleExecutionBinding,
            "domain-evidence-binding",
        ));
    }
    let required = [
        binding.operation_identity(),
        binding.binding_identity(),
        binding.basis_identity(),
        binding.execution_snapshot_identity(),
        binding.output_occurrence_identity(),
    ];
    if required.iter().any(|value| value.trim().is_empty())
        || binding
            .run_identity()
            .is_some_and(|value| value.trim().is_empty())
        || binding
            .stage_identity()
            .is_some_and(|value| value.trim().is_empty())
    {
        return Err(WorthQueryDomainEvidenceAdmissionDenial::new(
            WorthQueryDomainEvidenceAdmissionDenialKind::InvalidPortableValue,
            "domain-evidence-binding",
        ));
    }
    Ok(())
}
