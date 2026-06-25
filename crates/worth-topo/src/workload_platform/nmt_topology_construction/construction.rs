use super::counters::NmtTopologyConstructionCounters;
use super::denial::{
    missing_declaration, missing_required_evidence, topology_validation,
    NmtTopologyConstructionDenial,
};
use super::pattern_spec::{
    NmtTopologyPattern, OpenLayerStackSpec, OpenRadialFanSpec, OpenSheetPatchSpec,
    OpenWireChainSpec,
};
use super::posture::TopologyPostureReceipt;
use super::query_receipts::NmtTopologyConstructionQueryReceipts;
use super::receipts::NmtTopologyConstructionReceipt;
use super::topology_records::build_nmt_topology_view;
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::bootstrap_topology_interpretation;
use crate::validation::TopologyValidator;
use crate::workload_platform::topology_seed::{
    TopologySeedCounters, TopologySeedEntityIdentities, TopologySeedKind,
    TopologySeedQueryReceipts, TopologySeedReceipt, TopologySeedValidationReceipt,
};

pub struct NmtTopologyConstruction {
    pattern: NmtTopologyPattern,
    declaration: String,
}

impl NmtTopologyConstruction {
    pub fn open_wire_chain(spec: OpenWireChainSpec) -> Self {
        Self::new(NmtTopologyPattern::OpenWireChain(spec))
    }

    pub fn open_sheet_patch(spec: OpenSheetPatchSpec) -> Self {
        Self::new(NmtTopologyPattern::OpenSheetPatch(spec))
    }

    pub fn open_radial_fan(spec: OpenRadialFanSpec) -> Self {
        Self::new(NmtTopologyPattern::OpenRadialFan(spec))
    }

    pub fn open_layer_stack(spec: OpenLayerStackSpec) -> Self {
        Self::new(NmtTopologyPattern::OpenLayerStack(spec))
    }

    fn new(pattern: NmtTopologyPattern) -> Self {
        let declaration = pattern.human_name().to_string();
        Self {
            pattern,
            declaration,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn construct(
        self,
    ) -> Result<NmtTopologyConstructionReceipt, NmtTopologyConstructionDenial> {
        if self.declaration.trim().is_empty() {
            return Err(missing_declaration(self.pattern));
        }
        require_requested_evidence(&self.pattern)?;
        let query_receipts =
            NmtTopologyConstructionQueryReceipts::new(&self.pattern, self.declaration.clone())
                .map_err(|error| {
                    NmtTopologyConstructionQueryReceipts::map_denial(&self.pattern, error)
                })?;
        let topology = build_nmt_topology_view(&self.pattern)?;
        let interpreted = interpret_validated_topology(&self.pattern, &topology)?;
        let seed_receipt =
            seed_receipt_for_construction(&self.pattern, &self.declaration, &topology)?;
        let topology_posture = TopologyPostureReceipt::from_interpreted(
            &interpreted,
            topology.wires.len(),
            self.pattern.layer_count(),
        );
        let counters = NmtTopologyConstructionCounters::from_view(
            &topology,
            self.pattern.layer_count(),
            topology_posture.boundary_half_edge_count(),
            topology_posture.non_manifold_edge_count(),
            seed_receipt.validation().row_count(),
        );

        Ok(NmtTopologyConstructionReceipt::new(
            self.pattern,
            self.declaration,
            query_receipts,
            seed_receipt,
            topology_posture,
            counters,
        ))
    }
}

fn require_requested_evidence(
    pattern: &NmtTopologyPattern,
) -> Result<(), NmtTopologyConstructionDenial> {
    if let NmtTopologyPattern::OpenLayerStack(spec) = pattern {
        if !spec.requires_layer_identity() {
            return Err(missing_required_evidence(
                pattern.clone(),
                "open layer stack construction requires layer identity receipts so downstream workload stages can bind projection and replay evidence to each layer.",
            ));
        }
        if !spec.requires_boundary_receipts() {
            return Err(missing_required_evidence(
                pattern.clone(),
                "open layer stack construction requires open-boundary receipts before downstream NMT workloads can reason about no-options outcomes.",
            ));
        }
        if !spec.requires_radial_receipts() {
            return Err(missing_required_evidence(
                pattern.clone(),
                "open layer stack construction requires radial-adjacency receipts so non-manifold evidence cannot be laundered as ordinary sheet topology.",
            ));
        }
    }
    Ok(())
}

fn interpret_validated_topology(
    pattern: &NmtTopologyPattern,
    topology: &TopologyView,
) -> Result<
    crate::derived_topology::traversal_views::types::InterpretedTopologyView,
    NmtTopologyConstructionDenial,
> {
    let materialized = MaterializedTopologyView::from_complete_topology_view(topology.clone());
    let interpreted = bootstrap_topology_interpretation(&materialized);
    TopologyValidator::derived_validation_report(&materialized, &interpreted).map_err(|error| {
        topology_validation(
            pattern.clone(),
            format!(
                "{} failed {} validation before workload binding: {}",
                pattern.human_name(),
                error.validator(),
                error.message()
            ),
        )
    })?;
    Ok(interpreted)
}

fn seed_receipt_for_construction(
    pattern: &NmtTopologyPattern,
    declaration: &str,
    topology: &TopologyView,
) -> Result<TopologySeedReceipt, NmtTopologyConstructionDenial> {
    let seed_kind = match pattern {
        NmtTopologyPattern::OpenWireChain(_) => TopologySeedKind::OpenWire,
        NmtTopologyPattern::OpenSheetPatch(_) => TopologySeedKind::OpenSheet,
        NmtTopologyPattern::OpenRadialFan(_) => TopologySeedKind::OpenShellNmtEdgeFan,
        NmtTopologyPattern::OpenLayerStack(_) => TopologySeedKind::NmtOpenLayerStack,
    };
    let query_receipts =
        TopologySeedQueryReceipts::new(seed_kind, format!("topology seed for {declaration}"))
            .map_err(|error| {
                super::denial::query_admission(pattern.clone(), error.human_reason())
            })?;
    let materialized = MaterializedTopologyView::from_complete_topology_view(topology.clone());
    let interpreted = bootstrap_topology_interpretation(&materialized);
    let report = TopologyValidator::derived_validation_report(&materialized, &interpreted)
        .map_err(|error| {
            topology_validation(
                pattern.clone(),
                format!(
                    "{} failed {} validation while producing its compatibility seed receipt: {}",
                    pattern.human_name(),
                    error.validator(),
                    error.message()
                ),
            )
        })?;
    let validation = TopologySeedValidationReceipt::from_report(&report);
    let counters = TopologySeedCounters::from_view(topology, validation.row_count());
    let identities = TopologySeedEntityIdentities::from_view(topology);
    Ok(TopologySeedReceipt::new(
        seed_kind,
        query_receipts,
        identities,
        counters,
        validation,
        None,
    ))
}
