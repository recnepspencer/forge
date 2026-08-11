use worth_query_installation::facade::{
    WorthQueryInstalledArtifactContractAuthority, WorthQueryPortableArtifactContract,
    WorthQueryStructuralCounterRequiredness, WorthQueryTransformationEvidenceContract,
};

use super::material::WorthQueryDomainEvidenceMaterialParts;
use super::{
    WorthQueryAdmittedDecisionSummary, WorthQueryAdmittedDomainEvidenceSidecar,
    WorthQueryAdmittedStructuralCounter, WorthQueryCandidateRecord,
    WorthQueryCandidateSearchSummary, WorthQueryDecisionRecord,
    WorthQueryDomainEvidenceAdmissionDenial, WorthQueryDomainEvidenceAdmissionDenialKind,
    WorthQueryDomainEvidenceAdmissionLedger, WorthQueryDomainEvidenceCore,
    WorthQueryDomainEvidenceGovernance, WorthQueryDomainEvidenceMaterial,
    WorthQueryTransformationRecord, WorthQueryTransformationSummary,
};

/// Descriptive material admitted against an installed evidence contract.
///
/// This value carries no execution authority and cannot attach itself to an
/// execution or workflow-stage receipt. Only this admission owner can mint it.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedDomainEvidenceContent {
    contract_identity: String,
    governance: WorthQueryDomainEvidenceGovernance,
    core: WorthQueryDomainEvidenceCore,
    counter_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryAdmittedStructuralCounter>,
    decision_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryTransformationRecord>,
    identity: String,
}

impl WorthQueryAdmittedDomainEvidenceContent {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    pub fn governance(&self) -> &WorthQueryDomainEvidenceGovernance {
        &self.governance
    }

    pub fn core(&self) -> &WorthQueryDomainEvidenceCore {
        &self.core
    }

    pub fn counter_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryAdmittedStructuralCounter> {
        &self.counter_sidecar
    }

    pub fn decision_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord> {
        &self.decision_sidecar
    }

    pub fn candidate_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord> {
        &self.candidate_sidecar
    }

    pub fn transformation_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryTransformationRecord> {
        &self.transformation_sidecar
    }
}

pub struct WorthQueryDomainEvidenceContentAdmissionInput<'a> {
    pub contract: Option<&'a WorthQueryInstalledArtifactContractAuthority>,
    pub material: Option<WorthQueryDomainEvidenceMaterial>,
    pub ledger: Option<&'a mut WorthQueryDomainEvidenceAdmissionLedger>,
}

pub fn admit_domain_evidence_content(
    input: WorthQueryDomainEvidenceContentAdmissionInput<'_>,
) -> Result<Option<WorthQueryAdmittedDomainEvidenceContent>, WorthQueryDomainEvidenceAdmissionDenial>
{
    let Some((authority, material)) = resolve_evidence_presence(input.contract, input.material)?
    else {
        return Ok(None);
    };
    let contract = authority.contract();
    let contract_identity = contract.identity().as_str().to_owned();
    let prepared = prepare_material(contract, material);
    let admitted = admit_material(contract, prepared)?;
    if let Some(ledger) = input.ledger {
        ledger.validate_and_retain(&contract_identity, &admitted.counters)?;
    }
    let content = materialize_content(contract, admitted);
    Ok(Some(assemble_content(contract_identity, content)))
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

fn assemble_content(
    contract_identity: String,
    content: AdmittedContent,
) -> WorthQueryAdmittedDomainEvidenceContent {
    let identity = super::identity::domain_evidence_content_identity(
        &contract_identity,
        &content.core,
        &content.counter_sidecar,
        &content.decision_sidecar,
        &content.candidate_sidecar,
        &content.transformation_sidecar,
    );
    WorthQueryAdmittedDomainEvidenceContent {
        contract_identity,
        governance: content.governance,
        core: content.core,
        counter_sidecar: content.counter_sidecar,
        decision_sidecar: content.decision_sidecar,
        candidate_sidecar: content.candidate_sidecar,
        transformation_sidecar: content.transformation_sidecar,
        identity,
    }
}
