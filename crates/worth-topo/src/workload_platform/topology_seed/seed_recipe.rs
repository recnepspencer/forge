use super::{
    TopologySeedCleanFailClass, TopologySeedCleanFailReasonCode, TopologySeedCleanFailReceipt,
    TopologySeedCleanFailStage, TopologySeedCounters, TopologySeedEntityIdentities,
    TopologySeedKind, TopologySeedQueryReceipts, TopologySeedReceipt,
    TopologySeedValidationReceipt,
};
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::bootstrap_topology_interpretation;
use crate::validation::TopologyValidator;
use crate::workload_platform::topology_seed_recipes::TopologySeedRecipeOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedRecipe {
    kind: TopologySeedKind,
    requested_count: Option<usize>,
    declaration: Option<String>,
}

impl TopologySeedRecipe {
    pub(crate) fn new(kind: TopologySeedKind, requested_count: Option<usize>) -> Self {
        Self {
            kind,
            requested_count,
            declaration: None,
        }
    }

    pub fn with_declaration(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = Some(declaration.into());
        self
    }

    pub fn kind(&self) -> TopologySeedKind {
        self.kind
    }

    pub fn requested_count(&self) -> Option<usize> {
        self.requested_count
    }

    pub fn build(self) -> Result<TopologySeedReceipt, TopologySeedCleanFailReceipt> {
        let query_receipts = self.query_receipts()?;
        let output = self.build_recipe_output(&query_receipts)?;
        self.admit_recipe_output(query_receipts, output)
    }

    fn build_recipe_output(
        &self,
        query_receipts: &TopologySeedQueryReceipts,
    ) -> Result<TopologySeedRecipeOutput, TopologySeedCleanFailReceipt> {
        crate::workload_platform::topology_seed_recipes::build(self).map_err(|denial| {
            TopologySeedCleanFailReceipt::new(
                self.kind,
                TopologySeedCleanFailStage::ParameterAdmission,
                TopologySeedCleanFailClass::UnsupportedSeedParameter,
                denial.reason_code,
                denial.reason,
                Some(query_receipts.clone()),
                None,
                None,
            )
        })
    }

    fn admit_recipe_output(
        &self,
        query_receipts: TopologySeedQueryReceipts,
        output: TopologySeedRecipeOutput,
    ) -> Result<TopologySeedReceipt, TopologySeedCleanFailReceipt> {
        if let Some(denial) = dirty_seed_denial(self.kind) {
            let (identities, counters) = seed_evidence(&output.topology, 0);
            return Err(TopologySeedCleanFailReceipt::new(
                self.kind,
                TopologySeedCleanFailStage::SpatialBindingAdmission,
                TopologySeedCleanFailClass::DirtyTopology,
                denial.reason_code,
                denial.reason,
                Some(query_receipts),
                Some(identities),
                Some(counters),
            ));
        }

        let report = match validate_view(&output.topology) {
            Ok(report) => report,
            Err(error) => {
                let (identities, counters) = seed_evidence(&output.topology, 0);
                return Err(TopologySeedCleanFailReceipt::new(
                    self.kind,
                    TopologySeedCleanFailStage::TopologyValidation,
                    TopologySeedCleanFailClass::InvalidTopology,
                    TopologySeedCleanFailReasonCode::TopologyValidationRejectedSeed,
                    format!(
                        "topology seed failed {} validation before spatial binding: {}",
                        error.validator(),
                        error.message()
                    ),
                    Some(query_receipts),
                    Some(identities),
                    Some(counters),
                ));
            }
        };

        let validation = TopologySeedValidationReceipt::from_report(&report);
        let (identities, counters) = seed_evidence(&output.topology, validation.row_count());
        Ok(TopologySeedReceipt::new(
            self.kind,
            query_receipts,
            identities,
            counters,
            validation,
            output.neighborhood,
        ))
    }

    fn query_receipts(&self) -> Result<TopologySeedQueryReceipts, TopologySeedCleanFailReceipt> {
        let declaration = self
            .declaration
            .clone()
            .unwrap_or_else(|| self.kind.default_declaration());
        TopologySeedQueryReceipts::new(self.kind, declaration).map_err(|error| {
            TopologySeedCleanFailReceipt::new(
                self.kind,
                TopologySeedCleanFailStage::ParameterAdmission,
                TopologySeedCleanFailClass::WorkloadDeclaration,
                TopologySeedCleanFailReasonCode::WorkloadDeclarationRejectedSeed,
                error.human_reason(),
                None,
                None,
                None,
            )
        })
    }
}

fn validate_view(
    view: &TopologyView,
) -> Result<crate::validation::TopologyValidationReport, crate::validation::TopologyValidationError>
{
    let materialized = MaterializedTopologyView::from_complete_topology_view(view.clone());
    let interpreted = bootstrap_topology_interpretation(&materialized);
    TopologyValidator::derived_validation_report(&materialized, &interpreted)
}

fn seed_evidence(
    view: &TopologyView,
    validation_row_count: usize,
) -> (TopologySeedEntityIdentities, TopologySeedCounters) {
    (
        TopologySeedEntityIdentities::from_view(view),
        TopologySeedCounters::from_view(view, validation_row_count),
    )
}

fn dirty_seed_denial(kind: TopologySeedKind) -> Option<DirtyTopologySeedDenial> {
    match kind {
        TopologySeedKind::SelfIntersectingLoop => Some(DirtyTopologySeedDenial {
            reason_code: TopologySeedCleanFailReasonCode::SelfIntersectingLoopRequiresSpatialPolicy,
            reason:
                "self-intersecting loop seeds are intentionally dirty and must stop before spatial binding chooses an ambiguity policy",
        }),
        TopologySeedKind::NonManifoldWire => Some(DirtyTopologySeedDenial {
            reason_code: TopologySeedCleanFailReasonCode::NonManifoldWireCannotBindAsGeometry,
            reason:
                "non-manifold wire seeds are intentionally dirty and must stop before spatial binding can consume topology",
        }),
        TopologySeedKind::ThinWallLocalBasis => Some(DirtyTopologySeedDenial {
            reason_code: TopologySeedCleanFailReasonCode::ThinWallLocalBasisCannotBindAsGeometry,
            reason:
                "thin wall local-basis seeds are intentionally dirty and must stop before spatial binding can infer a stable planar frame",
        }),
        TopologySeedKind::OrientationInconsistency => Some(DirtyTopologySeedDenial {
            reason_code: TopologySeedCleanFailReasonCode::OrientationInconsistencyRequiresRepairPolicy,
            reason:
                "orientation-inconsistent seeds are intentionally dirty and must stop before spatial binding can choose a repair policy",
        }),
        _ => None,
    }
}

struct DirtyTopologySeedDenial {
    reason_code: TopologySeedCleanFailReasonCode,
    reason: &'static str,
}

impl From<TopologySeedRecipeOutput> for TopologyView {
    fn from(output: TopologySeedRecipeOutput) -> Self {
        output.topology
    }
}
