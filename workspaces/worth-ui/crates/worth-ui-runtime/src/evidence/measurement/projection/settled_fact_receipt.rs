use std::hash::{DefaultHasher, Hash, Hasher};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    stable_text_digest, UiDeclarationIdentity, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementQueryDependencySet,
};
use crate::evidence::shared::query_measurement_fact_family_digest::query_measurement_fact_family_set_digest;

use super::fact_receipt::UiProjectionFactObservation;
use super::UiSettledQueryFactReceiptDenial;

/// Exact UI-owned key for one retained ordinary Query settlement source.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiSettledQueryFactKey {
    view_binding_id: crate::capability::ViewBindingId,
    binding_key: worth_ui_query_binding::WorthUiAdmittedQueryBindingKey,
    settlement_reference: worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference,
}

/// UI measurement receipt derived from a retained Query-owned settlement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSettledQueryFactReceipt {
    key: UiSettledQueryFactKey,
    binding_reference: worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    required_measurement_dependencies: Box<[UiDeclaredMeasurementEvidenceRequirement]>,
    required_query_fact_families: Box<[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily]>,
    required_query_fact_family_set_digest: u64,
    consumed_fact_families: Box<[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily]>,
    consumed_fact_family_set_digest: u64,
    observations: Box<[UiProjectionFactObservation]>,
    observation_identity_digest: u64,
    source_generation: u64,
    source_order: u64,
    partial: bool,
    warning_count: usize,
}

impl UiSettledQueryFactKey {
    pub(crate) fn new(
        view_binding_id: crate::capability::ViewBindingId,
        binding_key: worth_ui_query_binding::WorthUiAdmittedQueryBindingKey,
        settlement_reference: worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference,
    ) -> Self {
        Self {
            view_binding_id,
            binding_key,
            settlement_reference,
        }
    }

    pub(crate) fn view_binding_id(&self) -> &crate::capability::ViewBindingId {
        &self.view_binding_id
    }

    pub(crate) fn binding_key(&self) -> &worth_ui_query_binding::WorthUiAdmittedQueryBindingKey {
        &self.binding_key
    }

    pub(crate) fn settlement_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference {
        &self.settlement_reference
    }
}

impl UiSettledQueryFactReceipt {
    pub fn view_binding_id(&self) -> &crate::capability::ViewBindingId {
        &self.key.view_binding_id
    }

    pub fn binding_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiAdmittedQueryBindingReference {
        &self.binding_reference
    }

    pub fn settlement_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference {
        &self.key.settlement_reference
    }

    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn declaration_support_authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.declaration_support_authority_generation
    }

    pub fn required_measurement_dependencies(&self) -> &[UiDeclaredMeasurementEvidenceRequirement] {
        &self.required_measurement_dependencies
    }

    pub fn required_query_fact_families(
        &self,
    ) -> &[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily] {
        &self.required_query_fact_families
    }

    pub fn required_query_fact_family_set_digest(&self) -> u64 {
        self.required_query_fact_family_set_digest
    }

    pub fn consumed_fact_families(
        &self,
    ) -> &[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily] {
        &self.consumed_fact_families
    }

    pub fn consumed_fact_family_set_digest(&self) -> u64 {
        self.consumed_fact_family_set_digest
    }

    pub fn observations(&self) -> &[UiProjectionFactObservation] {
        &self.observations
    }

    pub fn observation_identity_digest(&self) -> u64 {
        self.observation_identity_digest
    }

    pub fn source_generation(&self) -> u64 {
        self.source_generation
    }

    pub fn source_order(&self) -> u64 {
        self.source_order
    }

    pub fn is_partial(&self) -> bool {
        self.partial
    }

    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    pub(crate) fn key(&self) -> &UiSettledQueryFactKey {
        &self.key
    }

    pub(crate) fn same_binding_slot(&self, other: &Self) -> bool {
        self.view_binding_id() == other.view_binding_id()
            && self.binding_reference() == other.binding_reference()
    }

    pub(crate) fn reference_reporting_digests(&self) -> (u64, u64) {
        (
            opaque_reference_digest(self.binding_reference()),
            opaque_reference_digest(self.settlement_reference()),
        )
    }
}

fn opaque_reference_digest(reference: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    reference.hash(&mut hasher);
    hasher.finish()
}

pub fn consume_settled_query_measurement_fact(
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    measurement_policy: &crate::declaration::UiDeclaredMeasurementPolicyPosture,
    view_binding_id: crate::capability::ViewBindingId,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
) -> Result<UiSettledQueryFactReceipt, UiSettledQueryFactReceiptDenial> {
    let dependencies =
        crate::declaration::declared_query_measurement_dependencies(measurement_policy)
            .ok_or(UiSettledQueryFactReceiptDenial::NoQueryMeasurementDependencies)?;
    admit_declared_measurement_settled_query_fact(
        declaration_identity,
        declaration_support_authority_generation,
        dependencies,
        view_binding_id,
        fact,
    )
}

fn admit_declared_measurement_settled_query_fact(
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    dependencies: UiDeclaredMeasurementQueryDependencySet,
    view_binding_id: crate::capability::ViewBindingId,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
) -> Result<UiSettledQueryFactReceipt, UiSettledQueryFactReceiptDenial> {
    let batch = fact.measurement_facts();
    let consumed_fact_families = batch
        .observations()
        .iter()
        .map(|observation| observation.family())
        .collect::<Vec<_>>();
    let missing = dependencies
        .fact_families()
        .iter()
        .copied()
        .filter(|family| !consumed_fact_families.contains(family))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(
            UiSettledQueryFactReceiptDenial::MissingRequiredFactFamilies {
                required: dependencies.fact_families().to_vec().into_boxed_slice(),
                consumed: consumed_fact_families.into_boxed_slice(),
            },
        );
    }
    let observations = batch
        .observations()
        .iter()
        .copied()
        .map(UiProjectionFactObservation::from_query_observation)
        .collect::<Vec<_>>();
    let observation_identity_digest = observations.iter().fold(
        stable_text_digest("worth-ui.settled-query-fact-observations"),
        |digest, observation| digest ^ observation.identity_digest().rotate_left(17),
    );
    let source_generation = fact
        .source_generation()
        .expect("only retained settled facts can mint measurement receipts")
        .as_u64();
    let source_order = fact
        .source_order()
        .expect("only retained settled facts can mint measurement receipts")
        .as_u64();
    Ok(UiSettledQueryFactReceipt {
        key: UiSettledQueryFactKey {
            view_binding_id,
            binding_key: fact.binding_reference().key().clone(),
            settlement_reference: fact.settlement_reference().clone(),
        },
        binding_reference: fact.binding_reference().clone(),
        declaration_identity,
        declaration_support_authority_generation,
        required_measurement_dependencies: dependencies
            .required_measurement_dependencies()
            .to_vec()
            .into_boxed_slice(),
        required_query_fact_families: dependencies.fact_families().to_vec().into_boxed_slice(),
        required_query_fact_family_set_digest: query_measurement_fact_family_set_digest(
            dependencies.fact_families(),
        ),
        consumed_fact_family_set_digest: query_measurement_fact_family_set_digest(
            &consumed_fact_families,
        ),
        consumed_fact_families: consumed_fact_families.into_boxed_slice(),
        observations: observations.into_boxed_slice(),
        observation_identity_digest,
        source_generation,
        source_order,
        partial: fact.is_partial(),
        warning_count: fact.warning_count(),
    })
}
