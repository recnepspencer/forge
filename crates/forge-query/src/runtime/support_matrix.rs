use crate::identity::hash_parts;

use super::{
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimePublicApiContract,
};

const STABILIZED_EXTENSION_RULE: &str =
    "must-extend-stabilized-handle-state-lane-aspect-inspection-facade";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicSupportMatrixRow {
    surface: String,
    facade_family: Option<ForgeQueryRuntimeFacadeFamily>,
    status: ForgeQueryRuntimeFamilySupportStatus,
    owner_milestone: String,
    extension_rule: String,
    parallel_api_forbidden: bool,
    admission_fail_closed: bool,
    support_contract_digest: Option<String>,
    row_digest: String,
}

impl ForgeQueryRuntimePublicSupportMatrixRow {
    fn new(
        surface: impl Into<String>,
        facade_family: Option<ForgeQueryRuntimeFacadeFamily>,
        status: ForgeQueryRuntimeFamilySupportStatus,
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
        let mut parts = vec![
            format!("surface:{surface}"),
            format!(
                "family:{}",
                facade_family
                    .map(ForgeQueryRuntimeFacadeFamily::as_str)
                    .unwrap_or("matrix-only")
            ),
            format!("status:{}", status.as_str()),
            format!("owner:{owner_milestone}"),
            format!("extension:{extension_rule}"),
            format!("parallel_forbidden:{parallel_api_forbidden}"),
            format!("fail_closed:{admission_fail_closed}"),
        ];
        if let Some(digest) = &support_contract_digest {
            parts.push(format!("support:{digest}"));
        }
        let row_digest = hash_parts(&parts);
        Self {
            surface,
            facade_family,
            status,
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

    pub fn row_digest(&self) -> &str {
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
    matrix_digest: String,
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
                    owner_milestone_for_family(facade_family, status),
                    extension_rule_for_status(status),
                    true,
                    status != ForgeQueryRuntimeFamilySupportStatus::Supported,
                    Some(family.contract_digest().to_string()),
                )
            })
            .collect::<Vec<_>>();

        rows.push(ForgeQueryRuntimePublicSupportMatrixRow::new(
            "temporal-async-certification",
            None,
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt,
            "Milestone 9.7",
            STABILIZED_EXTENSION_RULE,
            true,
            true,
            None::<String>,
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
        let mut parts = vec![
            "forge_query_runtime_public_support_matrix_v1".to_string(),
            format!("posture:{}", contract.backend_posture().as_str()),
            format!("stable:{stable_row_count}"),
            format!("deferred:{deferred_row_count}"),
            format!("unsupported:{unsupported_row_count}"),
            format!("fail_closed:{fail_closed_row_count}"),
            format!("parallel_forbidden:{parallel_api_forbidden_row_count}"),
        ];
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        let matrix_digest = hash_parts(&parts);
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

    pub fn matrix_digest(&self) -> &str {
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

fn owner_milestone_for_family(
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
) -> &'static str {
    match (family, status) {
        (
            ForgeQueryRuntimeFacadeFamily::Read
            | ForgeQueryRuntimeFacadeFamily::Live
            | ForgeQueryRuntimeFacadeFamily::Computed
            | ForgeQueryRuntimeFacadeFamily::Effect
            | ForgeQueryRuntimeFacadeFamily::BranchPreview
            | ForgeQueryRuntimeFacadeFamily::Write
            | ForgeQueryRuntimeFacadeFamily::Inspect,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
        ) => "Milestone 9.3",
        (
            ForgeQueryRuntimeFacadeFamily::Intent,
            ForgeQueryRuntimeFamilySupportStatus::Unsupported,
        ) => "Milestone 9.x intent-authority-adapter",
        (ForgeQueryRuntimeFacadeFamily::Temporal, _) => "Milestone 9.4",
        (ForgeQueryRuntimeFacadeFamily::AsyncResource, _) => "Milestone 9.5",
        (ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery, _) => "Milestone 9.6",
        (ForgeQueryRuntimeFacadeFamily::StoreBackedExecution, _) => "Milestone 10",
        (ForgeQueryRuntimeFacadeFamily::DurableArtifacts, _) => "Milestone 11",
        _ => "current-runtime-support-profile",
    }
}

fn extension_rule_for_status(status: ForgeQueryRuntimeFamilySupportStatus) -> &'static str {
    match status {
        ForgeQueryRuntimeFamilySupportStatus::Supported => {
            "stable-runtime-backed-handle-state-lane-aspect-inspection-facade"
        }
        ForgeQueryRuntimeFamilySupportStatus::DeferredDebt => STABILIZED_EXTENSION_RULE,
        ForgeQueryRuntimeFamilySupportStatus::Unsupported => {
            "must-admit-through-runtime-support-profile-before-public-use"
        }
    }
}
