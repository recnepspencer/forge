use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactDisposition,
    WorthQueryArtifactHandleCore, WorthQueryArtifactProductionAdmission,
    WorthQueryMoveOnlyArtifactHandle, WorthQueryPreparedArtifactResource,
    WorthQueryRuntimeArtifactBinding, WorthQueryRuntimeArtifactOwner,
};

static NEXT_ARTIFACT_OCCURRENCE: AtomicU64 = AtomicU64::new(1);

impl WorthQueryMoveOnlyArtifactHandle {
    pub(crate) fn register(
        admission: WorthQueryArtifactProductionAdmission,
        prepared: WorthQueryPreparedArtifactResource,
    ) -> Result<Self, WorthQueryArtifactDenial> {
        validate_production(&admission, &prepared)?;
        let occurrence_ordinal = NEXT_ARTIFACT_OCCURRENCE.fetch_add(1, Ordering::Relaxed);
        let contract = admission.contract.contract();
        let owner_identity = crate::identity::hash_parts(&[
            "worth_query_runtime_artifact_owner_v1".into(),
            format!(
                "runtime:{}",
                admission.domain_authority.runtime_authority().as_u64()
            ),
            format!(
                "generation:{}",
                admission
                    .domain_authority
                    .installation_generation()
                    .ordinal()
            ),
            format!("operation:{}", admission.operation_identity),
            format!("binding:{}", admission.binding_identity),
            format!("run:{}", admission.run_identity),
            format!("stage:{}", admission.stage_identity),
            format!("contract:{}", contract.identity().as_str()),
            format!("occurrence-ordinal:{occurrence_ordinal}"),
        ]);
        let occurrence_identity = crate::identity::hash_parts(&[
            "worth_query_artifact_occurrence_v1".into(),
            format!("owner:{owner_identity}"),
            format!("contract:{}", contract.identity().as_str()),
            format!("run:{}", admission.run_identity),
            format!("stage:{}", admission.stage_identity),
        ]);
        let holder_stage = admission.stage_identity.clone();
        let owner = WorthQueryRuntimeArtifactOwner::register(
            WorthQueryRuntimeArtifactBinding {
                contract: admission.contract,
                domain_authority: admission.domain_authority,
                operation_identity: admission.operation_identity,
                binding_identity: admission.binding_identity,
                run_identity: admission.run_identity,
                producing_stage: admission.stage_identity,
                basis_identity: admission.basis_identity,
                provenance_identity: admission.evidence.provenance_identity().to_owned(),
                dependency_identity: admission.evidence.dependency_identity().to_owned(),
                owner_identity,
                occurrence_identity,
            },
            prepared,
        );
        Ok(Self {
            core: WorthQueryArtifactHandleCore::new_owner(
                owner,
                holder_stage,
                WorthQueryArtifactDisposition::Produced,
            ),
        })
    }
}

fn validate_production(
    admission: &WorthQueryArtifactProductionAdmission,
    prepared: &WorthQueryPreparedArtifactResource,
) -> Result<(), WorthQueryArtifactDenial> {
    let contract = admission.contract.contract();
    if !admission
        .domain_authority
        .is_current_installation_generation()
    {
        return Err(production_denial(
            WorthQueryArtifactDenialKind::StaleInstallationGeneration,
            contract,
            "artifact production uses a stale installation generation",
        ));
    }
    if !admission.evidence.is_valid() {
        return Err(production_denial(
            WorthQueryArtifactDenialKind::InvalidProductionEvidence,
            contract,
            "artifact production evidence is incomplete",
        ));
    }
    if prepared.semantic_projection().bytes().is_empty() {
        return Err(production_denial(
            WorthQueryArtifactDenialKind::EmptySemanticProjection,
            contract,
            "provider returned an empty canonical semantic projection",
        ));
    }
    let ownership = contract.ownership();
    if ownership.provider_family() != Some(prepared.provider_family) {
        return Err(production_denial(
            WorthQueryArtifactDenialKind::ProviderFamilyMismatch,
            contract,
            "provider resource family differs from the installed artifact contract",
        ));
    }
    if ownership.payload_owner() != Some(admission.contract.owner()) {
        return Err(production_denial(
            WorthQueryArtifactDenialKind::PayloadOwnerMismatch,
            contract,
            "installed package owner does not own the declared artifact payload",
        ));
    }
    if !contract
        .producer_roles()
        .iter()
        .any(|role| role == &admission.stage_identity)
    {
        return Err(production_denial(
            WorthQueryArtifactDenialKind::ProducerRoleNotInstalled,
            contract,
            "producer stage is not admitted by the installed artifact contract",
        ));
    }
    Ok(())
}

fn production_denial(
    kind: WorthQueryArtifactDenialKind,
    contract: &worth_query_installation::facade::WorthQueryPortableArtifactContract,
    detail: &'static str,
) -> WorthQueryArtifactDenial {
    WorthQueryArtifactDenial::new(kind, Some(contract.family().as_str()), detail)
}
