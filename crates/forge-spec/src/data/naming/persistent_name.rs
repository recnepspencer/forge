use serde::{Deserialize, Serialize};

use crate::data::identity::NamingAnchorId;
use crate::data::schema::SpecNodeKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentName {
    pub anchor_id: NamingAnchorId,
    pub expected_kind: SpecNodeKind,
    pub semantic_subselector: Option<String>,
}
