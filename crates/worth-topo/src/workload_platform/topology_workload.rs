use super::{
    TopologyWorkloadCounters, TopologyWorkloadDeclarationIdentity, TopologyWorkloadEnvelope,
    TopologyWorkloadFamily, TopologyWorkloadSupportPosture,
};

pub struct TopologyWorkload;

impl TopologyWorkload {
    pub fn declared(name: impl Into<String>) -> TopologyWorkloadDeclaration {
        TopologyWorkloadDeclaration { name: name.into() }
    }
}

pub struct TopologyWorkloadDeclaration {
    name: String,
}

impl TopologyWorkloadDeclaration {
    pub fn from_query_declaration(
        self,
        query_declaration: impl Into<String>,
    ) -> Result<TopologyWorkloadReceipt, TopologyWorkloadDenial> {
        let query_declaration = query_declaration.into();
        if self.name.trim().is_empty() {
            return Err(TopologyWorkloadDenial::MissingDeclarationName);
        }
        if query_declaration.trim().is_empty() {
            return Err(TopologyWorkloadDenial::MissingQueryDeclaration);
        }

        let identity = TopologyWorkloadDeclarationIdentity::new(self.name, query_declaration);
        let support_posture =
            TopologyWorkloadSupportPosture::admitted(TopologyWorkloadFamily::SeededTopology);
        let envelope = TopologyWorkloadEnvelope::new(
            identity.clone(),
            support_posture,
            TopologyWorkloadCounters::new(1, 1),
        );
        Ok(TopologyWorkloadReceipt { identity, envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyWorkloadReceipt {
    identity: TopologyWorkloadDeclarationIdentity,
    envelope: TopologyWorkloadEnvelope,
}

impl TopologyWorkloadReceipt {
    pub fn identity(&self) -> &TopologyWorkloadDeclarationIdentity {
        &self.identity
    }

    pub fn envelope(&self) -> &TopologyWorkloadEnvelope {
        &self.envelope
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyWorkloadDenial {
    MissingDeclarationName,
    MissingQueryDeclaration,
}

impl TopologyWorkloadDenial {
    pub fn human_reason(self) -> &'static str {
        match self {
            Self::MissingDeclarationName => "topology workload requires a declaration name",
            Self::MissingQueryDeclaration => {
                "topology workload requires a Forge Query declaration identity"
            }
        }
    }
}
