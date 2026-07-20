use worth_foundational::facade::AspectContract;

use crate::domain_operation::WorthQueryOperationEffectFamily;

use super::WorthQuerySemanticLocality;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalNodeOutput {
    DerivedAspect {
        contract: AspectContract,
        locality: WorthQuerySemanticLocality,
        consequences: Vec<WorthQueryConditionalConsequenceRole>,
    },
    OperationOutput {
        projection_role: crate::domain_operation::WorthQueryOperationProjectionRole,
    },
    WorkflowStageOutput {
        contract: crate::domain_operation::WorthQueryWorkflowValueContract,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryConditionalTouchRole {
    graph_role: String,
    scope: String,
}

impl WorthQueryConditionalTouchRole {
    pub fn new(
        graph_role: impl Into<String>,
        scope: impl Into<String>,
    ) -> Result<Self, &'static str> {
        let graph_role = graph_role.into();
        let scope = scope.into();
        if graph_role.trim().is_empty() || scope.trim().is_empty() {
            return Err("empty-conditional-touch-role");
        }
        Ok(Self { graph_role, scope })
    }

    pub fn graph_role(&self) -> &str {
        &self.graph_role
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryConditionalConsequenceRole {
    DerivedOnly,
    Touch(WorthQueryConditionalTouchRole),
    Effect(WorthQueryOperationEffectFamily),
}

pub(crate) fn canonicalize_output(output: &mut WorthQueryConditionalNodeOutput) {
    if let WorthQueryConditionalNodeOutput::DerivedAspect { consequences, .. } = output {
        consequences.sort();
        consequences.dedup();
    }
}

pub(crate) fn output_token(output: &WorthQueryConditionalNodeOutput) -> String {
    match output {
        WorthQueryConditionalNodeOutput::DerivedAspect {
            contract,
            locality,
            consequences,
        } => {
            let mut material = String::new();
            super::push_token(&mut material, "kind", "derived-aspect");
            super::push_token(&mut material, "contract", &super::contract_token(contract));
            super::push_token(&mut material, "locality", &super::locality_token(locality));
            for consequence in consequences {
                super::push_token(
                    &mut material,
                    "consequence",
                    &consequence_token(consequence),
                );
            }
            material
        }
        WorthQueryConditionalNodeOutput::OperationOutput { projection_role } => {
            format!("operation-output:{}", projection_role.as_str())
        }
        WorthQueryConditionalNodeOutput::WorkflowStageOutput { contract } => {
            format!(
                "workflow-stage-output:{}",
                workflow_value_contract_name(*contract)
            )
        }
    }
}

fn workflow_value_contract_name(
    contract: crate::domain_operation::WorthQueryWorkflowValueContract,
) -> &'static str {
    use crate::domain_operation::WorthQueryWorkflowValueContract as Contract;
    match contract {
        Contract::NotRequired => "not-required",
        Contract::Bool => "bool",
        Contract::I64 => "i64",
        Contract::U64 => "u64",
        Contract::Text => "text",
        Contract::EntityIdentity => "entity-identity",
        Contract::Projection => "projection",
    }
}

fn consequence_token(consequence: &WorthQueryConditionalConsequenceRole) -> String {
    match consequence {
        WorthQueryConditionalConsequenceRole::DerivedOnly => "derived-only".to_string(),
        WorthQueryConditionalConsequenceRole::Touch(touch) => {
            format!("touch:{}:{}", touch.graph_role(), touch.scope())
        }
        WorthQueryConditionalConsequenceRole::Effect(family) => {
            format!("effect:{}", family.as_str())
        }
    }
}
