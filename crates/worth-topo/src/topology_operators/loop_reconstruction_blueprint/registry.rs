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

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn without_operator_named(&self, operator_name: &str) -> Self {
        let mut operators = self.operators.clone();
        operators.retain(|operator| operator.operator_name() != operator_name);
        Self {
            operators,
            registry_identity: self.registry_identity.clone(),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn with_operator_classification(
        &self,
        operator_name: &str,
        classification: super::classification::PlanarBooleanLoopOperatorClassification,
    ) -> Self {
        let operators = self
            .operators
            .iter()
            .cloned()
            .map(|operator| {
                if operator.operator_name() == operator_name {
                    PlanarBooleanLoopOperatorRow::new(
                        operator.operator_name(),
                        classification,
                        operator.truth_authority(),
                        operator.required_query_surface(),
                        operator.topology_precedent(),
                        operator.proof_obligations(),
                        operator.support_warning(),
                    )
                } else {
                    operator
                }
            })
            .collect();
        Self {
            operators,
            registry_identity: self.registry_identity.clone(),
        }
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

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn without_validator_named(&self, validator_name: &str) -> Self {
        let mut validators = self.validators.clone();
        validators.retain(|validator| validator.validator_name() != validator_name);
        Self {
            validators,
            registry_identity: self.registry_identity.clone(),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn with_validator_runtime_lane(
        &self,
        validator_name: &str,
        runtime_lane: super::classification::PlanarBooleanLoopValidatorRuntimeLane,
    ) -> Self {
        let validators = self
            .validators
            .iter()
            .cloned()
            .map(|validator| {
                if validator.validator_name() == validator_name {
                    PlanarBooleanLoopValidatorRow::new(
                        validator.validator_name(),
                        runtime_lane,
                        validator.governs_topology_legality(),
                        validator.proof_obligations(),
                    )
                } else {
                    validator
                }
            })
            .collect();
        Self {
            validators,
            registry_identity: self.registry_identity.clone(),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn with_validator_topology_legality(
        &self,
        validator_name: &str,
        governs_topology_legality: bool,
    ) -> Self {
        let validators = self
            .validators
            .iter()
            .cloned()
            .map(|validator| {
                if validator.validator_name() == validator_name {
                    PlanarBooleanLoopValidatorRow::new(
                        validator.validator_name(),
                        validator.runtime_lane(),
                        governs_topology_legality,
                        validator.proof_obligations(),
                    )
                } else {
                    validator
                }
            })
            .collect();
        Self {
            validators,
            registry_identity: self.registry_identity.clone(),
        }
    }
}
