use super::ForgeQueryPersistentGraphIndexRequirementDeclaration;
use crate::identity::hash_parts;
use crate::runtime::ForgeQueryGraphReadAccessRequirementSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadFamilyIndexContract {
    digest: String,
    read_graph_digest: String,
    access_shape_digest: String,
    selectivity_shape_digest: String,
    requirement_set_digest: String,
    persistent_requirement_digest: Option<String>,
    requirement_row_digests: Vec<String>,
}

impl ForgeQueryGraphReadFamilyIndexContract {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn access_shape_digest(&self) -> &str {
        &self.access_shape_digest
    }

    pub fn selectivity_shape_digest(&self) -> &str {
        &self.selectivity_shape_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn persistent_requirement_digest(&self) -> Option<&str> {
        self.persistent_requirement_digest.as_deref()
    }

    pub fn requirement_row_digests(&self) -> &[String] {
        &self.requirement_row_digests
    }

    pub(crate) fn from_admission_parts(
        requirements: &ForgeQueryGraphReadAccessRequirementSet,
        persistent_requirement: Option<&ForgeQueryPersistentGraphIndexRequirementDeclaration>,
    ) -> Self {
        let read_graph_digest = requirements.read_graph_digest().to_string();
        let access_shape_digest = requirements.access_shape_digest().to_string();
        let selectivity_shape_digest = requirements.selectivity_shape_digest().to_string();
        let requirement_set_digest = requirements.digest().as_str().to_string();
        let persistent_requirement_digest =
            persistent_requirement.map(|requirement| requirement.digest().to_string());
        let requirement_row_digests = requirements
            .rows()
            .iter()
            .map(|row| hash_parts(&[row.digest_part()]))
            .collect::<Vec<_>>();
        let digest = hash_parts(
            &[
                "forge_query_graph_read_family_index_contract_v1".to_string(),
                format!("read_graph:{read_graph_digest}"),
                format!("access_shape:{access_shape_digest}"),
                format!("selectivity_shape:{selectivity_shape_digest}"),
                format!("requirements:{requirement_set_digest}"),
                format!(
                    "persistent_requirement:{}",
                    persistent_requirement_digest.as_deref().unwrap_or("none")
                ),
                format!("requirement_row_count:{}", requirement_row_digests.len()),
            ]
            .into_iter()
            .chain(
                requirement_row_digests
                    .iter()
                    .map(|digest| format!("requirement_row:{digest}")),
            )
            .collect::<Vec<_>>(),
        );
        Self {
            digest,
            read_graph_digest,
            access_shape_digest,
            selectivity_shape_digest,
            requirement_set_digest,
            persistent_requirement_digest,
            requirement_row_digests,
        }
    }
}
