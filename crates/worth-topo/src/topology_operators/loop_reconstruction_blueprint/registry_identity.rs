use super::closeout::PlanarBooleanLoopBlueprintCloseout;
use super::operator_row::PlanarBooleanLoopOperatorRow;
use super::validator_row::PlanarBooleanLoopValidatorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopBlueprintRegistryIdentity {
    digest: String,
}

impl PlanarBooleanLoopBlueprintRegistryIdentity {
    pub(super) fn derive(
        operators: &[PlanarBooleanLoopOperatorRow],
        validators: &[PlanarBooleanLoopValidatorRow],
        closeout: &PlanarBooleanLoopBlueprintCloseout,
    ) -> Self {
        let mut digest = String::from("loop-blueprint-phase-2");
        digest.push_str("|operators:");
        append_operator_digest(&mut digest, operators);
        digest.push_str("|validators:");
        append_validator_digest(&mut digest, validators);
        digest.push_str("|counts:");
        digest.push_str(&closeout.required_phase_2_operator_rows().to_string());
        digest.push(':');
        digest.push_str(&closeout.required_phase_2_validator_rows().to_string());
        digest.push(':');
        digest.push_str(&closeout.support_gated_future_operators().to_string());
        Self { digest }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

fn append_operator_digest(digest: &mut String, operators: &[PlanarBooleanLoopOperatorRow]) {
    for operator in operators {
        digest.push('[');
        digest.push_str(operator.operator_name());
        digest.push('|');
        digest.push_str(&format!("{:?}", operator.classification()));
        digest.push('|');
        digest.push_str(&format!("{:?}", operator.required_query_surface()));
        digest.push(']');
    }
}

fn append_validator_digest(digest: &mut String, validators: &[PlanarBooleanLoopValidatorRow]) {
    for validator in validators {
        digest.push('[');
        digest.push_str(validator.validator_name());
        digest.push('|');
        digest.push_str(&format!("{:?}", validator.runtime_lane()));
        digest.push('|');
        digest.push_str(if validator.governs_topology_legality() {
            "topology"
        } else {
            "prepared"
        });
        digest.push(']');
    }
}
