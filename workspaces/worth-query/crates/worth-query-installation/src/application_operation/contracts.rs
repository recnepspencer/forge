use worth_query_declaration::facade::application_schema::{
    ApplicationAuthorizationPath, ApplicationOperationProgramTarget,
};

use crate::domain_operation::{
    WorthQueryOperationEffectContract, WorthQueryOperationEffectFamily,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationGraphReadContract, WorthQueryOperationGraphReadRole,
    WorthQueryOperationInvariantContract, WorthQueryOperationTouchContract,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryInstalledAbilityRequirement {
    ability: String,
    scope_entity: String,
    policy: String,
    policy_paths: Vec<ApplicationAuthorizationPath>,
}

impl WorthQueryInstalledAbilityRequirement {
    pub(crate) fn new(
        ability: String,
        scope_entity: String,
        policy: String,
        policy_paths: Vec<ApplicationAuthorizationPath>,
    ) -> Self {
        Self {
            ability,
            scope_entity,
            policy,
            policy_paths,
        }
    }

    pub fn ability(&self) -> &str {
        &self.ability
    }

    pub fn scope_entity(&self) -> &str {
        &self.scope_entity
    }

    pub fn policy(&self) -> &str {
        &self.policy
    }

    pub fn policy_paths(&self) -> &[ApplicationAuthorizationPath] {
        &self.policy_paths
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCompiledApplicationOperationContracts {
    ability_requirements: Vec<WorthQueryInstalledAbilityRequirement>,
    graph_reads: WorthQueryOperationGraphReadContract,
    touches: WorthQueryOperationTouchContract,
    effects: WorthQueryOperationEffectContract,
    invariants: WorthQueryOperationInvariantContract,
    program: Vec<ApplicationOperationProgramTarget>,
}

impl WorthQueryCompiledApplicationOperationContracts {
    pub(crate) fn compile(
        mut ability_requirements: Vec<WorthQueryInstalledAbilityRequirement>,
        mut program: Vec<ApplicationOperationProgramTarget>,
    ) -> Self {
        ability_requirements.sort();
        ability_requirements.dedup();
        program.sort();
        program.dedup();
        let primary_role = "primary".to_string();
        let graph_reads = WorthQueryOperationGraphReadContract::Declared {
            roles: vec![WorthQueryOperationGraphReadRole {
                role: primary_role.clone(),
                participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: WorthQueryOperationGraphAccess::Project,
                semantic_reads: Vec::new(),
            }],
        };
        let touches = WorthQueryOperationTouchContract::Declared {
            graph_roles: vec![primary_role],
            scopes: program.iter().map(program_scope).collect(),
        };
        let effects = WorthQueryOperationEffectContract::Declared {
            effect_families: vec![WorthQueryOperationEffectFamily::Mutation],
        };
        Self {
            ability_requirements,
            graph_reads,
            touches,
            effects,
            invariants: WorthQueryOperationInvariantContract::NotRequired,
            program,
        }
    }

    pub fn ability_requirements(&self) -> &[WorthQueryInstalledAbilityRequirement] {
        &self.ability_requirements
    }

    pub fn graph_reads(&self) -> &WorthQueryOperationGraphReadContract {
        &self.graph_reads
    }

    pub fn touches(&self) -> &WorthQueryOperationTouchContract {
        &self.touches
    }

    pub fn effects(&self) -> &WorthQueryOperationEffectContract {
        &self.effects
    }

    pub fn invariants(&self) -> &WorthQueryOperationInvariantContract {
        &self.invariants
    }

    pub fn program(&self) -> &[ApplicationOperationProgramTarget] {
        &self.program
    }
}

fn program_scope(target: &ApplicationOperationProgramTarget) -> String {
    match target {
        ApplicationOperationProgramTarget::Create { entity } => format!("create:{entity}"),
        ApplicationOperationProgramTarget::Delete { entity } => format!("delete:{entity}"),
        ApplicationOperationProgramTarget::Write {
            entity,
            aspect,
            field,
        } => format!("write:{entity}/{aspect}/{field}"),
        ApplicationOperationProgramTarget::Link { relation, from, to } => {
            format!("link:{relation}:{from}->{to}")
        }
        ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
            format!("unlink:{relation}:{from}->{to}")
        }
        ApplicationOperationProgramTarget::Emit { effect } => format!("emit:{effect}"),
    }
}
