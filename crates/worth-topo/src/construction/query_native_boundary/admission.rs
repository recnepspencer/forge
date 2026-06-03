use super::birth_synopsis::{
    TopologyPrimitiveConstructionBirthFamily, TopologyPrimitiveConstructionQueryBirthSynopsis,
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
    match synopsis.family() {
        TopologyPrimitiveConstructionBirthFamily::SimplexSolid => {
            synopsis.supported_vertex_count() == 4
                && synopsis.supported_edge_count() == 6
                && synopsis.supported_loop_count() == 4
                && synopsis.supported_wire_count() == 0
                && synopsis.supported_face_count() == 4
                && synopsis.supported_shell_count() == 1
                && synopsis.supported_body_count() == 1
        }
        TopologyPrimitiveConstructionBirthFamily::Orthotope => {
            synopsis.supported_vertex_count() == 8
                && synopsis.supported_edge_count() == 12
                && synopsis.supported_loop_count() == 6
                && synopsis.supported_wire_count() == 0
                && synopsis.supported_face_count() == 6
                && synopsis.supported_shell_count() == 1
                && synopsis.supported_body_count() == 1
        }
        TopologyPrimitiveConstructionBirthFamily::RegularPrism => {
            synopsis.supported_vertex_count() >= 6
                && synopsis.supported_vertex_count() % 2 == 0
                && synopsis.supported_edge_count() == synopsis.supported_vertex_count() * 3 / 2
                && synopsis.supported_face_count() == synopsis.supported_vertex_count() / 2 + 2
                && synopsis.supported_loop_count() == synopsis.supported_face_count()
                && synopsis.supported_wire_count() == 0
                && synopsis.supported_shell_count() == 1
                && synopsis.supported_body_count() == 1
        }
        TopologyPrimitiveConstructionBirthFamily::RegularPyramid => {
            synopsis.supported_vertex_count() >= 4
                && synopsis.supported_edge_count() == (synopsis.supported_vertex_count() - 1) * 2
                && synopsis.supported_face_count() == synopsis.supported_vertex_count()
                && synopsis.supported_loop_count() == synopsis.supported_face_count()
                && synopsis.supported_wire_count() == 0
                && synopsis.supported_shell_count() == 1
                && synopsis.supported_body_count() == 1
        }
        TopologyPrimitiveConstructionBirthFamily::WireBody => {
            synopsis.supported_vertex_count() >= 3
                && synopsis.supported_edge_count() == synopsis.supported_vertex_count()
                && synopsis.supported_loop_count() == 1
                && synopsis.supported_wire_count() == 1
                && synopsis.supported_face_count() == 0
                && synopsis.supported_shell_count() == 0
                && synopsis.supported_body_count() == 1
        }
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole => {
            synopsis.supported_vertex_count() >= 6
                && synopsis.supported_edge_count() == synopsis.supported_vertex_count()
                && synopsis.supported_loop_count() >= 2
                && synopsis.supported_wire_count() == 0
                && synopsis.supported_face_count() == 1
                && synopsis.supported_shell_count() == 1
                && synopsis.supported_body_count() == 1
        }
    }
}
