use forge_query::facade::BasisResolutionMode;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;
use worth_ui_query_binding::{
    WorthUiQueryBindingSubsystem, WorthUiQueryMeasurementFactFamily,
    WorthUiQueryMeasurementFactObservation, WorthUiQueryMeasurementFactReceipt,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryPrerequisiteEvidence,
};

use crate::declaration::{
    declared_query_measurement_dependencies, stable_text_digest, UiDeclarationIdentity,
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementPolicyPosture,
    UiDeclaredMeasurementQueryDependencySet,
};

use super::query_measurement_fact_family_set_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiProjectionFactReceiptDenial {
    NoQueryMeasurementDependencies,
    QueryFactReceipt(WorthUiQueryMeasurementFactReceiptError),
    MissingRequiredFactFamilies {
        required: Box<[WorthUiQueryMeasurementFactFamily]>,
        consumed: Box<[WorthUiQueryMeasurementFactFamily]>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiProjectionFactObservation {
    family: WorthUiQueryMeasurementFactFamily,
    extent_bits: u32,
    identity_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionFactReceipt {
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    query_basis_digest: Box<str>,
    query_resolution_mode: BasisResolutionMode,
    projection_contract_digest: Box<str>,
    projection_consumption_declaration_digest: Box<str>,
    projection_consumption_receipt_digest: Box<str>,
    projection_fact_set_digest: Box<str>,
    projection_source_identity: Box<str>,
    required_measurement_dependencies: Box<[UiDeclaredMeasurementEvidenceRequirement]>,
    required_query_fact_families: Box<[WorthUiQueryMeasurementFactFamily]>,
    required_query_fact_family_set_digest: u64,
    consumed_fact_families: Box<[WorthUiQueryMeasurementFactFamily]>,
    consumed_fact_family_set_digest: u64,
    observations: Box<[UiProjectionFactObservation]>,
    observation_identity_digest: u64,
}

impl UiProjectionFactObservation {
    fn from_query_observation(observation: WorthUiQueryMeasurementFactObservation) -> Self {
        let identity_digest = stable_text_digest("worth-ui.projection-fact-observation")
            ^ stable_text_digest(query_measurement_family_name(observation.family()))
                .rotate_left(7)
            ^ (observation.extent().to_bits() as u64).rotate_left(13);
        Self {
            family: observation.family(),
            extent_bits: observation.extent().to_bits(),
            identity_digest,
        }
    }

    pub fn family(&self) -> WorthUiQueryMeasurementFactFamily {
        self.family
    }

    pub fn extent(&self) -> f32 {
        f32::from_bits(self.extent_bits)
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

impl UiProjectionFactReceipt {
    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn declaration_support_authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.declaration_support_authority_generation
    }

    pub fn query_basis_digest(&self) -> &str {
        &self.query_basis_digest
    }

    pub fn query_resolution_mode(&self) -> &BasisResolutionMode {
        &self.query_resolution_mode
    }

    pub fn projection_contract_digest(&self) -> &str {
        &self.projection_contract_digest
    }

    pub fn projection_consumption_declaration_digest(&self) -> &str {
        &self.projection_consumption_declaration_digest
    }

    pub fn projection_consumption_receipt_digest(&self) -> &str {
        &self.projection_consumption_receipt_digest
    }

    pub fn projection_fact_set_digest(&self) -> &str {
        &self.projection_fact_set_digest
    }

    pub fn projection_source_identity(&self) -> &str {
        &self.projection_source_identity
    }

    pub fn required_measurement_dependencies(&self) -> &[UiDeclaredMeasurementEvidenceRequirement] {
        &self.required_measurement_dependencies
    }

    pub fn required_query_fact_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.required_query_fact_families
    }

    pub fn required_query_fact_family_set_digest(&self) -> u64 {
        self.required_query_fact_family_set_digest
    }

    pub fn consumed_fact_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
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
}

pub fn consume_declared_measurement_projection_facts(
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    measurement_policy: &UiDeclaredMeasurementPolicyPosture,
    query_prerequisites: WorthUiQueryPrerequisiteEvidence,
    consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
) -> Result<UiProjectionFactReceipt, UiProjectionFactReceiptDenial> {
    let dependencies = declared_query_measurement_dependencies(measurement_policy)
        .ok_or(UiProjectionFactReceiptDenial::NoQueryMeasurementDependencies)?;
    let query_receipt = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_projection_consumption(query_prerequisites, consumption)
        .map_err(UiProjectionFactReceiptDenial::QueryFactReceipt)?;
    admit_declared_measurement_projection_fact_receipt(
        declaration_identity,
        declaration_support_authority_generation,
        dependencies,
        query_receipt,
    )
}

pub(crate) fn admit_declared_measurement_projection_fact_receipt(
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    dependencies: UiDeclaredMeasurementQueryDependencySet,
    query_receipt: WorthUiQueryMeasurementFactReceipt,
) -> Result<UiProjectionFactReceipt, UiProjectionFactReceiptDenial> {
    validate_consumed_query_fact_families(&dependencies, &query_receipt)?;
    Ok(receipt_from_query_fact_receipt(
        declaration_identity,
        declaration_support_authority_generation,
        dependencies,
        query_receipt,
    ))
}

fn validate_consumed_query_fact_families(
    dependencies: &UiDeclaredMeasurementQueryDependencySet,
    query_receipt: &WorthUiQueryMeasurementFactReceipt,
) -> Result<(), UiProjectionFactReceiptDenial> {
    let missing = dependencies
        .fact_families()
        .iter()
        .copied()
        .filter(|family| !query_receipt.consumed_families().contains(family))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return Ok(());
    }

    Err(UiProjectionFactReceiptDenial::MissingRequiredFactFamilies {
        required: dependencies.fact_families().to_vec().into_boxed_slice(),
        consumed: query_receipt
            .consumed_families()
            .to_vec()
            .into_boxed_slice(),
    })
}

fn receipt_from_query_fact_receipt(
    declaration_identity: UiDeclarationIdentity,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    dependencies: UiDeclaredMeasurementQueryDependencySet,
    query_receipt: WorthUiQueryMeasurementFactReceipt,
) -> UiProjectionFactReceipt {
    let required_query_fact_family_set_digest =
        query_measurement_fact_family_set_digest(dependencies.fact_families());
    let consumed_fact_family_set_digest =
        query_measurement_fact_family_set_digest(query_receipt.consumed_families());
    let observations = query_receipt
        .observations()
        .iter()
        .copied()
        .map(UiProjectionFactObservation::from_query_observation)
        .collect::<Vec<_>>();
    let observation_identity_digest = observations.iter().fold(
        stable_text_digest("worth-ui.projection-fact-observations"),
        |digest, observation| digest ^ observation.identity_digest().rotate_left(17),
    );
    UiProjectionFactReceipt {
        declaration_identity,
        declaration_support_authority_generation,
        query_basis_digest: query_receipt
            .prerequisites()
            .resolution_report()
            .basis_digest()
            .as_str()
            .into(),
        query_resolution_mode: query_receipt
            .prerequisites()
            .resolution_report()
            .resolution_mode()
            .clone(),
        projection_contract_digest: query_receipt.projection_contract_digest().into(),
        projection_consumption_declaration_digest: query_receipt
            .projection_consumption_declaration_digest()
            .into(),
        projection_consumption_receipt_digest: query_receipt
            .projection_consumption_receipt_digest()
            .into(),
        projection_fact_set_digest: query_receipt.projection_fact_set_digest().into(),
        projection_source_identity: query_receipt.projection_source_identity().into(),
        required_measurement_dependencies: dependencies
            .required_measurement_dependencies()
            .to_vec()
            .into_boxed_slice(),
        required_query_fact_families: dependencies.fact_families().to_vec().into_boxed_slice(),
        required_query_fact_family_set_digest,
        consumed_fact_families: query_receipt
            .consumed_families()
            .to_vec()
            .into_boxed_slice(),
        consumed_fact_family_set_digest,
        observations: observations.into_boxed_slice(),
        observation_identity_digest,
    }
}

fn query_measurement_family_name(family: WorthUiQueryMeasurementFactFamily) -> &'static str {
    match family {
        WorthUiQueryMeasurementFactFamily::ScrollContentExtent => "scroll-content-extent",
    }
}
