use crate::authorized_projection::AuthorizedProjectionFieldPath;
#[cfg(test)]
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareLiveRelevanceContract {
    authorized_field_paths: Vec<AuthorizedProjectionFieldPath>,
    digest: String,
}

impl PolicyAwareLiveRelevanceContract {
    #[cfg(test)]
    pub(crate) fn new(authorized_field_paths: Vec<AuthorizedProjectionFieldPath>) -> Self {
        let digest = hash_parts(
            &authorized_field_paths
                .iter()
                .map(|field| {
                    format!(
                        "authorized_live_relevance:{}",
                        field.terminal_projection_for_boundary()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            authorized_field_paths,
            digest,
        }
    }

    pub fn authorized_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.authorized_field_paths
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
