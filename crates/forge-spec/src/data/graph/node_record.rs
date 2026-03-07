use serde::{Deserialize, Serialize};

use crate::data::identity::SpecNodeId;
use crate::data::payload::PayloadKey;
use crate::data::schema::SpecNodeKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeRecord {
    pub id: SpecNodeId,
    pub kind: SpecNodeKind,
    pub payload: Option<PayloadKey>,
}
