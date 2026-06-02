use super::{
    birth_synopsis::TopologyPrimitiveConstructionQueryBirthSynopsis,
    handoff::TopologyPrimitiveConstructionQueryHandoff, TopologyConstructionQueryHandoffError,
};

#[derive(Debug)]
pub enum TopologyConstructionQueryAdmittedHandoffError {
    Handoff(TopologyConstructionQueryHandoffError),
    ImpossibleBirthAttachment(String),
    BirthCompleteness(String),
}

impl std::fmt::Display for TopologyConstructionQueryAdmittedHandoffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Handoff(error) => write!(f, "{error}"),
            Self::ImpossibleBirthAttachment(reason) => write!(f, "{reason}"),
            Self::BirthCompleteness(reason) => write!(f, "{reason}"),
        }
    }
}

impl std::error::Error for TopologyConstructionQueryAdmittedHandoffError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionQueryAdmittedHandoff {
    handoff_name: &'static str,
    topology_query_handoff: TopologyPrimitiveConstructionQueryHandoff,
    birth_completeness_digest: String,
    birth_mapping_digest: String,
    supported_loop_count: usize,
    supported_body_count: usize,
    admitted_handoff_digest: String,
}

impl TopologyPrimitiveConstructionQueryAdmittedHandoff {
    fn new(
        topology_query_handoff: TopologyPrimitiveConstructionQueryHandoff,
        birth_completeness_digest: String,
        birth_mapping_digest: String,
        supported_loop_count: usize,
        supported_body_count: usize,
    ) -> Self {
        let handoff_name = "worth-topo.query-native-construction-admitted-handoff";
        let parts = [
            handoff_name.to_string(),
            topology_query_handoff.handoff_digest().to_string(),
            birth_completeness_digest.clone(),
            birth_mapping_digest.clone(),
            supported_loop_count.to_string(),
            supported_body_count.to_string(),
        ];
        Self {
            handoff_name,
            topology_query_handoff,
            birth_completeness_digest,
            birth_mapping_digest,
            supported_loop_count,
            supported_body_count,
            admitted_handoff_digest: super::digest_parts(&parts),
        }
    }

    pub fn handoff_name(&self) -> &str {
        self.handoff_name
    }

    pub fn topology_query_handoff(&self) -> &TopologyPrimitiveConstructionQueryHandoff {
        &self.topology_query_handoff
    }

    pub fn source_birth_digest(&self) -> &str {
        self.topology_query_handoff.source_birth_digest()
    }

    pub fn topology_birth_class(&self) -> &str {
        self.topology_query_handoff.topology_birth_class()
    }

    pub fn topology_query_envelope(
        &self,
    ) -> &super::envelope::TopologyPrimitiveConstructionQueryEnvelope {
        self.topology_query_handoff.topology_query_envelope()
    }

    pub fn birth_completeness_digest(&self) -> &str {
        &self.birth_completeness_digest
    }

    pub fn birth_mapping_digest(&self) -> &str {
        &self.birth_mapping_digest
    }

    pub fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    pub fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }

    pub fn admitted_handoff_digest(&self) -> &str {
        &self.admitted_handoff_digest
    }
}

pub fn prepare_primitive_construction_query_admitted_handoff(
    topology_query_handoff: &TopologyPrimitiveConstructionQueryHandoff,
    birth_completeness_digest: &str,
    birth_mapping_digest: &str,
    supported_loop_count: usize,
    supported_body_count: usize,
) -> Result<
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    if topology_query_handoff
        .topology_query_envelope()
        .fact_rows()
        .is_empty()
    {
        return Err(
            TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                "topology query admitted handoff requires a non-empty retained query envelope"
                    .to_string(),
            ),
        );
    }
    Ok(TopologyPrimitiveConstructionQueryAdmittedHandoff::new(
        topology_query_handoff.clone(),
        birth_completeness_digest.to_string(),
        birth_mapping_digest.to_string(),
        supported_loop_count,
        supported_body_count,
    ))
}

pub fn prepare_primitive_construction_query_admitted_handoff_from_synopsis(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
    birth_completeness_digest: &str,
    birth_mapping_digest: &str,
    supported_loop_count: usize,
    supported_body_count: usize,
) -> Result<
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    let topology_query_handoff =
        super::admission::prepare_primitive_construction_query_handoff(synopsis)
            .map_err(TopologyConstructionQueryAdmittedHandoffError::Handoff)?;
    prepare_primitive_construction_query_admitted_handoff(
        &topology_query_handoff,
        birth_completeness_digest,
        birth_mapping_digest,
        supported_loop_count,
        supported_body_count,
    )
}
