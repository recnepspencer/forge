use crate::identity::hash_parts;

use super::eligibility::{AdmittedProjectionConsumption, ProjectionConsumptionWarningKind};
use super::facts::ProjectionMaterializedFactPosture;
use super::facts::{ProjectionFactKind, ProjectionFactRequest};
use super::source::{
    ProjectionConsumptionSource, ProjectionSourceFamily, ProjectionSourceReferenceIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionContractSourcePosture {
    QueryOwnedReceiptSource,
    RelationalAuthoritySource,
    BridgeAuthoritySource,
}

impl ProjectionContractSourcePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryOwnedReceiptSource => "query_owned_receipt_source",
            Self::RelationalAuthoritySource => "relational_authority_source",
            Self::BridgeAuthoritySource => "bridge_authority_source",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionContractSupportPosture {
    Admitted,
    AdmittedWithWarnings(Vec<ProjectionConsumptionWarningKind>),
}

impl ProjectionContractSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::AdmittedWithWarnings(_) => "admitted_with_warnings",
        }
    }

    pub fn warning_kinds(&self) -> &[ProjectionConsumptionWarningKind] {
        match self {
            Self::Admitted => &[],
            Self::AdmittedWithWarnings(warnings) => warnings,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundProjectionFactFamily {
    request: ProjectionFactRequest,
    support_posture: ProjectionContractSupportPosture,
}

impl BoundProjectionFactFamily {
    pub fn request(&self) -> &ProjectionFactRequest {
        &self.request
    }

    pub fn kind(&self) -> ProjectionFactKind {
        self.request.kind()
    }

    pub fn field_key(&self) -> Option<&str> {
        self.request.field_key()
    }

    pub fn support_posture(&self) -> &ProjectionContractSupportPosture {
        &self.support_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterializedProjectionContract {
    declaration_digest: String,
    eligibility_digest: String,
    query_digest: Option<String>,
    basis_digest: Option<String>,
    result_digest: Option<String>,
    canonical_result_shape_digest: String,
    narrowed_result_shape_digest: String,
    authorized_projection_identity: String,
    policy_digest: String,
    tenant_schema_basis_digest: String,
    source_family: ProjectionSourceFamily,
    source_posture: ProjectionContractSourcePosture,
    source_identity: String,
    source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    fact_families: Vec<BoundProjectionFactFamily>,
    support_posture: ProjectionContractSupportPosture,
    contract_digest: String,
}

impl MaterializedProjectionContract {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn query_digest(&self) -> Option<&str> {
        self.query_digest.as_deref()
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn result_digest(&self) -> Option<&str> {
        self.result_digest.as_deref()
    }

    pub fn canonical_result_shape_digest(&self) -> &str {
        &self.canonical_result_shape_digest
    }

    pub fn narrowed_result_shape_digest(&self) -> &str {
        &self.narrowed_result_shape_digest
    }

    pub fn authorized_projection_identity(&self) -> &str {
        &self.authorized_projection_identity
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn source_posture(&self) -> ProjectionContractSourcePosture {
        self.source_posture
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn source_reference_identities(&self) -> &[ProjectionSourceReferenceIdentity] {
        &self.source_reference_identities
    }

    pub fn fact_families(&self) -> &[BoundProjectionFactFamily] {
        &self.fact_families
    }

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
    }

    pub fn support_posture(&self) -> &ProjectionContractSupportPosture {
        &self.support_posture
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

pub(crate) fn bind_materialized_projection_contract(
    admitted: &AdmittedProjectionConsumption,
) -> MaterializedProjectionContract {
    let declaration = admitted.declaration();
    let source = declaration.source();
    let support_posture = if admitted.warning_kinds().is_empty() {
        ProjectionContractSupportPosture::Admitted
    } else {
        ProjectionContractSupportPosture::AdmittedWithWarnings(admitted.warning_kinds().to_vec())
    };
    let fact_families = declaration
        .requested()
        .requested()
        .cloned()
        .map(|request| BoundProjectionFactFamily {
            request,
            support_posture: support_posture.clone(),
        })
        .collect::<Vec<_>>();
    let contract_digest = hash_parts(&contract_digest_parts(
        admitted,
        source,
        declaration.binding(),
        &fact_families,
        &support_posture,
    ));
    MaterializedProjectionContract {
        declaration_digest: declaration.declaration_digest().to_string(),
        eligibility_digest: admitted.eligibility_digest().to_string(),
        query_digest: source.query_digest().map(str::to_string),
        basis_digest: source.basis_digest().map(str::to_string),
        result_digest: source.result_digest().map(str::to_string),
        canonical_result_shape_digest: declaration.binding().result_shape_digest().to_string(),
        narrowed_result_shape_digest: declaration
            .binding()
            .narrowed_result_shape_digest()
            .to_string(),
        authorized_projection_identity: declaration
            .binding()
            .authorized_projection_identity()
            .to_string(),
        policy_digest: declaration.binding().policy_digest().to_string(),
        tenant_schema_basis_digest: declaration
            .binding()
            .tenant_schema_basis_digest()
            .to_string(),
        source_family: source.family(),
        source_posture: contract_source_posture(source.family()),
        source_identity: source.source_identity().to_string(),
        source_reference_identities: source.source_reference_identities().to_vec(),
        materialized_fact_posture: source.materialized_fact_posture().cloned(),
        fact_families,
        support_posture,
        contract_digest,
    }
}

fn contract_digest_parts(
    admitted: &AdmittedProjectionConsumption,
    source: &ProjectionConsumptionSource,
    binding: &super::declaration::ProjectionConsumptionBindingContext,
    fact_families: &[BoundProjectionFactFamily],
    support_posture: &ProjectionContractSupportPosture,
) -> Vec<String> {
    let mut parts = vec![
        "materialized_projection_contract_v1".to_string(),
        format!(
            "declaration:{}",
            admitted.declaration().declaration_digest()
        ),
        format!("eligibility:{}", admitted.eligibility_digest()),
        format!("source_family:{}", source.family().as_str()),
        format!(
            "source_posture:{}",
            contract_source_posture(source.family()).as_str()
        ),
        format!("source_identity:{}", source.source_identity()),
        format!("result_shape:{}", binding.result_shape_digest()),
        format!(
            "narrowed_result_shape:{}",
            binding.narrowed_result_shape_digest()
        ),
        format!(
            "authorized_projection:{}",
            binding.authorized_projection_identity()
        ),
        format!("policy:{}", binding.policy_digest()),
        format!("tenant_schema:{}", binding.tenant_schema_basis_digest()),
        format!("support_posture:{}", support_posture.as_str()),
    ];
    if let Some(query_digest) = source.query_digest() {
        parts.push(format!("query:{query_digest}"));
    }
    if let Some(basis_digest) = source.basis_digest() {
        parts.push(format!("basis:{basis_digest}"));
    }
    if let Some(result_digest) = source.result_digest() {
        parts.push(format!("result:{result_digest}"));
    }
    if let Some(posture) = source.materialized_fact_posture() {
        parts.push(format!(
            "materialized_fact_posture:{}",
            posture.posture_digest()
        ));
    }
    if !support_posture.warning_kinds().is_empty() {
        parts.push(format!(
            "warnings:{}",
            hash_parts(
                &support_posture
                    .warning_kinds()
                    .iter()
                    .map(|warning| warning.as_str().to_string())
                    .collect::<Vec<_>>()
            )
        ));
    }
    parts.extend(source.source_reference_identities().iter().map(|identity| {
        format!(
            "source_reference:{}:{}",
            identity.label(),
            identity.identity()
        )
    }));
    parts.extend(fact_families.iter().map(fact_family_digest_part));
    parts
}

fn fact_family_digest_part(fact_family: &BoundProjectionFactFamily) -> String {
    match fact_family.field_key() {
        Some(field_key) => format!(
            "fact:{}:{}:{}",
            fact_family.kind().as_str(),
            field_key,
            fact_family.support_posture().as_str()
        ),
        None => format!(
            "fact:{}:{}",
            fact_family.kind().as_str(),
            fact_family.support_posture().as_str()
        ),
    }
}

fn contract_source_posture(family: ProjectionSourceFamily) -> ProjectionContractSourcePosture {
    match family {
        ProjectionSourceFamily::QueryReadReceipt
        | ProjectionSourceFamily::QueryWriteReceipt
        | ProjectionSourceFamily::QueryContextExecution => {
            ProjectionContractSourcePosture::QueryOwnedReceiptSource
        }
        ProjectionSourceFamily::RelationalRowSet
        | ProjectionSourceFamily::RelationalGroupedProjection => {
            ProjectionContractSourcePosture::RelationalAuthoritySource
        }
        ProjectionSourceFamily::BridgeTruthViewRowSet
        | ProjectionSourceFamily::BridgeGroupedTruthView => {
            ProjectionContractSourcePosture::BridgeAuthoritySource
        }
    }
}
