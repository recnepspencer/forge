use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCapabilityGeneration, WorthQueryNativeAccessDenial, WorthQueryNativeAccessKey,
    WorthQueryNativeFieldAccess, WorthQueryOperationContinuationPosture,
    WorthQueryOperationWindowPolicy, WorthQuerySettledDomainProjection,
};
use worth_proof::TransitionOutcome;

use super::{
    WorthQueryAdmittedCollectionWindow, WorthQueryBoundCollectionWindow,
    WorthQueryCollectionCapabilityCounters, WorthQueryCollectionCapabilityDenial,
    WorthQueryCollectionContinuation, WorthQueryCollectionCursor, WorthQueryCollectionCursorParts,
    WorthQueryCollectionRowHandle, WorthQueryCollectionWindowAdmissionOutcome,
    WorthQueryCollectionWindowBreadth, WorthQueryCollectionWindowCounters,
    WorthQueryCollectionWindowDenial, WorthQueryCollectionWindowDenialKind,
    WorthQueryCollectionWindowOutcome, WorthQueryCollectionWindowParts,
    WorthQueryCollectionWindowWarning,
};

mod binding;
pub(crate) use binding::{prepare_collection_binding, WorthQueryPreparedCollectionBinding};

pub type WorthQueryCollectionCapabilityOutcome<D, O, F, L> = TransitionOutcome<
    WorthQueryBoundCollection<D, O, F, L>,
    WorthQueryCollectionCapabilityStop<D, O, F, L>,
    std::convert::Infallible,
    WorthQueryCollectionCapabilityStop<D, O, F, L>,
>;

#[must_use = "the stopped collection admission retains the exact settled projection"]
pub struct WorthQueryCollectionCapabilityStop<D, O, F, L: BasisOperationLane> {
    projection: WorthQuerySettledDomainProjection<D, O, F, L>,
    denial: WorthQueryCollectionCapabilityDenial,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCollectionCapabilityStop<D, O, F, L> {
    pub(crate) fn new(
        projection: WorthQuerySettledDomainProjection<D, O, F, L>,
        denial: WorthQueryCollectionCapabilityDenial,
    ) -> Self {
        Self { projection, denial }
    }

    pub const fn denial(&self) -> &WorthQueryCollectionCapabilityDenial {
        &self.denial
    }

    pub fn into_projection(self) -> WorthQuerySettledDomainProjection<D, O, F, L> {
        self.projection
    }
}

impl<D, O, F, L: BasisOperationLane> std::fmt::Debug
    for WorthQueryCollectionCapabilityStop<D, O, F, L>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryCollectionCapabilityStop")
            .field("denial", &self.denial)
            .finish_non_exhaustive()
    }
}

pub struct WorthQueryBoundCollection<D, O, F, L: BasisOperationLane> {
    projection: WorthQuerySettledDomainProjection<D, O, F, L>,
    rows: Vec<WorthQueryCollectionRowHandle>,
    capability_identity: u64,
    capability_generation: WorthQueryBoundCapabilityGeneration,
    basis_identity: String,
    source_identity: String,
    binding_identity: String,
    ordering_identity: String,
    result_shape_identity: String,
    collection_delivery_contract_identity: String,
    window_policy: WorthQueryOperationWindowPolicy,
    continuation_posture: WorthQueryOperationContinuationPosture,
    maintenance_index:
        crate::domain_installation::collection_delivery::WorthQueryCollectionMaintenanceIndex,
    counters: WorthQueryCollectionCapabilityCounters,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundCollection<D, O, F, L> {
    pub fn beginning_cursor(&self) -> WorthQueryCollectionCursor {
        self.cursor_at(0)
    }

    pub fn declare_window(
        &self,
        cursor: WorthQueryCollectionCursor,
        breadth: WorthQueryCollectionWindowBreadth,
    ) -> WorthQueryCollectionWindowAdmissionOutcome {
        let mut counters = WorthQueryCollectionWindowCounters::default();
        if !self.projection.bound_operation().installation_is_current() {
            counters.authority_checks += 1;
            return TransitionOutcome::Stale(WorthQueryCollectionWindowDenial::new(
                WorthQueryCollectionWindowDenialKind::StaleInstallationGeneration,
                counters,
            ));
        }
        counters.authority_checks += 1;
        if cursor.capability_identity != self.capability_identity {
            return admission_denied(
                WorthQueryCollectionWindowDenialKind::ForeignCapability,
                counters,
            );
        }
        counters.cursor_checks += 1;
        if cursor.capability_generation != self.capability_generation {
            return admission_denied(
                WorthQueryCollectionWindowDenialKind::CapabilityGenerationMismatch,
                counters,
            );
        }
        counters.cursor_checks += 1;
        if cursor.basis_identity != self.basis_identity {
            return admission_denied(
                WorthQueryCollectionWindowDenialKind::CursorBasisMismatch,
                counters,
            );
        }
        counters.cursor_checks += 1;
        if cursor.ordering_identity != self.ordering_identity {
            return admission_denied(
                WorthQueryCollectionWindowDenialKind::CursorOrderingMismatch,
                counters,
            );
        }
        counters.breadth_checks += 1;
        TransitionOutcome::Success(WorthQueryAdmittedCollectionWindow::mint(
            cursor, breadth, counters,
        ))
    }

    pub fn native_value<'a>(
        &'a self,
        row: &WorthQueryCollectionRowHandle,
        key: &WorthQueryNativeAccessKey,
    ) -> Result<WorthQueryNativeFieldAccess<'a>, WorthQueryCollectionRowAccessDenial> {
        if row.capability_identity != self.capability_identity
            || row.capability_generation != self.capability_generation
        {
            return Err(WorthQueryCollectionRowAccessDenial::ForeignRowHandle);
        }
        self.projection
            .native_value(key, row.row_ordinal)
            .map_err(WorthQueryCollectionRowAccessDenial::NativeAccess)
    }

    pub fn result_shape_identity(&self) -> &str {
        &self.result_shape_identity
    }

    pub(crate) fn collection_delivery_contract_identity(&self) -> &str {
        &self.collection_delivery_contract_identity
    }

    pub(crate) const fn capability_identity(&self) -> u64 {
        self.capability_identity
    }

    pub(crate) const fn capability_generation(&self) -> WorthQueryBoundCapabilityGeneration {
        self.capability_generation
    }

    pub(crate) fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub(crate) fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub(crate) fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub(crate) fn ordering_identity(&self) -> &str {
        &self.ordering_identity
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub const fn continuation_posture(&self) -> WorthQueryOperationContinuationPosture {
        self.continuation_posture
    }

    pub const fn counters(&self) -> WorthQueryCollectionCapabilityCounters {
        self.counters
    }

    pub(crate) fn into_maintenance_index(
        self,
    ) -> crate::domain_installation::collection_delivery::WorthQueryCollectionMaintenanceIndex {
        self.maintenance_index
    }

    pub fn resolve_window(
        &self,
        admitted: WorthQueryAdmittedCollectionWindow,
    ) -> WorthQueryCollectionWindowOutcome {
        let mut counters = admitted.counters;
        let window_contract_identity = admitted.identity().to_string();
        let basis = admitted.basis();
        let end = match self.admitted_window_end(basis, &mut counters) {
            Ok(end) => end,
            Err(outcome) => return outcome,
        };
        counters.ordered_index_probes += 1;
        let rows = self.rows[basis.start_row_ordinal..end].to_vec();
        counters.rows_visited += rows.len();
        counters.window_rows_materialized += rows.len();
        let continuation = self.continuation_at(end);
        let warnings = self.window_warnings(admitted.breadth);
        TransitionOutcome::Success(WorthQueryBoundCollectionWindow::from_parts(
            WorthQueryCollectionWindowParts {
                capability_identity: self.capability_identity,
                capability_generation: self.capability_generation,
                source_identity: self.source_identity.clone(),
                binding_identity: self.binding_identity.clone(),
                result_shape_identity: self.result_shape_identity.clone(),
                collection_delivery_contract_identity: self
                    .collection_delivery_contract_identity
                    .clone(),
                window_contract_identity,
                basis_identity: self.basis_identity.clone(),
                ordering_identity: self.ordering_identity.clone(),
                admitted_width: basis.admitted_width,
                cursor: admitted.cursor,
                rows,
                continuation,
                result_state: self.projection.result_state(),
                warnings,
                counters,
            },
        ))
    }

    fn admitted_window_end(
        &self,
        basis: &super::admission::WorthQueryCollectionWindowAdmissionBasis,
        counters: &mut WorthQueryCollectionWindowCounters,
    ) -> Result<usize, WorthQueryCollectionWindowOutcome> {
        counters.authority_checks += 1;
        if basis.capability_identity != self.capability_identity
            || basis.capability_generation != self.capability_generation
        {
            return Err(denied(
                WorthQueryCollectionWindowDenialKind::ForeignAdmission,
                *counters,
            ));
        }
        if basis.start_row_ordinal > self.rows.len() {
            return Err(denied(
                WorthQueryCollectionWindowDenialKind::CursorPastCollectionEnd,
                *counters,
            ));
        }
        self.window_end(basis.start_row_ordinal, basis.admitted_width, *counters)
    }

    fn continuation_at(&self, end: usize) -> WorthQueryCollectionContinuation {
        match (end < self.rows.len(), self.continuation_posture) {
            (false, _) => WorthQueryCollectionContinuation::Complete,
            (true, WorthQueryOperationContinuationPosture::SnapshotCursor) => {
                WorthQueryCollectionContinuation::SnapshotMore(self.cursor_at(end))
            }
            (true, WorthQueryOperationContinuationPosture::LiveCursor) => {
                WorthQueryCollectionContinuation::LiveMore(self.cursor_at(end))
            }
            (true, WorthQueryOperationContinuationPosture::NotRequired) => {
                unreachable!("validated complete collections cannot expose continuation")
            }
        }
    }

    fn window_end(
        &self,
        start: usize,
        width: usize,
        counters: WorthQueryCollectionWindowCounters,
    ) -> Result<usize, WorthQueryCollectionWindowOutcome> {
        if self.window_policy == WorthQueryOperationWindowPolicy::CompleteCollection {
            if start != 0 {
                return Err(denied(
                    WorthQueryCollectionWindowDenialKind::CompleteCollectionRequiresBeginning,
                    counters,
                ));
            }
            if self.rows.len() > width {
                return Err(denied(
                    WorthQueryCollectionWindowDenialKind::CompleteCollectionExceedsBreadth,
                    counters,
                ));
            }
            return Ok(self.rows.len());
        }
        Ok(start.saturating_add(width).min(self.rows.len()))
    }

    fn window_warnings(
        &self,
        breadth: WorthQueryCollectionWindowBreadth,
    ) -> Vec<WorthQueryCollectionWindowWarning> {
        let mut warnings = Vec::new();
        if !self.projection.warnings().is_empty() {
            warnings.push(
                WorthQueryCollectionWindowWarning::ExecutionWarningsPresent {
                    count: self.projection.warnings().len(),
                },
            );
        }
        if self.projection.projection_warnings().is_some() {
            warnings.push(WorthQueryCollectionWindowWarning::ProjectionWarningsPresent);
        }
        if breadth.mounting_budget_clamped() {
            warnings.push(WorthQueryCollectionWindowWarning::MountingBudgetClamped);
        }
        warnings
    }

    fn cursor_at(&self, next_row_ordinal: usize) -> WorthQueryCollectionCursor {
        WorthQueryCollectionCursor::mint(WorthQueryCollectionCursorParts {
            capability_identity: self.capability_identity,
            capability_generation: self.capability_generation,
            basis_identity: self.basis_identity.clone(),
            ordering_identity: self.ordering_identity.clone(),
            next_row_ordinal,
        })
    }
}

#[derive(Debug)]
pub enum WorthQueryCollectionRowAccessDenial {
    ForeignRowHandle,
    NativeAccess(WorthQueryNativeAccessDenial),
}

fn denied(
    kind: WorthQueryCollectionWindowDenialKind,
    counters: WorthQueryCollectionWindowCounters,
) -> WorthQueryCollectionWindowOutcome {
    TransitionOutcome::Denied(WorthQueryCollectionWindowDenial::new(kind, counters))
}

fn admission_denied(
    kind: WorthQueryCollectionWindowDenialKind,
    counters: WorthQueryCollectionWindowCounters,
) -> WorthQueryCollectionWindowAdmissionOutcome {
    TransitionOutcome::Denied(WorthQueryCollectionWindowDenial::new(kind, counters))
}
