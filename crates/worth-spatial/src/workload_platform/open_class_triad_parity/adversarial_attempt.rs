use super::denial::{OpenClassTriadParityDenial, OpenClassTriadParityDenialKind};
use super::lane_set::OpenClassParityLaneSet;
use super::open_class::OpenTopologyClass;
use super::receipt::OpenClassTriadParityReceipt;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;

impl OpenClassTriadParityReceipt {
    pub fn attempt_cross_class_checkpoint_replay(
        &self,
        source: OpenTopologyClass,
        target: OpenTopologyClass,
    ) -> Result<(), OpenClassTriadParityDenial> {
        let source_set = require_class(self, source)?;
        let target_set = require_class(self, target)?;
        if source == target {
            return Ok(());
        }
        Err(OpenClassTriadParityDenial::with_source(
            OpenClassTriadParityDenialKind::CrossClassCheckpointReplay,
            source,
            target,
            Some(ProjectionFactParityLane::Retained),
            format!(
                "Retained checkpoint from {} cannot satisfy {} parity; source retained lane {} targets topology {}.",
                source.human_name(),
                target.human_name(),
                source_set.retained_lane_identity().unwrap_or("missing retained lane"),
                target_set.topology_identity()
            ),
        ))
    }

    pub fn attempt_projection_consumed_as_retained(
        &self,
        target: OpenTopologyClass,
    ) -> Result<(), OpenClassTriadParityDenial> {
        let target_set = require_class(self, target)?;
        Err(OpenClassTriadParityDenial::with_source(
            OpenClassTriadParityDenialKind::ProjectionConsumptionMismatch,
            target,
            target,
            Some(ProjectionFactParityLane::ProjectionConsumed),
            format!(
                "Projection-consumed facts for {} cannot be replayed as retained checkpoint evidence; projection lane {} is checked at the projection-consumption boundary.",
                target.human_name(),
                target_set
                    .projection_consumed_lane_identity()
                    .unwrap_or("missing projection-consumed lane")
            ),
        ))
    }

    pub fn attempt_storm_extraction_bundle_link(
        &self,
        target: OpenTopologyClass,
        storm_bundle_digest: &str,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::new(
            OpenClassTriadParityDenialKind::StormExtractionUnsupported,
            Some(target),
            format!(
                "Closed storm extraction bundle {storm_bundle_digest} is not valid authority for {}; open classes must keep their own topology parity receipts.",
                target.human_name()
            ),
        ))
    }

    pub fn attempt_denied_lane_upgrade(
        &self,
        target: OpenTopologyClass,
        lane: ProjectionFactParityLane,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::new(
            OpenClassTriadParityDenialKind::DeniedLaneUpgrade,
            Some(target),
            format!(
                "{} cannot upgrade a denied parity state through the {}.",
                target.human_name(),
                lane.human_name()
            ),
        ))
    }

    pub fn attempt_missing_lane_evidence(
        &self,
        target: OpenTopologyClass,
        lane: ProjectionFactParityLane,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::new(
            OpenClassTriadParityDenialKind::MissingLaneEvidence,
            Some(target),
            format!(
                "{} has no options without the {}; parity cannot fall back to a generic integrity mismatch.",
                target.human_name(),
                lane.human_name()
            ),
        ))
    }
}

fn require_class(
    receipt: &OpenClassTriadParityReceipt,
    topology_class: OpenTopologyClass,
) -> Result<&OpenClassParityLaneSet, OpenClassTriadParityDenial> {
    receipt
        .require_class_for_attempt(topology_class)
        .ok_or_else(|| {
            OpenClassTriadParityDenial::new(
                OpenClassTriadParityDenialKind::MissingOpenClass,
                Some(topology_class),
                format!(
                    "Open-class triad parity is missing {} evidence.",
                    topology_class.human_name()
                ),
            )
        })
}
