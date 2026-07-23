use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryNativeAccessDenial, WorthQueryNativeAccessKey, WorthQueryNativeAccessLayout,
    WorthQueryNativeFieldAccess,
};
use crate::ordinary::read::WorthQueryProjectionOutcome;
use crate::projection_consumption::{
    ProjectionConsumptionWarnings, WorthQueryConsumedProjectionAuthority,
};
use crate::runtime::{WorthQueryRuntimeError, WorthQueryWorkspace};

use super::refresh_work::WorthQueryLiveProjectionRefreshWork;
use super::source::WorthQueryProjectionLifecycleSource;
use super::{
    WorthQueryLiveBoundDomainProjection, WorthQueryLiveBoundWorkflowProjection,
    WorthQueryProjectionPromotionDenialKind,
};

pub struct WorthQueryLiveProjectionRefresh {
    authority: Box<WorthQueryConsumedProjectionAuthority>,
    warnings: Option<ProjectionConsumptionWarnings>,
    native_access: Option<WorthQueryNativeAccessLayout>,
    delivery: crate::ordinary::live::WorthQueryManagedLiveDelivery,
    work: WorthQueryLiveProjectionRefreshWork,
    impact: std::sync::Arc<crate::domain_installation::WorthQueryImpactDecision>,
}

impl WorthQueryLiveProjectionRefresh {
    pub fn authority(&self) -> &WorthQueryConsumedProjectionAuthority {
        &self.authority
    }

    pub fn warnings(&self) -> Option<&ProjectionConsumptionWarnings> {
        self.warnings.as_ref()
    }

    pub fn delivery(&self) -> &crate::ordinary::live::WorthQueryManagedLiveDelivery {
        &self.delivery
    }

    pub fn work(&self) -> WorthQueryLiveProjectionRefreshWork {
        self.work
    }

    pub fn impact(&self) -> &crate::domain_installation::WorthQueryImpactDecision {
        &self.impact
    }

    pub fn native_value<'a>(
        &'a self,
        key: &WorthQueryNativeAccessKey,
        row: usize,
    ) -> Result<WorthQueryNativeFieldAccess<'a>, WorthQueryNativeAccessDenial> {
        let Some(layout) = &self.native_access else {
            return Err(WorthQueryNativeAccessLayout::unbound_denial(
                &self.authority,
                key,
            ));
        };
        layout.access(&self.authority, key, row)
    }
}

#[derive(Debug)]
pub enum WorthQueryLiveProjectionRefreshError {
    Impact {
        denial: crate::domain_installation::WorthQueryImpactAdmissionDenial,
        work: WorthQueryLiveProjectionRefreshWork,
        owner_delivery_retained: bool,
    },
    Authority(WorthQueryLiveProjectionRefreshAuthorityStop),
    Runtime {
        error: WorthQueryRuntimeError,
        work: WorthQueryLiveProjectionRefreshWork,
        owner_delivery_retained: bool,
    },
    Conditional {
        kind: WorthQueryProjectionPromotionDenialKind,
        detail: String,
        work: WorthQueryLiveProjectionRefreshWork,
        owner_delivery_retained: bool,
    },
    Projection {
        outcome: Box<WorthQueryProjectionOutcome>,
        work: WorthQueryLiveProjectionRefreshWork,
    },
    NativeAccess {
        denial: WorthQueryNativeAccessDenial,
        work: WorthQueryLiveProjectionRefreshWork,
    },
}

impl WorthQueryLiveProjectionRefreshError {
    pub fn work(&self) -> WorthQueryLiveProjectionRefreshWork {
        match self {
            Self::Impact { work, .. } => *work,
            Self::Authority(stop) => stop.work(),
            Self::Runtime { work, .. }
            | Self::Conditional { work, .. }
            | Self::Projection { work, .. }
            | Self::NativeAccess { work, .. } => *work,
        }
    }

    /// Reports whether exact staged owner evidence remains available for a
    /// corrected retry after this stop.
    pub fn owner_delivery_retained(&self) -> bool {
        match self {
            Self::Impact {
                owner_delivery_retained,
                ..
            }
            | Self::Runtime {
                owner_delivery_retained,
                ..
            }
            | Self::Conditional {
                owner_delivery_retained,
                ..
            } => *owner_delivery_retained,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveProjectionRefreshAuthorityStop {
    kind: crate::domain_installation::WorthQueryDomainHandleDenialKind,
    work: WorthQueryLiveProjectionRefreshWork,
}

impl WorthQueryLiveProjectionRefreshAuthorityStop {
    pub fn kind(&self) -> crate::domain_installation::WorthQueryDomainHandleDenialKind {
        self.kind
    }

    pub fn work(&self) -> WorthQueryLiveProjectionRefreshWork {
        self.work
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    WorthQueryLiveBoundDomainProjection<D, O, F, L>
{
    pub fn refresh(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryLiveProjectionRefresh, WorthQueryLiveProjectionRefreshError> {
        refresh_source(self.snapshot(), self.managed_handle(), workspace, None)
    }

    pub fn refresh_owner_delivery(
        &self,
        delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryLiveProjectionRefresh, WorthQueryLiveProjectionRefreshError> {
        refresh_source(
            self.snapshot(),
            self.managed_handle(),
            workspace,
            Some(delivery),
        )
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    WorthQueryLiveBoundWorkflowProjection<D, O, F, L>
{
    pub fn refresh(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryLiveProjectionRefresh, WorthQueryLiveProjectionRefreshError> {
        refresh_source(self.snapshot(), self.managed_handle(), workspace, None)
    }

    pub fn refresh_owner_delivery(
        &self,
        delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryLiveProjectionRefresh, WorthQueryLiveProjectionRefreshError> {
        refresh_source(
            self.snapshot(),
            self.managed_handle(),
            workspace,
            Some(delivery),
        )
    }
}

pub(in crate::domain_installation::operation_execution) struct WorthQueryPendingOwnerImpact<'a> {
    pub(super) delivery: &'a worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    pub(super) closure:
        &'a crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
}

impl<'a> WorthQueryPendingOwnerImpact<'a> {
    pub(in crate::domain_installation::operation_execution) const fn new(
        delivery: &'a worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        closure: &'a crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
    ) -> Self {
        Self { delivery, closure }
    }
}

pub(super) fn refresh_source<D: 'static, O: 'static, F: 'static, L: BasisOperationLane, S>(
    source: &S,
    handle: &crate::ordinary::live::WorthQueryManagedLiveHandle,
    workspace: &mut WorthQueryWorkspace,
    pending_owner_delivery: Option<
        &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    >,
) -> Result<WorthQueryLiveProjectionRefresh, WorthQueryLiveProjectionRefreshError>
where
    S: WorthQueryProjectionLifecycleSource<D, O, F, L>,
{
    let mut work = WorthQueryLiveProjectionRefreshWork::authority_checked();
    super::source::validate_live_source_authority(source, workspace).map_err(|denial| {
        WorthQueryLiveProjectionRefreshError::Authority(
            WorthQueryLiveProjectionRefreshAuthorityStop {
                kind: denial.kind(),
                work,
            },
        )
    })?;
    let closure = source.semantic_dependency_closure().ok_or_else(||
        crate::domain_installation::WorthQueryImpactAdmissionDenial::new(
            crate::domain_installation::WorthQueryImpactAdmissionDenialKind::DependencyClosureUnavailable,
            crate::domain_installation::WorthQueryImpactCounters::default(),
        )
    ).map_err(|denial| WorthQueryLiveProjectionRefreshError::Impact {
        denial,
        work,
        owner_delivery_retained: false,
    })?;
    let (delivery, impact) = match pending_owner_delivery {
        Some(owner_delivery) => {
            let completion = super::owner_refresh::refresh_owner_delivery::<D, O, F, L, S>(
                source,
                handle,
                workspace,
                WorthQueryPendingOwnerImpact::new(owner_delivery, closure),
            )?;
            work = completion.work();
            let (delivery, impact, _, _, _) = completion.into_parts();
            (delivery, impact)
        }
        None => {
            work.begin_drain();
            let delivery = handle.drain(workspace).map_err(|error| {
                WorthQueryLiveProjectionRefreshError::Runtime {
                    error,
                    work,
                    owner_delivery_retained: false,
                }
            })?;
            work.retain_delivery(&delivery);
            let impact = std::sync::Arc::new(
                crate::domain_installation::WorthQueryImpactDecision::from_managed_live_delivery(
                    closure, &delivery,
                ),
            );
            work.retain_impact(&impact);
            (delivery, impact)
        }
    };
    work.begin_read();
    let read =
        handle
            .read(workspace)
            .map_err(|error| WorthQueryLiveProjectionRefreshError::Runtime {
                error,
                work,
                owner_delivery_retained: false,
            })?;
    let outcome = handle.project_contract(&read, source.projection_authority_contract());
    work.retain_projection();
    let (authority, warnings) = outcome.into_admitted().map_err(|outcome| {
        WorthQueryLiveProjectionRefreshError::Projection {
            outcome: Box::new(outcome),
            work,
        }
    })?;
    let native_layout = source.native_access_layout();
    if native_layout.is_some() {
        work.begin_native_rebind();
    }
    let native_access = native_layout
        .map(|layout| layout.rebind(source.consumer_contract(), &authority))
        .transpose()
        .map_err(|denial| WorthQueryLiveProjectionRefreshError::NativeAccess { denial, work })?;
    Ok(WorthQueryLiveProjectionRefresh {
        authority,
        warnings,
        native_access,
        delivery,
        work,
        impact,
    })
}
