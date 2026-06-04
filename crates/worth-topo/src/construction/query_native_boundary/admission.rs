use super::birth_synopsis::{
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};

use super::envelope::TopologyPrimitiveConstructionQueryEnvelope;
use super::handoff::TopologyPrimitiveConstructionQueryHandoff;
use super::receipt::TopologyPrimitiveConstructionQueryReceipt;

#[derive(Debug)]
pub enum TopologyConstructionQueryReceiptError {
    UnsupportedBirthClass(&'static str),
}

impl std::fmt::Display for TopologyConstructionQueryReceiptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedBirthClass(reason) => {
                write!(f, "unsupported topology construction birth class: {reason}")
            }
        }
    }
}

impl std::error::Error for TopologyConstructionQueryReceiptError {}

#[derive(Debug)]
pub enum TopologyConstructionQueryEnvelopeError {
    Receipt(TopologyConstructionQueryReceiptError),
}

impl std::fmt::Display for TopologyConstructionQueryEnvelopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Receipt(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for TopologyConstructionQueryEnvelopeError {}

#[derive(Debug)]
pub enum TopologyConstructionQueryHandoffError {
    Envelope(TopologyConstructionQueryEnvelopeError),
}

impl std::fmt::Display for TopologyConstructionQueryHandoffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Envelope(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for TopologyConstructionQueryHandoffError {}

pub fn prepare_primitive_construction_query_receipt(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<TopologyPrimitiveConstructionQueryReceipt, TopologyConstructionQueryReceiptError> {
    let admitted = primitive_birth_contract_matches_counts(synopsis);
    if !admitted {
        return Err(TopologyConstructionQueryReceiptError::UnsupportedBirthClass(
            "only admitted primitive construction birth plans may cross the topology Query-native construction receipt boundary",
        ));
    }
    Ok(TopologyPrimitiveConstructionQueryReceipt::new(synopsis))
}

pub fn prepare_primitive_construction_query_envelope(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<TopologyPrimitiveConstructionQueryEnvelope, TopologyConstructionQueryEnvelopeError> {
    let receipt = prepare_primitive_construction_query_receipt(synopsis)
        .map_err(TopologyConstructionQueryEnvelopeError::Receipt)?;
    Ok(TopologyPrimitiveConstructionQueryEnvelope::new(
        synopsis, receipt,
    ))
}

pub fn prepare_primitive_construction_query_handoff(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<TopologyPrimitiveConstructionQueryHandoff, TopologyConstructionQueryHandoffError> {
    let topology_query_envelope = prepare_primitive_construction_query_envelope(synopsis)
        .map_err(TopologyConstructionQueryHandoffError::Envelope)?;
    Ok(TopologyPrimitiveConstructionQueryHandoff::new(
        synopsis.clone(),
        topology_query_envelope,
    ))
}

fn primitive_birth_contract_matches_counts(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> bool {
    let topology = synopsis.birth_contract().topology_contract();
    synopsis.supported_vertex_count() == topology.vertex_count()
        && synopsis.supported_edge_count() == topology.edge_count()
        && synopsis.supported_loop_count() == topology.loop_count()
        && synopsis.supported_wire_count() == topology.wire_count()
        && synopsis.supported_face_count() == topology.face_count()
        && synopsis.supported_shell_count() == topology.shell_count()
        && synopsis.supported_body_count() == topology.body_count()
}
