use worth_query_installation::facade::{
    WorthQueryInstalledArtifactContractAuthority, WorthQueryTransformationEvidenceContract,
};

use super::admitted::{
    WorthQueryAdmittedDomainEvidenceParts, WorthQueryDomainEvidenceBindingParts,
};
use super::{
    WorthQueryAdmittedDomainEvidence, WorthQueryDomainEvidenceAdmissionDenial,
    WorthQueryDomainEvidenceAdmissionDenialKind, WorthQueryDomainEvidenceAdmissionLedger,
    WorthQueryDomainEvidenceBinding, WorthQueryDomainEvidenceCore,
    WorthQueryDomainEvidenceGovernance, WorthQueryDomainEvidenceMaterial,
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
    let (authority, material) = match (input.contract, input.material) {
        (None, None) => return Ok(None),
        (Some(_), None) => {
            return Err(WorthQueryDomainEvidenceAdmissionDenial::new(
                WorthQueryDomainEvidenceAdmissionDenialKind::MissingRequiredMaterial,
                "installed-domain-evidence",
            ))
        }
        (None, Some(_)) => {
            return Err(WorthQueryDomainEvidenceAdmissionDenial::new(
                WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredMaterial,
                "uninstalled-domain-evidence",
            ))
        }
        (Some(authority), Some(material)) => (authority, material),
    };
    validate_binding(&input.binding)?;
    let contract = authority.contract();
    let contract_identity = contract.identity().as_str().to_owned();
    let material = material.into_parts();
    let (decision_records, candidate_records, transformation_records) =
        material.sidecar.into_parts();
    let counters =
        super::counter_admission::admit_counters(contract.counters(), material.counters)?;
    let decisions = super::decision_admission::admit_decisions(
        contract.decisions(),
        material.decisions,
        decision_records.as_deref(),
    )?;
    let candidate_search = super::search_admission::admit_candidate_search(
        contract.search(),
        material.candidate_search,
        candidate_records.as_deref(),
    )?;
    let transformation = super::transformation_admission::admit_transformation(
        contract.transformation(),
        material.transformation,
        transformation_records.as_deref(),
        &input.binding.output_occurrence_identity,
    )?;
    if let Some(ledger) = input.ledger {
        ledger.validate_and_retain(&contract_identity, &counters)?;
    }
    let governance = WorthQueryDomainEvidenceGovernance::from_contract(contract.governance());
    let core = WorthQueryDomainEvidenceCore {
        counters,
        decisions,
        candidate_search,
        transformation,
    };
    let decision_sidecar = super::sidecar_policy::materialize_sidecar(
        !contract.decisions().schemas().is_empty(),
        decision_records,
        contract.governance(),
        super::identity::decision_sidecar_digest,
    );
    let candidate_sidecar = super::sidecar_policy::materialize_sidecar(
        contract.search().universe_family().is_some(),
        candidate_records,
        contract.governance(),
        super::identity::candidate_sidecar_digest,
    );
    let transformation_sidecar = super::sidecar_policy::materialize_sidecar(
        matches!(
            contract.transformation(),
            WorthQueryTransformationEvidenceContract::Declared { .. }
        ),
        transformation_records,
        contract.governance(),
        super::identity::transformation_sidecar_digest,
    );
    let binding = WorthQueryDomainEvidenceBinding::from_parts(input.binding);
    let identity = super::identity::domain_evidence_identity(
        &contract_identity,
        &binding,
        &core,
        &decision_sidecar,
        &candidate_sidecar,
        &transformation_sidecar,
    );
    Ok(Some(WorthQueryAdmittedDomainEvidence::from_parts(
        WorthQueryAdmittedDomainEvidenceParts {
            contract_identity,
            binding,
            governance,
            core,
            decision_sidecar,
            candidate_sidecar,
            transformation_sidecar,
            identity,
        },
    )))
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
