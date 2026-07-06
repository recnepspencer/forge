use super::closeout::{
    PlanarBooleanOverlapBlueprintCloseout, PlanarBooleanOverlapBlueprintCloseoutDenial,
};
use super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::phase_2_inventory::{phase_2_operators, phase_2_validators};
use super::registry_identity::PlanarBooleanOverlapBlueprintRegistryIdentity;
use super::validator_row::PlanarBooleanOverlapValidatorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapBlueprintRegistry {
    operators: Vec<PlanarBooleanOverlapOperatorRow>,
    validators: Vec<PlanarBooleanOverlapValidatorRow>,
    closeout: PlanarBooleanOverlapBlueprintCloseout,
    identity: PlanarBooleanOverlapBlueprintRegistryIdentity,
}

impl PlanarBooleanOverlapBlueprintRegistry {
    pub fn phase_2() -> Self {
        Self::from_rows(phase_2_operators(), phase_2_validators())
            .expect("phase 2 overlap blueprint registry must close out")
    }

    fn from_rows(
        operators: Vec<PlanarBooleanOverlapOperatorRow>,
        validators: Vec<PlanarBooleanOverlapValidatorRow>,
    ) -> Result<Self, PlanarBooleanOverlapBlueprintCloseoutDenial> {
        let closeout = PlanarBooleanOverlapBlueprintCloseout::certify(&operators, &validators)?;
        let identity =
            PlanarBooleanOverlapBlueprintRegistryIdentity::derive(&operators, &validators);
        Ok(Self {
            operators,
            validators,
            closeout,
            identity,
        })
    }

    #[cfg(test)]
    pub(crate) fn try_from_rows(
        operators: Vec<PlanarBooleanOverlapOperatorRow>,
        validators: Vec<PlanarBooleanOverlapValidatorRow>,
    ) -> Result<Self, PlanarBooleanOverlapBlueprintCloseoutDenial> {
        Self::from_rows(operators, validators)
    }

    pub fn operator_classification_matrix(
        &self,
    ) -> PlanarBooleanOverlapOperatorClassificationMatrix {
        PlanarBooleanOverlapOperatorClassificationMatrix {
            operators: self.operators.clone(),
            registry_identity: self.identity.clone(),
        }
    }

    pub fn validator_registration_plan(&self) -> PlanarBooleanOverlapValidatorRegistrationPlan {
        PlanarBooleanOverlapValidatorRegistrationPlan {
            validators: self.validators.clone(),
            registry_identity: self.identity.clone(),
        }
    }

    pub fn into_classification_matrix_and_validator_plan(
        self,
    ) -> (
        PlanarBooleanOverlapOperatorClassificationMatrix,
        PlanarBooleanOverlapValidatorRegistrationPlan,
    ) {
        let registry_identity = self.identity;
        (
            PlanarBooleanOverlapOperatorClassificationMatrix {
                operators: self.operators,
                registry_identity: registry_identity.clone(),
            },
            PlanarBooleanOverlapValidatorRegistrationPlan {
                validators: self.validators,
                registry_identity,
            },
        )
    }

    pub fn closeout(&self) -> &PlanarBooleanOverlapBlueprintCloseout {
        &self.closeout
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapOperatorClassificationMatrix {
    operators: Vec<PlanarBooleanOverlapOperatorRow>,
    registry_identity: PlanarBooleanOverlapBlueprintRegistryIdentity,
}

impl PlanarBooleanOverlapOperatorClassificationMatrix {
    pub fn phase_2() -> Self {
        PlanarBooleanOverlapBlueprintRegistry::phase_2().operator_classification_matrix()
    }

    pub fn operators(&self) -> &[PlanarBooleanOverlapOperatorRow] {
        &self.operators
    }

    pub fn operator(&self, operator_name: &str) -> Option<&PlanarBooleanOverlapOperatorRow> {
        self.operators
            .iter()
            .find(|operator| operator.operator_name() == operator_name)
    }

    pub fn registry_identity(&self) -> &PlanarBooleanOverlapBlueprintRegistryIdentity {
        &self.registry_identity
    }

    #[cfg(test)]
    pub(crate) fn without_operator_named(&self, operator_name: &str) -> Self {
        let mut operators = self.operators.clone();
        operators.retain(|operator| operator.operator_name() != operator_name);
        Self {
            operators,
            registry_identity: self.registry_identity.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapValidatorRegistrationPlan {
    validators: Vec<PlanarBooleanOverlapValidatorRow>,
    registry_identity: PlanarBooleanOverlapBlueprintRegistryIdentity,
}

impl PlanarBooleanOverlapValidatorRegistrationPlan {
    pub fn phase_2() -> Self {
        PlanarBooleanOverlapBlueprintRegistry::phase_2().validator_registration_plan()
    }

    pub fn validators(&self) -> &[PlanarBooleanOverlapValidatorRow] {
        &self.validators
    }

    pub fn validator(&self, validator_name: &str) -> Option<&PlanarBooleanOverlapValidatorRow> {
        self.validators
            .iter()
            .find(|validator| validator.validator_name() == validator_name)
    }

    pub fn registry_identity(&self) -> &PlanarBooleanOverlapBlueprintRegistryIdentity {
        &self.registry_identity
    }
}
