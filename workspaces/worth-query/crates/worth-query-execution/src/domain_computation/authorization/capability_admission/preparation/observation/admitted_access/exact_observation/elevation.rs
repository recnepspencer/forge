//! Atomic elevation upper-bound and current-support observation.

use std::sync::Arc;

use worth_query_declaration::facade::application_capability::ApplicationCapabilityElevationRequestProjection;
use worth_query_installation::facade::ApplicationSchema;

use super::WorthQueryExactCapabilityObservation;
use crate::domain_computation::authorization::capability_observation::WorthQueryObservedCapabilityDecision;
use crate::domain_computation::authorization::capability_registry::WorthQueryInstalledCapabilityPlan;
use crate::domain_computation::authorization::delegation_admission::observe_elevation_upper_bound;
use crate::domain_computation::authorization::elevation_progression::WorthQueryElevationUpperBound;
use crate::domain_computation::authorization::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use crate::domain_computation::authorization::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryRetainedCapabilitySupport, WorthQueryRuntimeTimeSample,
};

impl<Schema> WorthQueryExactCapabilityObservation<'_, Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation::authorization) fn resolve_elevation_identity(
        &self,
        capability_identity: [u8; 32],
        installed: &WorthQueryInstalledCapabilityPlan,
        value: worth_foundational::facade::AspectValue,
    ) -> Result<worth_relational::facade::identity::EntityId, WorthQueryOperationAuthorizationDenial>
    {
        self.validate_plan(capability_identity, installed)?;
        let field = installed
            .contract
            .elevation()
            .definition()
            .ok_or_else(|| rejected(installed))?
            .identity();
        self.resolve_lifecycle_field(field, value)
            .map(|resolved| resolved.entity_id())
    }

    pub(in crate::domain_computation::authorization) fn resolve_review_identity(
        &self,
        capability_identity: [u8; 32],
        installed: &WorthQueryInstalledCapabilityPlan,
        value: worth_foundational::facade::AspectValue,
    ) -> Result<worth_relational::facade::identity::EntityId, WorthQueryOperationAuthorizationDenial>
    {
        self.validate_plan(capability_identity, installed)?;
        let field = installed
            .contract
            .elevation()
            .definition()
            .ok_or_else(|| rejected(installed))?
            .review()
            .identity();
        self.resolve_lifecycle_field(field, value)
            .map(|resolved| resolved.entity_id())
    }

    pub(in crate::domain_computation::authorization) fn resolve_elevation_upper_bound<
        Scope,
        Context,
    >(
        &self,
        capability_identity: [u8; 32],
        installed: &WorthQueryInstalledCapabilityPlan,
        proposed: &ApplicationCapabilityElevationRequestProjection<Schema, Scope, Context>,
        sample: &WorthQueryRuntimeTimeSample,
    ) -> Result<
        (
            WorthQueryElevationUpperBound,
            WorthQueryRetainedCapabilitySupport,
        ),
        WorthQueryOperationAuthorizationDenial,
    > {
        self.validate_plan(capability_identity, installed)?;
        let target = self.resolve_request(proposed.target())?;
        let grant = self.resolve_selector(proposed.grant())?;
        if grant.entity_kind() != installed.grant_kind {
            return Err(rejected(installed));
        }
        let retained = WorthQueryRetainedCapabilityRequest::capture(
            capability_identity,
            self.principal,
            proposed.target(),
            &target,
        );
        let observed = observe_elevation_upper_bound(
            self.session,
            self.relational,
            self.snapshot.clone(),
            self.runtime.authorization.bridge(),
            installed,
            &retained,
            sample,
            grant.entity_id(),
            None,
        )?;
        let (decision, observed_grant) = observed.into_parts();
        if observed_grant != grant.entity_id() {
            return Err(rejected(installed));
        }
        Ok((
            WorthQueryElevationUpperBound::capture(
                capability_identity,
                self.principal,
                proposed.target(),
                &target,
                grant.entity_id(),
            ),
            WorthQueryRetainedCapabilitySupport::elevation_upper_bound(
                decision,
                Arc::clone(&installed.capability_authority_identity),
                grant.entity_id(),
                retained,
                sample.clone(),
            ),
        ))
    }

    pub(in crate::domain_computation::authorization) fn observe_current_elevation_support(
        &self,
        installed: &WorthQueryInstalledCapabilityPlan,
        supporting: &WorthQueryRetainedCapabilitySupport,
    ) -> Result<
        (
            WorthQueryRuntimeTimeSample,
            WorthQueryObservedCapabilityDecision,
        ),
        WorthQueryOperationAuthorizationDenial,
    > {
        self.validate_plan(supporting.request().capability_identity, installed)?;
        let sample = self.runtime.sample_capability_time(installed)?;
        let expected = supporting.decision();
        if !expected.remains_current_in(
            self.relational,
            self.snapshot,
            self.runtime.authorization.bridge(),
        ) {
            return Err(stale(installed.contract.name()));
        }
        let observed = observe_elevation_upper_bound(
            self.session,
            self.relational,
            self.snapshot.clone(),
            self.runtime.authorization.bridge(),
            installed,
            supporting.request(),
            &sample,
            supporting.grant(),
            Some(expected),
        )?;
        Ok((sample, observed))
    }

    fn validate_plan(
        &self,
        capability_identity: [u8; 32],
        installed: &WorthQueryInstalledCapabilityPlan,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let owned = self
            .runtime
            .authorization
            .capability_plan_by_identity(&capability_identity)
            .ok_or_else(|| stale(installed.contract.name()))?;
        std::ptr::eq(owned, installed)
            .then_some(())
            .ok_or_else(|| stale(installed.contract.name()))
    }
}

fn rejected(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::ElevationProjectionRejected,
        installed.contract.name(),
    )
}

fn stale(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
        subject,
    )
}
