use crate::validation::data::{
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule, InvariantVerdict,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ValidationResultIdentity {
    pub(crate) execution_point: InvariantExecutionPoint,
    pub(crate) failure_effect: InvariantFailureEffect,
    pub(crate) rule: InvariantRule,
    pub(crate) target_scope_identity: String,
}

impl ValidationResultIdentity {
    pub(crate) fn from_parts(
        execution_point: InvariantExecutionPoint,
        failure_effect: InvariantFailureEffect,
        rule: InvariantRule,
        verdict: &InvariantVerdict,
    ) -> Self {
        let target_scope_identity = match verdict {
            InvariantVerdict::Pass => "pass".to_string(),
            InvariantVerdict::Advisory { violation, .. }
            | InvariantVerdict::Violation(violation) => {
                format!("{:?}:{}", violation.code, violation.detail)
            }
        };
        Self {
            execution_point,
            failure_effect,
            rule,
            target_scope_identity,
        }
    }
}
