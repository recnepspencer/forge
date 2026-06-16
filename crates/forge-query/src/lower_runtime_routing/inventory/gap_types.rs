use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::ForgeQueryLowerRuntimeAuthorityOwner;
use super::ForgeQueryLowerRuntimeSeamKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeGapRegistryRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    concrete_seam: &'static str,
    current_shape: &'static str,
    missing_contract_owner: ForgeQueryLowerRuntimeAuthorityOwner,
    missing_contract: &'static str,
    required_closeout: &'static str,
}

impl ForgeQueryLowerRuntimeGapRegistryRow {
    #[allow(dead_code)]
    pub(crate) const fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        concrete_seam: &'static str,
        current_shape: &'static str,
        missing_contract_owner: ForgeQueryLowerRuntimeAuthorityOwner,
        missing_contract: &'static str,
        required_closeout: &'static str,
    ) -> Self {
        Self {
            seam_key,
            concrete_seam,
            current_shape,
            missing_contract_owner,
            missing_contract,
            required_closeout,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn row_digest(&self) -> String {
        self.row_identity().as_str().to_string()
    }

    fn row_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_gap_registry_row_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("seam"), self.seam_key.as_str())
            .field_value(
                ForgeQueryEvidenceTag::new("concrete_seam"),
                self.concrete_seam,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("current_shape"),
                self.current_shape,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("missing_contract_owner"),
                self.missing_contract_owner.as_str(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("missing_contract"),
                self.missing_contract,
            )
            .field_value(
                ForgeQueryEvidenceTag::new("required_closeout"),
                self.required_closeout,
            )
            .seal()
    }

    pub fn required_closeout(&self) -> &'static str {
        self.required_closeout
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeGapRegistry {
    rows: &'static [ForgeQueryLowerRuntimeGapRegistryRow],
}

impl ForgeQueryLowerRuntimeGapRegistry {
    pub(crate) const fn new(rows: &'static [ForgeQueryLowerRuntimeGapRegistryRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryLowerRuntimeGapRegistryRow] {
        self.rows
    }

    pub fn registry_digest(&self) -> String {
        let row_identities = self
            .rows
            .iter()
            .map(ForgeQueryLowerRuntimeGapRegistryRow::row_identity)
            .collect::<Vec<_>>();
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_gap_registry_v1",
            )
            .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
            .seal()
            .as_str()
            .to_string()
    }

    pub fn debt_exit_criteria_digest(&self) -> String {
        let closeout_identities = self
            .rows
            .iter()
            .map(|row| {
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("required_closeout"),
                    row.required_closeout(),
                )
                .seal()
            })
            .collect::<Vec<_>>();
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_gap_debt_exit_criteria_v1",
            )
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("rows"),
                &closeout_identities,
            )
            .seal()
            .as_str()
            .to_string()
    }
}
