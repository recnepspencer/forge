use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareLiveRelevanceContract {
    authorized_fields: Vec<String>,
    digest: String,
}

impl PolicyAwareLiveRelevanceContract {
    pub(crate) fn new(authorized_fields: Vec<String>) -> Self {
        let digest = hash_parts(
            &authorized_fields
                .iter()
                .map(|field| format!("authorized_live_relevance:{field}"))
                .collect::<Vec<_>>(),
        );
        Self {
            authorized_fields,
            digest,
        }
    }

    pub fn authorized_fields(&self) -> &[String] {
        &self.authorized_fields
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
