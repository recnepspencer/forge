use serde::{Deserialize, Serialize};

use crate::{CdcTouchedAspectScope, EntitySetUniformAspectScope, SingleEntityAspectScope};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StableBasisReadScope {
    SingleEntity(SingleEntityAspectScope),
    UniformEntitySet(EntitySetUniformAspectScope),
    CdcTouched(CdcTouchedAspectScope),
}

impl StableBasisReadScope {
    pub fn fingerprint(&self) -> String {
        match self {
            Self::SingleEntity(scope) => format!("single:{}", scope.entity_id()),
            Self::UniformEntitySet(scope) => {
                format!("uniform:{}", scope.entity_ids().join(","))
            }
            Self::CdcTouched(scope) => {
                format!(
                    "cdc:{}:{}",
                    scope.cdc_token(),
                    scope.touched_entity_ids().join(",")
                )
            }
        }
    }
}
