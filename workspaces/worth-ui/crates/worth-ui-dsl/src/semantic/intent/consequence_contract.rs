#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIntentConsequenceContractSpec {
    mounted_posture: bool,
    query_collection_change: Option<Box<str>>,
}

impl WorthUiIntentConsequenceContractSpec {
    pub const fn none() -> Self {
        Self {
            mounted_posture: false,
            query_collection_change: None,
        }
    }

    pub const fn mounted_posture() -> Self {
        Self {
            mounted_posture: true,
            query_collection_change: None,
        }
    }

    pub fn query_collection_change(query: impl Into<Box<str>>) -> Self {
        Self {
            mounted_posture: false,
            query_collection_change: Some(query.into()),
        }
    }

    pub fn mounted_posture_and_query(query: impl Into<Box<str>>) -> Self {
        Self {
            mounted_posture: true,
            query_collection_change: Some(query.into()),
        }
    }

    pub const fn includes_mounted_posture(&self) -> bool {
        self.mounted_posture
    }

    pub fn query_collection_change_identity(&self) -> Option<&str> {
        self.query_collection_change.as_deref()
    }

    pub(crate) fn revision_token(&self) -> String {
        match (
            self.mounted_posture,
            self.query_collection_change.as_deref(),
        ) {
            (false, None) => "consequences:none".into(),
            (true, None) => "consequences:mounted-posture".into(),
            (false, Some(query)) => format!("consequences:query-collection-change:{query}"),
            (true, Some(query)) => {
                format!("consequences:mounted-posture+query-collection-change:{query}")
            }
        }
    }
}
