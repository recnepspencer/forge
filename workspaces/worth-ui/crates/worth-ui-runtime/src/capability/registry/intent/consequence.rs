/// Owner families a concrete product outcome is allowed to return.
///
/// This is definition meaning: the associated outcome type fixes the maximum
/// set, while each authored declaration must narrow that set further.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentProductConsequenceFamilies {
    query_collection_change: bool,
    query_projection: bool,
}

/// Typed, owner-issued consequences returned with one completed product
/// outcome. Fixed fields make duplicate owner families unrepresentable.
#[must_use]
pub struct UiIntentProductConsequences {
    query_collection_change: Option<worth_ui_query_binding::WorthUiCollectionChangeConsequence>,
    query_projection: Option<worth_ui_query_binding::UiProjectionObservation>,
}

impl UiIntentProductConsequenceFamilies {
    pub const NONE: Self = Self {
        query_collection_change: false,
        query_projection: false,
    };

    pub const QUERY_COLLECTION_CHANGE: Self = Self {
        query_collection_change: true,
        query_projection: false,
    };

    pub const QUERY_PROJECTION: Self = Self {
        query_collection_change: false,
        query_projection: true,
    };

    pub const fn permits_query_collection_change(self) -> bool {
        self.query_collection_change
    }

    pub const fn permits_query_projection(self) -> bool {
        self.query_projection
    }
}

impl UiIntentProductConsequences {
    pub const fn none() -> Self {
        Self {
            query_collection_change: None,
            query_projection: None,
        }
    }

    pub fn query_collection_change(
        consequence: worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    ) -> Self {
        Self {
            query_collection_change: Some(consequence),
            query_projection: None,
        }
    }

    pub fn query_projection(observation: worth_ui_query_binding::UiProjectionObservation) -> Self {
        Self {
            query_collection_change: None,
            query_projection: Some(observation),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Option<worth_ui_query_binding::WorthUiCollectionChangeConsequence>,
        Option<worth_ui_query_binding::UiProjectionObservation>,
    ) {
        (self.query_collection_change, self.query_projection)
    }
}
