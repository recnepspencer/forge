use worth_foundational::facade::{CanonicalDigestDerivationDenial, CanonicalDigestWorkBudget};
use worth_query_declaration::facade::application_query::{
    ApplicationQueryResultTraversalDirection, ErasedApplicationQueryDefinition,
};

use super::{
    canonical_basis::{prepare_continuation_basis, ContinuationCanonicalInput},
    WorthQueryApplicationCanonicalArtifact, WorthQueryInstalledGraphOrdering,
    WorthQueryInstalledGraphReadContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledApplicationContinuationContract {
    canonical: WorthQueryApplicationCanonicalArtifact,
    collection_path: String,
    slot_type: String,
    relation: String,
    parent_entity: String,
    child_entity: String,
    direction: ApplicationQueryResultTraversalDirection,
    ordering: Vec<WorthQueryInstalledGraphOrdering>,
}

impl WorthQueryInstalledApplicationContinuationContract {
    pub(super) fn compile(
        definition: &ErasedApplicationQueryDefinition,
        graph: &WorthQueryInstalledGraphReadContract,
        budget: CanonicalDigestWorkBudget,
    ) -> Result<Option<Self>, CanonicalDigestDerivationDenial> {
        let Some(target) = definition.continuation() else {
            return Ok(None);
        };
        let relation = graph
            .relations()
            .iter()
            .find(|relation| relation.slot_type() == target.slot_type())
            .expect("validated continuation target resolves to one installed relation");
        let ordering = graph
            .ordering()
            .iter()
            .filter(|ordering| ordering.collection_path() == relation.result_path())
            .cloned()
            .collect::<Vec<_>>();
        debug_assert!(!ordering.is_empty());
        let collection_path = relation.result_path().to_string();
        let slot_type = relation.slot_type().to_string();
        let relation_name = relation.relation().to_string();
        let parent_entity = relation.parent_entity().to_string();
        let child_entity = relation.child_entity().to_string();
        let direction = relation.direction();
        let canonical = prepare_continuation_basis(
            &ContinuationCanonicalInput {
                graph_digest: graph.digest(),
                collection_path: &collection_path,
                slot_type: &slot_type,
                relation: &relation_name,
                parent_entity: &parent_entity,
                child_entity: &child_entity,
                direction,
                ordering: &ordering,
            },
            budget,
        )?;
        Ok(Some(Self {
            canonical,
            collection_path,
            slot_type,
            relation: relation_name,
            parent_entity,
            child_entity,
            direction,
            ordering,
        }))
    }

    pub fn digest(&self) -> &worth_foundational::facade::CanonicalDigestId {
        self.canonical.digest()
    }

    pub fn canonical_basis(&self) -> &WorthQueryApplicationCanonicalArtifact {
        &self.canonical
    }

    pub fn collection_path(&self) -> &str {
        &self.collection_path
    }

    pub fn slot_type(&self) -> &str {
        &self.slot_type
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn parent_entity(&self) -> &str {
        &self.parent_entity
    }

    pub fn child_entity(&self) -> &str {
        &self.child_entity
    }

    pub const fn direction(&self) -> ApplicationQueryResultTraversalDirection {
        self.direction
    }

    pub fn ordering(&self) -> &[WorthQueryInstalledGraphOrdering] {
        &self.ordering
    }
}
