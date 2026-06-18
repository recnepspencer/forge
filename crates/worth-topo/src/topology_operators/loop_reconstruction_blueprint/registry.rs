use super::closeout::{
    PlanarBooleanLoopBlueprintCloseout, PlanarBooleanLoopBlueprintCloseoutDenial,
};
use super::operator_row::PlanarBooleanLoopOperatorRow;
use super::phase_2_inventory::{phase_2_operators, phase_2_validators};
use super::registry_identity::PlanarBooleanLoopBlueprintRegistryIdentity;
use super::required_phase_2_operator_lanes::REQUIRED_PHASE_2_OPERATOR_LANES;
use super::required_phase_2_validator_lanes::REQUIRED_PHASE_2_VALIDATOR_LANES;
use super::validator_row::PlanarBooleanLoopValidatorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopBlueprintRegistry {
    operators: Vec<PlanarBooleanLoopOperatorRow>,
    validators: Vec<PlanarBooleanLoopValidatorRow>,
    closeout: PlanarBooleanLoopBlueprintCloseout,
    identity: PlanarBooleanLoopBlueprintRegistryIdentity,
}

impl PlanarBooleanLoopBlueprintRegistry {
    pub fn phase_2() -> Self {
        Self::from_rows(phase_2_operators(), phase_2_validators())
            .expect("phase 2 loop blueprint registry must close out")
    }

    fn from_rows(
        operators: Vec<PlanarBooleanLoopOperatorRow>,
        validators: Vec<PlanarBooleanLoopValidatorRow>,
    ) -> Result<Self, PlanarBooleanLoopBlueprintCloseoutDenial> {
        let closeout = PlanarBooleanLoopBlueprintCloseout::certify(&operators, &validators)?;
        let identity =
            PlanarBooleanLoopBlueprintRegistryIdentity::derive(&operators, &validators, &closeout);
        Ok(Self {
            operators,
            validators,
            closeout,
            identity,
        })
    }

    #[cfg(test)]
    pub(crate) fn try_from_rows(
        operators: Vec<PlanarBooleanLoopOperatorRow>,
        validators: Vec<PlanarBooleanLoopValidatorRow>,
    ) -> Result<Self, PlanarBooleanLoopBlueprintCloseoutDenial> {
        Self::from_rows(operators, validators)
    }

    pub fn operators(&self) -> &[PlanarBooleanLoopOperatorRow] {
        &self.operators
    }

    pub fn validators(&self) -> &[PlanarBooleanLoopValidatorRow] {
        &self.validators
    }

    pub fn operator(&self, operator_name: &str) -> Option<&PlanarBooleanLoopOperatorRow> {
        self.operators
            .iter()
            .find(|operator| operator.operator_name() == operator_name)
    }

    pub fn validator(&self, validator_name: &str) -> Option<&PlanarBooleanLoopValidatorRow> {
        self.validators
            .iter()
            .find(|validator| validator.validator_name() == validator_name)
    }

    pub fn closeout(&self) -> &PlanarBooleanLoopBlueprintCloseout {
        &self.closeout
    }

    pub fn identity(&self) -> &PlanarBooleanLoopBlueprintRegistryIdentity {
        &self.identity
    }

    pub fn operator_classification_matrix(&self) -> PlanarBooleanLoopOperatorClassificationMatrix {
        PlanarBooleanLoopOperatorClassificationMatrix {
            operators: self.operators.clone(),
            registry_identity: self.identity.clone(),
        }
    }

    pub fn validator_registration_plan(&self) -> PlanarBooleanLoopValidatorRegistrationPlan {
        PlanarBooleanLoopValidatorRegistrationPlan {
            validators: self.validators.clone(),
            registry_identity: self.identity.clone(),
        }
    }

    pub fn required_operator_names(&self) -> impl Iterator<Item = &'static str> {
        REQUIRED_PHASE_2_OPERATOR_LANES
            .iter()
            .map(|(operator_name, _, _)| *operator_name)
    }

    pub fn required_validator_names(&self) -> impl Iterator<Item = &'static str> {
        REQUIRED_PHASE_2_VALIDATOR_LANES
            .iter()
            .map(|(validator_name, _, _)| *validator_name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopOperatorClassificationMatrix {
    operators: Vec<PlanarBooleanLoopOperatorRow>,
    registry_identity: PlanarBooleanLoopBlueprintRegistryIdentity,
}

impl PlanarBooleanLoopOperatorClassificationMatrix {
    pub fn phase_2() -> Self {
        PlanarBooleanLoopBlueprintRegistry::phase_2().operator_classification_matrix()
    }

    pub fn operators(&self) -> &[PlanarBooleanLoopOperatorRow] {
        &self.operators
    }

    pub fn operator(&self, operator_name: &str) -> Option<&PlanarBooleanLoopOperatorRow> {
        self.operators
            .iter()
            .find(|operator| operator.operator_name() == operator_name)
    }

    pub fn registry_identity(&self) -> &PlanarBooleanLoopBlueprintRegistryIdentity {
        &self.registry_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopValidatorRegistrationPlan {
    validators: Vec<PlanarBooleanLoopValidatorRow>,
    registry_identity: PlanarBooleanLoopBlueprintRegistryIdentity,
}

impl PlanarBooleanLoopValidatorRegistrationPlan {
    pub fn phase_2() -> Self {
        PlanarBooleanLoopBlueprintRegistry::phase_2().validator_registration_plan()
    }

    pub fn validators(&self) -> &[PlanarBooleanLoopValidatorRow] {
        &self.validators
    }

    pub fn validator(&self, validator_name: &str) -> Option<&PlanarBooleanLoopValidatorRow> {
        self.validators
            .iter()
            .find(|validator| validator.validator_name() == validator_name)
    }

    pub fn registry_identity(&self) -> &PlanarBooleanLoopBlueprintRegistryIdentity {
        &self.registry_identity
    }
}
