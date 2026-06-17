use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryRuntime, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeFamilyTeachingPosture,
    ForgeQueryRuntimePublicApiContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicSupportMatrixRow {
    surface: String,
    facade_family: Option<ForgeQueryRuntimeFacadeFamily>,
    status: ForgeQueryRuntimeFamilySupportStatus,
    teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture,
    owner_milestone: String,
    extension_rule: String,
    parallel_api_forbidden: bool,
    admission_fail_closed: bool,
    support_contract_digest: Option<String>,
    row_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimePublicSupportMatrixRow {
    fn new(
        surface: impl Into<String>,
        facade_family: Option<ForgeQueryRuntimeFacadeFamily>,
        status: ForgeQueryRuntimeFamilySupportStatus,
        teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture,
        owner_milestone: impl Into<String>,
        extension_rule: impl Into<String>,
        parallel_api_forbidden: bool,
        admission_fail_closed: bool,
        support_contract_digest: Option<impl Into<String>>,
    ) -> Self {
        let surface = surface.into();
        let owner_milestone = owner_milestone.into();
        let extension_rule = extension_rule.into();
        let support_contract_digest = support_contract_digest.map(Into::into);
        let row_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicSupportMatrixRow)
                .field_shape(ForgeQueryEvidenceTag::new("surface"), surface.clone())
                .field_shape(
                    ForgeQueryEvidenceTag::new("facade_family"),
                    facade_family
                        .map(ForgeQueryRuntimeFacadeFamily::as_str)
                        .unwrap_or("matrix-only"),
                )
                .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("teaching_posture"),
                    teaching_posture.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("owner_milestone"),
                    owner_milestone.clone(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("extension_rule"),
                    extension_rule.clone(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("parallel_api_forbidden"),
                    parallel_api_forbidden,
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("admission_fail_closed"),
                    admission_fail_closed,
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("support_contract_digest"),
                    support_contract_digest.as_deref(),
                )
                .seal();
        Self {
            surface,
            facade_family,
            status,
            teaching_posture,
            owner_milestone,
            extension_rule,
            parallel_api_forbidden,
            admission_fail_closed,
            support_contract_digest,
            row_digest,
        }
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn facade_family(&self) -> Option<ForgeQueryRuntimeFacadeFamily> {
        self.facade_family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> ForgeQueryRuntimeFamilyTeachingPosture {
        self.teaching_posture
    }

    pub fn ordinary_downstream_dx(&self) -> bool {
        self.teaching_posture == ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx
    }

    pub fn owner_milestone(&self) -> &str {
        &self.owner_milestone
    }

    pub fn extension_rule(&self) -> &str {
        &self.extension_rule
    }

    pub fn parallel_api_forbidden(&self) -> bool {
        self.parallel_api_forbidden
    }

    pub fn admission_fail_closed(&self) -> bool {
        self.admission_fail_closed
    }

    pub fn support_contract_digest(&self) -> Option<&str> {
        self.support_contract_digest.as_deref()
    }

    pub fn row_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicSupportMatrix {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    rows: Vec<ForgeQueryRuntimePublicSupportMatrixRow>,
    stable_row_count: usize,
    deferred_row_count: usize,
    unsupported_row_count: usize,
    fail_closed_row_count: usize,
    parallel_api_forbidden_row_count: usize,
    matrix_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimePublicSupportMatrix {
    pub fn from_public_api_contract(contract: &ForgeQueryRuntimePublicApiContract) -> Self {
        let mut rows = contract
            .families()
            .iter()
            .map(|family| {
                let status = family.status();
                let facade_family = family.family();
                ForgeQueryRuntimePublicSupportMatrixRow::new(
                    facade_family.as_str(),
                    Some(facade_family),
                    status,
                    family.teaching_posture(),
                    family.owner_closure(),
                    family.extension_rule(),
                    family.parallel_api_forbidden(),
                    family.admission_fail_closed(),
                    Some(family.contract_digest().to_string()),
                )
            })
            .collect::<Vec<_>>();

        rows.push(ForgeQueryRuntimePublicSupportMatrixRow::new(
            "authoritative-mutation-evidence-certification",
            None,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
            "Runtime Authoritative Mutation Evidence Gate",
            "must-extend-target-binding-naming-continuity-causality-provenance-contract",
            true,
            false,
            Some(
                ForgeQueryRuntime::public_authoritative_mutation_evidence_support_for_posture(
                    contract.backend_posture(),
                )
                .support_digest()
                .to_string(),
            ),
        ));

        rows.push(ForgeQueryRuntimePublicSupportMatrixRow::new(
            "temporal-async-certification",
            None,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
            "Milestone 9.4",
            "must-extend-stabilized-handle-state-lane-aspect-inspection-facade",
            true,
            false,
            None::<String>,
        ));

        rows.push(ForgeQueryRuntimePublicSupportMatrixRow::new(
            "temporal-async-remask",
            None,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
            "Milestone 9.4",
            "must-remask-before-runtime-delivery-state-and-inspection-projection",
            true,
            true,
            None::<String>,
        ));

        rows.push(ForgeQueryRuntimePublicSupportMatrixRow::new(
            "downstream-delivery-contract",
            None,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
            "Milestone 9.4",
            "must-project-runtime-backed-delivery-basis-resume-and-support-posture-through-one-query-owned-contract",
            true,
            false,
            Some(
                ForgeQueryRuntimeDownstreamDeliveryContract::from_backend_posture(
                    contract.backend_posture(),
                )
                .contract_for_reporting()
                .to_string(),
            ),
        ));

        let stable_row_count = rows
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::Supported)
            .count();
        let deferred_row_count = rows
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::DeferredDebt)
            .count();
        let unsupported_row_count = rows
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported)
            .count();
        let fail_closed_row_count = rows
            .iter()
            .filter(|row| row.admission_fail_closed())
            .count();
        let parallel_api_forbidden_row_count = rows
            .iter()
            .filter(|row| row.parallel_api_forbidden())
            .count();
        let matrix_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicSupportMatrix)
                .field_shape(
                    ForgeQueryEvidenceTag::new("backend_posture"),
                    contract.backend_posture().as_str(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("stable_row_count"),
                    stable_row_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("deferred_row_count"),
                    deferred_row_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("unsupported_row_count"),
                    unsupported_row_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("fail_closed_row_count"),
                    fail_closed_row_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("parallel_api_forbidden_row_count"),
                    parallel_api_forbidden_row_count,
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("row_digest"),
                    rows.iter().map(|row| row.row_digest().as_str()),
                )
                .seal();
        Self {
            backend_posture: contract.backend_posture(),
            rows,
            stable_row_count,
            deferred_row_count,
            unsupported_row_count,
            fail_closed_row_count,
            parallel_api_forbidden_row_count,
            matrix_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn rows(&self) -> &[ForgeQueryRuntimePublicSupportMatrixRow] {
        &self.rows
    }

    pub fn stable_row_count(&self) -> usize {
        self.stable_row_count
    }

    pub fn deferred_row_count(&self) -> usize {
        self.deferred_row_count
    }

    pub fn unsupported_row_count(&self) -> usize {
        self.unsupported_row_count
    }

    pub fn fail_closed_row_count(&self) -> usize {
        self.fail_closed_row_count
    }

    pub fn parallel_api_forbidden_row_count(&self) -> usize {
        self.parallel_api_forbidden_row_count
    }

    pub fn matrix_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.matrix_digest
    }

    pub fn row(&self, surface: &str) -> Option<&ForgeQueryRuntimePublicSupportMatrixRow> {
        self.rows.iter().find(|row| row.surface() == surface)
    }

    pub fn row_for_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Option<&ForgeQueryRuntimePublicSupportMatrixRow> {
        self.rows
            .iter()
            .find(|row| row.facade_family() == Some(family))
    }
}
