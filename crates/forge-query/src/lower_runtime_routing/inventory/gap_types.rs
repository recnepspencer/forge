use crate::identity::hash_parts;

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
        hash_parts(&[
            "lower_runtime_gap_registry_row_v1".to_string(),
            format!("seam:{}", self.seam_key.as_str()),
            format!("concrete_seam:{}", self.concrete_seam),
            format!("current_shape:{}", self.current_shape),
            format!(
                "missing_contract_owner:{}",
                self.missing_contract_owner.as_str()
            ),
            format!("missing_contract:{}", self.missing_contract),
            format!("required_closeout:{}", self.required_closeout),
        ])
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
        hash_parts(
            &self
                .rows
                .iter()
                .map(ForgeQueryLowerRuntimeGapRegistryRow::row_digest)
                .collect::<Vec<_>>(),
        )
    }

    pub fn debt_exit_criteria_digest(&self) -> String {
        hash_parts(
            &self
                .rows
                .iter()
                .map(|row| row.required_closeout().to_string())
                .collect::<Vec<_>>(),
        )
    }
}
