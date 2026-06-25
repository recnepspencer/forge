use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiHostAvailableBoundsObservation, WorthUiHostFrameObservationDraft,
    WorthUiHostIconMetricObservation, WorthUiHostMeasurementReadinessPosture,
    WorthUiHostObservationAdmissionDenial, WorthUiHostObservationAdmissionDenialCode,
    WorthUiHostObservationBasis, WorthUiHostObservationCounters,
    WorthUiHostScrollViewportObservation, WorthUiHostTextMetricObservation,
    WorthUiHostViewportObservation, WorthUiMountedProductViewReceipt, WorthUiRuntimeFactId,
    WorthUiRuntimeHost,
};

use super::canonical::canonical_observation_parts;
use super::digest::digest_parts;
use super::metric_validation::validate_metric_rows;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAdmittedHostFrameObservationReceipt {
    basis: WorthUiHostObservationBasis,
    readiness: WorthUiHostMeasurementReadinessPosture,
    available_bounds: Vec<WorthUiHostAvailableBoundsObservation>,
    viewports: Vec<WorthUiHostViewportObservation>,
    scroll_viewports: Vec<WorthUiHostScrollViewportObservation>,
    text_metrics: Vec<WorthUiHostTextMetricObservation>,
    icon_metrics: Vec<WorthUiHostIconMetricObservation>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiHostObservationCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMeasuredProductViewReceipt {
    mounted_product_view: WorthUiMountedProductViewReceipt,
    host_observations: WorthUiAdmittedHostFrameObservationReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

impl WorthUiRuntimeHost {
    pub fn admit_host_frame_observations(
        &self,
        mounted_product_view: &WorthUiMountedProductViewReceipt,
        draft: WorthUiHostFrameObservationDraft,
    ) -> Result<
        WorthUiAdmittedHostFrameObservationReceipt,
        Vec<WorthUiHostObservationAdmissionDenial>,
    > {
        WorthUiAdmittedHostFrameObservationReceipt::admit(mounted_product_view, draft)
    }

    pub fn measure_mounted_product_view(
        &self,
        mounted_product_view: &WorthUiMountedProductViewReceipt,
        observations: WorthUiAdmittedHostFrameObservationReceipt,
    ) -> Result<WorthUiMeasuredProductViewReceipt, WorthUiHostObservationAdmissionDenial> {
        WorthUiMeasuredProductViewReceipt::new(mounted_product_view.clone(), observations)
    }
}

impl WorthUiAdmittedHostFrameObservationReceipt {
    fn admit(
        mounted_product_view: &WorthUiMountedProductViewReceipt,
        draft: WorthUiHostFrameObservationDraft,
    ) -> Result<Self, Vec<WorthUiHostObservationAdmissionDenial>> {
        let mut denials = Vec::new();
        if draft.basis().mounted_product_view_digest() != mounted_product_view.receipt_digest() {
            denials.push(WorthUiHostObservationAdmissionDenial::new(
                WorthUiHostObservationAdmissionDenialCode::StaleMountedProductView,
                draft.basis().mounted_product_view_digest().to_string(),
            ));
        }
        let known_nodes = mounted_node_ids(mounted_product_view);
        validate_node_rows(
            "available_bounds",
            draft.available_bounds().iter().map(|row| row.node_id()),
            &known_nodes,
            &mut denials,
        );
        validate_node_rows(
            "viewport",
            draft.viewports().iter().map(|row| row.node_id()),
            &known_nodes,
            &mut denials,
        );
        validate_node_rows(
            "scroll_viewport",
            draft.scroll_viewports().iter().map(|row| row.node_id()),
            &known_nodes,
            &mut denials,
        );
        validate_node_rows(
            "text_metric",
            draft.text_metrics().iter().map(|row| row.node_id()),
            &known_nodes,
            &mut denials,
        );
        validate_node_rows(
            "icon_metric",
            draft.icon_metrics().iter().map(|row| row.node_id()),
            &known_nodes,
            &mut denials,
        );
        validate_metric_rows(&draft, &mut denials);
        if !denials.is_empty() {
            denials.sort_by(|left, right| {
                left.code()
                    .token()
                    .cmp(right.code().token())
                    .then_with(|| left.subject().cmp(right.subject()))
            });
            return Err(denials);
        }
        let counters = WorthUiHostObservationCounters::from_counts(
            draft.available_bounds().len(),
            draft.viewports().len(),
            draft.scroll_viewports().len(),
            draft.text_metrics().len(),
            draft.icon_metrics().len(),
            usize::from(draft.dpi_scale().is_some()),
            draft.elapsed_time().len(),
        );
        let readiness = if draft.available_bounds().is_empty() {
            WorthUiHostMeasurementReadinessPosture::MissingAvailableBounds
        } else {
            WorthUiHostMeasurementReadinessPosture::Ready
        };
        let observation_digest =
            digest_parts(canonical_observation_parts(&draft, readiness, counters));
        let consumed_facts = vec![WorthUiRuntimeFactId::host_measurement_observation(
            observation_digest.to_string(),
        )];
        let receipt_parts = [
            "admitted_host_frame_observation".to_owned(),
            observation_digest.to_string(),
        ]
        .into_iter()
        .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned()));
        let receipt_digest = digest_parts(receipt_parts);
        Ok(Self {
            basis: draft.basis().clone(),
            readiness,
            available_bounds: draft.available_bounds().to_vec(),
            viewports: draft.viewports().to_vec(),
            scroll_viewports: draft.scroll_viewports().to_vec(),
            text_metrics: draft.text_metrics().to_vec(),
            icon_metrics: draft.icon_metrics().to_vec(),
            consumed_facts,
            counters,
            receipt_digest,
        })
    }

    pub fn basis(&self) -> &WorthUiHostObservationBasis {
        &self.basis
    }

    pub fn readiness(&self) -> WorthUiHostMeasurementReadinessPosture {
        self.readiness
    }

    pub fn available_bounds(&self) -> &[WorthUiHostAvailableBoundsObservation] {
        &self.available_bounds
    }

    pub fn viewports(&self) -> &[crate::runtime::WorthUiHostViewportObservation] {
        &self.viewports
    }

    pub fn scroll_viewports(&self) -> &[crate::runtime::WorthUiHostScrollViewportObservation] {
        &self.scroll_viewports
    }

    pub fn text_metrics(&self) -> &[WorthUiHostTextMetricObservation] {
        &self.text_metrics
    }

    pub fn icon_metrics(&self) -> &[WorthUiHostIconMetricObservation] {
        &self.icon_metrics
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn counters(&self) -> WorthUiHostObservationCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMeasuredProductViewReceipt {
    fn new(
        mounted_product_view: WorthUiMountedProductViewReceipt,
        host_observations: WorthUiAdmittedHostFrameObservationReceipt,
    ) -> Result<Self, WorthUiHostObservationAdmissionDenial> {
        if host_observations.basis().mounted_product_view_digest()
            != mounted_product_view.receipt_digest()
        {
            return Err(WorthUiHostObservationAdmissionDenial::new(
                WorthUiHostObservationAdmissionDenialCode::StaleMountedProductView,
                host_observations
                    .basis()
                    .mounted_product_view_digest()
                    .to_string(),
            ));
        }
        let mut consumed_facts = mounted_product_view.consumed_facts().to_vec();
        consumed_facts.extend(host_observations.consumed_facts().iter().cloned());
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            [
                "measured_product_view".to_owned(),
                mounted_product_view.receipt_digest().to_string(),
                host_observations.receipt_digest().to_string(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Ok(Self {
            mounted_product_view,
            host_observations,
            consumed_facts,
            receipt_digest,
        })
    }

    pub fn mounted_product_view(&self) -> &WorthUiMountedProductViewReceipt {
        &self.mounted_product_view
    }

    pub fn host_observations(&self) -> &WorthUiAdmittedHostFrameObservationReceipt {
        &self.host_observations
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn mounted_node_ids(mounted_product_view: &WorthUiMountedProductViewReceipt) -> BTreeSet<String> {
    mounted_product_view
        .composition_tree()
        .graph_access()
        .child_rows()
        .iter()
        .map(|row| row.node().node_id().as_str().to_owned())
        .collect()
}

fn validate_node_rows<'a>(
    family: &str,
    rows: impl Iterator<Item = &'a str>,
    known_nodes: &BTreeSet<String>,
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
) {
    let mut seen = BTreeSet::new();
    for node_id in rows {
        let key = format!("{family}:{node_id}");
        if !seen.insert(key.clone()) {
            denials.push(WorthUiHostObservationAdmissionDenial::new(
                WorthUiHostObservationAdmissionDenialCode::DuplicateObservationRow,
                key.clone(),
            ));
        }
        if !known_nodes.contains(node_id) {
            denials.push(WorthUiHostObservationAdmissionDenial::new(
                WorthUiHostObservationAdmissionDenialCode::UnknownMountedNode,
                key,
            ));
        }
    }
}
