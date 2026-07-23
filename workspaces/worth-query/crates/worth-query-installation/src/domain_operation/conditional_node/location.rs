#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryConditionalNodeLocation {
    Operation {
        node_identity: String,
    },
    WorkflowStage {
        stage_identity: String,
        node_identity: String,
    },
}

impl WorthQueryConditionalNodeLocation {
    pub fn operation(node_identity: impl Into<String>) -> Result<Self, &'static str> {
        let node_identity = node_identity.into();
        validate_identity(&node_identity)?;
        Ok(Self::Operation { node_identity })
    }

    pub fn workflow_stage(
        stage_identity: impl Into<String>,
        node_identity: impl Into<String>,
    ) -> Result<Self, &'static str> {
        let stage_identity = stage_identity.into();
        let node_identity = node_identity.into();
        validate_identity(&stage_identity)?;
        validate_identity(&node_identity)?;
        Ok(Self::WorkflowStage {
            stage_identity,
            node_identity,
        })
    }

    pub fn stage_identity(&self) -> Option<&str> {
        match self {
            Self::Operation { .. } => None,
            Self::WorkflowStage { stage_identity, .. } => Some(stage_identity),
        }
    }

    pub fn node_identity(&self) -> &str {
        match self {
            Self::Operation { node_identity } | Self::WorkflowStage { node_identity, .. } => {
                node_identity
            }
        }
    }
}

fn validate_identity(value: &str) -> Result<(), &'static str> {
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_whitespace) {
        Err("invalid-conditional-node-location-identity")
    } else {
        Ok(())
    }
}
