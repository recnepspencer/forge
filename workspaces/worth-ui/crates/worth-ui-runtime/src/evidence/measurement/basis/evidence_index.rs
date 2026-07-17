use std::collections::BTreeMap;

use crate::evidence::measurement::MeasurementEvidenceInput;
use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct UiMeasurementEvidenceIndex {
    query_by_source: BTreeMap<
        worth_ui_query_binding::WorthUiQueryAuthorityIndexKey,
        Vec<super::UiQueryAllocationTargetMapping>,
    >,
    host_by_request: BTreeMap<worth_ui_host_contract::UiMeasurementRequestIdentity, usize>,
    durable_by_input: BTreeMap<u64, usize>,
}

impl UiMeasurementEvidenceIndex {
    pub(super) fn build(
        inputs: &[MeasurementEvidenceInput],
        query_target: UiGraphNodeIdentity,
    ) -> Self {
        let mut index = Self::default();
        for (position, input) in inputs.iter().enumerate() {
            if let Some(receipt) = input.as_query_projection_fact() {
                let mapping = super::UiQueryAllocationTargetMapping::from_admitted_receipt(
                    receipt,
                    query_target,
                );
                let rows = index
                    .query_by_source
                    .entry(mapping.authority_index_key().clone())
                    .or_default();
                if !rows.iter().any(|row| row == &mapping) {
                    rows.push(mapping);
                    rows.sort_by_key(super::UiQueryAllocationTargetMapping::identity_digest);
                }
            }
            if let Some(result) = input.as_host_measurement_result() {
                index
                    .host_by_request
                    .insert(result.request_identity(), position);
            }
            if let Some(support) = input.as_sibling_resize_support().filter(|support| {
                support.source()
                    == crate::evidence::UiMeasurementSiblingResizeSupportSource::RuntimeDurableResizeWitness
            }) {
                index
                    .durable_by_input
                    .insert(support.source_identity_digest(), position);
            }
        }
        index
    }

    pub(super) fn query_mappings(
        &self,
        authority_index_key: &worth_ui_query_binding::WorthUiQueryAuthorityIndexKey,
    ) -> &[super::UiQueryAllocationTargetMapping] {
        self.query_by_source
            .get(authority_index_key)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub(super) fn host_position(
        &self,
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    ) -> Option<usize> {
        self.host_by_request.get(&request_identity).copied()
    }

    pub(super) fn durable_position(&self, input_identity_digest: u64) -> Option<usize> {
        self.durable_by_input.get(&input_identity_digest).copied()
    }

    pub(super) fn query_rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &worth_ui_query_binding::WorthUiQueryAuthorityIndexKey,
            &super::UiQueryAllocationTargetMapping,
        ),
    > {
        self.query_by_source
            .iter()
            .flat_map(|(authority, mappings)| {
                mappings.iter().map(move |mapping| (authority, mapping))
            })
    }

    pub(super) fn host_requests(
        &self,
    ) -> impl Iterator<Item = worth_ui_host_contract::UiMeasurementRequestIdentity> + '_ {
        self.host_by_request.keys().copied()
    }

    pub(super) fn durable_inputs(&self) -> impl Iterator<Item = u64> + '_ {
        self.durable_by_input.keys().copied()
    }
}
