use super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::validator_row::PlanarBooleanOverlapValidatorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapBlueprintRegistryIdentity {
    digest: String,
}

impl PlanarBooleanOverlapBlueprintRegistryIdentity {
    pub(crate) fn derive(
        operators: &[PlanarBooleanOverlapOperatorRow],
        validators: &[PlanarBooleanOverlapValidatorRow],
    ) -> Self {
        let operator_names = operators
            .iter()
            .map(|row| row.operator_name())
            .collect::<Vec<_>>()
            .join("|");
        let validator_names = validators
            .iter()
            .map(|row| row.validator_name())
            .collect::<Vec<_>>()
            .join("|");
        Self {
            digest: format!(
                "planar-boolean-overlap-blueprint:{}:{}",
                operator_names, validator_names
            ),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
