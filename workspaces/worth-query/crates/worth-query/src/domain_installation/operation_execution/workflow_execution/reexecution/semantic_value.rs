use super::WorthQueryWorkflowValue;

/// Typed, owned semantic projection of a workflow value.
///
/// This deliberately excludes receipt and run identity. Projection values retain
/// their declared query identity and native typed rows; receipt digests never act
/// as a substitute for result semantics.
#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryWorkflowSemanticValue {
    NotRequired,
    Bool(bool),
    I64(i64),
    U64(u64),
    Text(String),
    EntityIdentity(String),
    Projection {
        canonical_query_identity: String,
        rows: Vec<crate::memory_workspace::WorthQueryEntity>,
    },
}

impl WorthQueryWorkflowValue {
    pub(crate) fn semantic_value(&self) -> WorthQueryWorkflowSemanticValue {
        match self {
            Self::NotRequired => WorthQueryWorkflowSemanticValue::NotRequired,
            Self::Bool(value) => WorthQueryWorkflowSemanticValue::Bool(*value),
            Self::I64(value) => WorthQueryWorkflowSemanticValue::I64(*value),
            Self::U64(value) => WorthQueryWorkflowSemanticValue::U64(*value),
            Self::Text(value) => WorthQueryWorkflowSemanticValue::Text(value.clone()),
            Self::EntityIdentity(value) => {
                WorthQueryWorkflowSemanticValue::EntityIdentity(value.clone())
            }
            Self::Projection(completion) => WorthQueryWorkflowSemanticValue::Projection {
                canonical_query_identity: completion
                    .result()
                    .receipt()
                    .canonical_query_digest()
                    .to_owned(),
                rows: completion.result().rows().to_vec(),
            },
        }
    }
}
