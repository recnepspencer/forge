use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusRequiredScenarioInventory {
    scenario_ids: Vec<String>,
    inventory_digest: String,
}

impl PrimitiveConstructionCorpusRequiredScenarioInventory {
    pub(crate) fn new(scenario_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let scenario_ids = scenario_ids.into_iter().map(Into::into).collect::<Vec<_>>();
        let inventory_digest = digest_owned_parts(&scenario_ids);
        Self {
            scenario_ids,
            inventory_digest,
        }
    }

    pub(crate) fn scenario_ids(&self) -> &[String] {
        &self.scenario_ids
    }

    pub(crate) fn contains(&self, scenario_id: &str) -> bool {
        self.scenario_ids
            .iter()
            .any(|required| required == scenario_id)
    }

    pub(crate) fn all_present<T>(&self, mut lookup: impl FnMut(&str) -> Option<T>) -> bool {
        self.scenario_ids
            .iter()
            .all(|scenario_id| lookup(scenario_id).is_some())
    }

    pub(crate) fn row_for<'a, T>(
        &self,
        scenario_id: &str,
        mut lookup: impl FnMut(&str) -> Option<&'a T>,
    ) -> Option<&'a T> {
        self.contains(scenario_id)
            .then(|| lookup(scenario_id))
            .flatten()
    }

    pub(crate) fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}
