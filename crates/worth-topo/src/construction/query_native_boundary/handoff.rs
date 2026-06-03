use super::birth_synopsis::TopologyPrimitiveConstructionQueryBirthSynopsis;
use super::envelope::TopologyPrimitiveConstructionQueryEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionQueryHandoff {
    handoff_name: &'static str,
    birth_synopsis: TopologyPrimitiveConstructionQueryBirthSynopsis,
    topology_query_envelope: TopologyPrimitiveConstructionQueryEnvelope,
    handoff_digest: String,
}

impl TopologyPrimitiveConstructionQueryHandoff {
    pub(crate) fn new(
        birth_synopsis: TopologyPrimitiveConstructionQueryBirthSynopsis,
        topology_query_envelope: TopologyPrimitiveConstructionQueryEnvelope,
    ) -> Self {
        let handoff_name = "worth-topo.query-native-construction-handoff";
        let parts = [
            handoff_name.to_string(),
            birth_synopsis.source_birth_digest().to_string(),
            birth_synopsis.topology_birth_class().to_string(),
            topology_query_envelope.envelope_digest().to_string(),
        ];
        Self {
            handoff_name,
            birth_synopsis,
            topology_query_envelope,
            handoff_digest: super::digest_parts(&parts),
        }
    }

    pub fn handoff_name(&self) -> &str {
        self.handoff_name
    }

    pub fn birth_synopsis(&self) -> &TopologyPrimitiveConstructionQueryBirthSynopsis {
        &self.birth_synopsis
    }

    pub fn source_birth_digest(&self) -> &str {
        self.birth_synopsis.source_birth_digest()
    }

    pub fn topology_birth_class(&self) -> &str {
        self.birth_synopsis.topology_birth_class()
    }

    pub fn scaffold_digest(&self) -> &str {
        self.birth_synopsis.scaffold_digest()
    }

    pub fn family(&self) -> super::birth_synopsis::TopologyPrimitiveConstructionBirthFamily {
        self.birth_synopsis.family()
    }

    pub fn topology_query_envelope(&self) -> &TopologyPrimitiveConstructionQueryEnvelope {
        &self.topology_query_envelope
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}
