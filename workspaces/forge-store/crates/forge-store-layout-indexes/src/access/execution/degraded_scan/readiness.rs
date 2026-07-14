use forge_proof::raw::{
    CheckedAdmitExecutionReadyRecipeTransition, ContextualTransition, ExecutionReadinessContext,
    ExecutionReadyAdmissionReadiness, ExecutionReadyRecipe, TransitionOutcome,
};

use super::authority::{
    readiness_authority, DegradedScanReadinessAuthority, DegradedScanReadinessDeferred,
};
use super::{DegradedScanLoweringBasis, LoweredDegradedExactScan, StaleDegradedExactScan};
use crate::planning::SelectedDegradedExactScan;

pub(super) type DegradedScanReadyRecipe = ExecutionReadyRecipe<
    SelectedDegradedExactScan,
    forge_proof::raw::FreshnessScopedBasis<
        forge_proof::raw::CurrentValidity,
        forge_proof::raw::AssumptionBasis<DegradedScanLoweringBasis>,
    >,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct DegradedScanReady {
    recipe: DegradedScanReadyRecipe,
    current_materialization: crate::CurrentLayoutMaterialization,
    rebind_trace: Option<super::DegradedScanRebindTrace>,
}

impl DegradedScanReady {
    fn issue(
        lowered: LoweredDegradedExactScan,
        current_materialization: crate::CurrentLayoutMaterialization,
    ) -> Self {
        Self::issue_with_rebind(lowered, current_materialization, None)
    }

    fn issue_with_rebind(
        lowered: LoweredDegradedExactScan,
        current_materialization: crate::CurrentLayoutMaterialization,
        rebind_trace: Option<super::DegradedScanRebindTrace>,
    ) -> Self {
        let outcome = CheckedAdmitExecutionReadyRecipeTransition.transition(
            lowered.into_recipe(),
            ExecutionReadyAdmissionReadiness::<
                SelectedDegradedExactScan,
                DegradedScanLoweringBasis,
                &'static str,
                DegradedScanReadinessAuthority,
                DegradedScanReadinessDeferred,
                DegradedScanReadinessDeferred,
                DegradedScanReadinessDeferred,
            >::ready(ExecutionReadinessContext::new(
                "degraded-scan-ready",
                readiness_authority(),
            )),
        );
        match outcome {
            TransitionOutcome::Success(recipe) => Self {
                recipe,
                current_materialization,
                rebind_trace,
            },
            _ => unreachable!("degraded readiness is issued only after owner classification"),
        }
    }

    fn from_rebind(
        lowered: LoweredDegradedExactScan,
        current_materialization: crate::CurrentLayoutMaterialization,
        rebind_trace: super::DegradedScanRebindTrace,
    ) -> Self {
        Self::issue_with_rebind(lowered, current_materialization, Some(rebind_trace))
    }

    pub fn selected(&self) -> &SelectedDegradedExactScan {
        self.recipe.payload()
    }
    pub fn basis(&self) -> &DegradedScanLoweringBasis {
        self.recipe.strong_basis().value()
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
    pub const fn rebind_trace(&self) -> Option<&super::DegradedScanRebindTrace> {
        self.rebind_trace.as_ref()
    }
    pub(super) fn into_parts(
        self,
    ) -> (DegradedScanReadyRecipe, crate::CurrentLayoutMaterialization) {
        (self.recipe, self.current_materialization)
    }
}

pub(in crate::access::execution::degraded_scan) fn admit_rebound_ready(
    lowered: LoweredDegradedExactScan,
    admission: super::DegradedScanRebindAdmission,
) -> DegradedScanReady {
    let (current_materialization, rebind_trace) = admission.into_ready_basis();
    DegradedScanReady::from_rebind(lowered, current_materialization, rebind_trace)
}

macro_rules! define_degraded_readiness_cases {
    ($( $variant:ident($payload:ty) => $name:literal ),+ $(,)?) => {
        #[derive(Debug, PartialEq, Eq)]
        enum DegradedScanReadinessCase {
            $( $variant($payload), )+
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct DegradedScanReadinessCaseId(&'static str);

        impl DegradedScanReadinessCaseId {
            pub const fn name(self) -> &'static str {
                self.0
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DegradedScanReadinessView<'a> {
            $( $variant(&'a $payload), )+
        }

        impl DegradedScanReadinessCase {
            const fn id(&self) -> DegradedScanReadinessCaseId {
                match self {
                    $( Self::$variant(_) => DegradedScanReadinessCaseId($name), )+
                }
            }

            const fn view(&self) -> DegradedScanReadinessView<'_> {
                match self {
                    $( Self::$variant(value) => DegradedScanReadinessView::$variant(value), )+
                }
            }
        }

        pub fn degraded_scan_readiness_cases(
        ) -> impl Iterator<Item = DegradedScanReadinessCaseId> {
            [$( DegradedScanReadinessCaseId($name), )+].into_iter()
        }
    };
}

define_degraded_readiness_cases!(
    Ready(DegradedScanReady) => "layout.degraded_scan.readiness.ready",
    Stale(StaleDegradedExactScan) => "layout.degraded_scan.readiness.stale",
);

#[derive(Debug, PartialEq, Eq)]
pub struct DegradedScanReadinessOutcome {
    case: DegradedScanReadinessCase,
}

impl DegradedScanReadinessOutcome {
    fn ready(value: DegradedScanReady) -> Self {
        Self {
            case: DegradedScanReadinessCase::Ready(value),
        }
    }
    fn stale(value: StaleDegradedExactScan) -> Self {
        Self {
            case: DegradedScanReadinessCase::Stale(value),
        }
    }
    pub fn view(&self) -> DegradedScanReadinessView<'_> {
        self.case.view()
    }
    pub const fn case_id(&self) -> DegradedScanReadinessCaseId {
        self.case.id()
    }
    pub fn into_ready(self) -> Result<DegradedScanReady, Self> {
        match self.case {
            DegradedScanReadinessCase::Ready(value) => Ok(value),
            case => Err(Self { case }),
        }
    }

    pub fn into_stale(self) -> Result<StaleDegradedExactScan, Self> {
        match self.case {
            DegradedScanReadinessCase::Stale(value) => Ok(value),
            case => Err(Self { case }),
        }
    }
}

pub(in crate::access::execution::degraded_scan) fn classify_readiness(
    lowered: LoweredDegradedExactScan,
    frontier: crate::CurrentMaterializationFrontier,
) -> DegradedScanReadinessOutcome {
    let materialization = lowered.selected().materialization().clone();
    match materialization.classify_freshness_at(frontier) {
        Ok(crate::MaterializationFreshness::Current(current)) => {
            DegradedScanReadinessOutcome::ready(DegradedScanReady::issue(lowered, current))
        }
        Ok(crate::MaterializationFreshness::Stale(stale)) => {
            DegradedScanReadinessOutcome::stale(lowered.stale(stale))
        }
        Err(_) => unreachable!("degraded selection retains exact admitted materialization"),
    }
}
