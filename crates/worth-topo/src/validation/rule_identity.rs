use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyValidationRuleIdentity {
    namespace: String,
    name: String,
    version: u16,
}

impl TopologyValidationRuleIdentity {
    fn registered(name: &'static str) -> Self {
        Self {
            namespace: "worth.topo.validation".to_string(),
            name: name.to_string(),
            version: 1,
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn stable_key(&self) -> String {
        format!("{}:{}:v{}", self.namespace, self.name, self.version)
    }

    pub fn is_registered(&self) -> bool {
        self.namespace == "worth.topo.validation"
            && self.version == 1
            && matches!(
                self.name.as_str(),
                "ownership" | "loop_wiring" | "radial_rings" | "shell_closure" | "vertex_disks"
            )
    }
}

pub fn ownership_rule() -> TopologyValidationRuleIdentity {
    TopologyValidationRuleIdentity::registered("ownership")
}

pub fn loop_wiring_rule() -> TopologyValidationRuleIdentity {
    TopologyValidationRuleIdentity::registered("loop_wiring")
}

pub fn radial_rings_rule() -> TopologyValidationRuleIdentity {
    TopologyValidationRuleIdentity::registered("radial_rings")
}

pub fn shell_closure_rule() -> TopologyValidationRuleIdentity {
    TopologyValidationRuleIdentity::registered("shell_closure")
}

pub fn vertex_disks_rule() -> TopologyValidationRuleIdentity {
    TopologyValidationRuleIdentity::registered("vertex_disks")
}
