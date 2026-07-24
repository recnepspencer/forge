use worth_query_installation::facade::{
    WorthQueryInstalledArtifactContractAuthority, WorthQueryPortableArtifactContract,
    WorthQueryStructuralCounterRequiredness, WorthQueryTransformationEvidenceContract,
};

use super::admitted::{
    WorthQueryAdmittedDomainEvidenceParts, WorthQueryDomainEvidenceBindingParts,
};
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

pub(crate) struct WorthQueryDomainEvidenceAdmissionInput<'a> {
    pub(crate) contract: Option<&'a WorthQueryInstalledArtifactContractAuthority>,
    pub(crate) material: Option<WorthQueryDomainEvidenceMaterial>,
    pub(crate) binding: WorthQueryDomainEvidenceBindingParts,
    pub(crate) ledger: Option<&'a mut WorthQueryDomainEvidenceAdmissionLedger>,
}

pub(crate) fn admit_domain_evidence(
    input: WorthQueryDomainEvidenceAdmissionInput<'_>,
) -> Result<Option<WorthQueryAdmittedDomainEvidence>, WorthQueryDomainEvidenceAdmissionDenial> {
    let Some((authority, material)) = resolve_evidence_presence(input.contract, input.material)?
    else {
        return Ok(None);
    };
    validate_binding(&input.binding)?;
    let contract = authority.contract();
    let contract_identity = contract.identity().as_str().to_owned();
    let prepared = prepare_material(contract, material);
    let admitted = admit_material(
        contract,
        prepared,
        &input.binding.output_occurrence_identity,
    )?;
    if let Some(ledger) = input.ledger {
        ledger.validate_and_retain(&contract_identity, &admitted.counters)?;
    }
    let content = materialize_content(contract, admitted);
    Ok(Some(assemble_evidence(
        contract_identity,
        input.binding,
        content,
    )))
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
        contract.counters().rows().iter().any(|schema| {
            schema.requiredness() == WorthQueryStructuralCounterRequiredness::OptionalSidecar
        }),
        (!optional_counters.is_empty()).then_some(optional_counters),
        contract.governance(),
        super::identity::counter_sidecar_digest,
    );
    let decision_sidecar = super::sidecar_policy::materialize_sidecar(
        !contract.decisions().schemas().is_empty(),
        admitted.decision_records,
        contract.governance(),
        super::identity::decision_sidecar_digest,
    );
    let candidate_sidecar = super::sidecar_policy::materialize_sidecar(
        contract.search().universe_family().is_some(),
        admitted.candidate_records,
        contract.governance(),
        super::identity::candidate_sidecar_digest,
    );
    let transformation_sidecar = super::sidecar_policy::materialize_sidecar(
        matches!(
            contract.transformation(),
            WorthQueryTransformationEvidenceContract::Declared { .. }
        ),
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

fn assemble_evidence(
    contract_identity: String,
    binding: WorthQueryDomainEvidenceBindingParts,
    content: AdmittedContent,
) -> WorthQueryAdmittedDomainEvidence {
    let binding = WorthQueryDomainEvidenceBinding::from_parts(binding);
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
    binding: &WorthQueryDomainEvidenceBindingParts,
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let required = [
        binding.operation_identity.as_str(),
        binding.binding_identity.as_str(),
        binding.basis_identity.as_str(),
        binding.execution_snapshot_identity.as_str(),
        binding.output_occurrence_identity.as_str(),
    ];
    if required.iter().any(|value| value.trim().is_empty())
        || binding
            .run_identity
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        || binding
            .stage_identity
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
    {
        return Err(WorthQueryDomainEvidenceAdmissionDenial::new(
            WorthQueryDomainEvidenceAdmissionDenialKind::InvalidPortableValue,
            "domain-evidence-binding",
        ));
    }
    Ok(())
}
