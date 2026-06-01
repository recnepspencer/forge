use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeContinuationFamily,
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeTruthContext,
    ForgeQueryDeclarationEnvelopeClass, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationReceiptClass, ForgeQueryDeclarationRelationalAuthorityFamily,
    ForgeQueryDeclarationRelationalTruthClaim, ForgeQueryDeclarationSignalExecutionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryLowerAuthorityRouteFamily, ForgeQuerySignalCompatibilityPosture,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    classification::{
        ForgeQueryDeclarationEntryLowerOwnerCrate, ForgeQueryDeclarationEntrySeamClassification,
    },
    digest::derive_crossing_row_digest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryCrossingSurface {
    Envelope,
    RelationalTruthRouting,
    BridgeContinuationRouting,
    SignalCompatibility,
}

impl ForgeQueryDeclarationEntryCrossingSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Envelope => "envelope",
            Self::RelationalTruthRouting => "relational_truth_routing",
            Self::BridgeContinuationRouting => "bridge_continuation_routing",
            Self::SignalCompatibility => "signal_compatibility",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryCrossingRow {
    entrypoint_key: &'static str,
    surface: ForgeQueryDeclarationEntryCrossingSurface,
    declaration_family_key: &'static str,
    primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
    lower_owner_crate: ForgeQueryDeclarationEntryLowerOwnerCrate,
    route_family: Option<ForgeQueryLowerAuthorityRouteFamily>,
    receipt_class: Option<ForgeQueryDeclarationReceiptClass>,
    envelope_class: Option<ForgeQueryDeclarationEnvelopeClass>,
    relational_truth_claim: Option<ForgeQueryDeclarationRelationalTruthClaim>,
    relational_authority_family: Option<ForgeQueryDeclarationRelationalAuthorityFamily>,
    bridge_continuation_mode: Option<ForgeQueryDeclarationBridgeContinuationMode>,
    bridge_truth_context: Option<ForgeQueryDeclarationBridgeTruthContext>,
    bridge_continuation_family: Option<ForgeQueryDeclarationBridgeContinuationFamily>,
    signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
    basis_families: Vec<BasisFamily>,
    seam_classification: ForgeQueryDeclarationEntrySeamClassification,
    row_digest: String,
}

impl ForgeQueryDeclarationEntryCrossingRow {
    #[allow(clippy::too_many_arguments)]
    fn new(
        entrypoint_key: &'static str,
        surface: ForgeQueryDeclarationEntryCrossingSurface,
        declaration_family_key: &'static str,
        primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
        lower_owner_crate: ForgeQueryDeclarationEntryLowerOwnerCrate,
        route_family: Option<ForgeQueryLowerAuthorityRouteFamily>,
        receipt_class: Option<ForgeQueryDeclarationReceiptClass>,
        envelope_class: Option<ForgeQueryDeclarationEnvelopeClass>,
        relational_truth_claim: Option<ForgeQueryDeclarationRelationalTruthClaim>,
        relational_authority_family: Option<ForgeQueryDeclarationRelationalAuthorityFamily>,
        bridge_continuation_mode: Option<ForgeQueryDeclarationBridgeContinuationMode>,
        bridge_truth_context: Option<ForgeQueryDeclarationBridgeTruthContext>,
        bridge_continuation_family: Option<ForgeQueryDeclarationBridgeContinuationFamily>,
        signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
        basis_families: Vec<BasisFamily>,
        seam_classification: ForgeQueryDeclarationEntrySeamClassification,
        handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> Self {
        let row_digest = derive_crossing_row_digest(
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_family_key,
            entrypoint_key,
            surface,
            primary_authority_family,
            lower_owner_crate,
            route_family,
            receipt_class,
            envelope_class,
            relational_truth_claim,
            relational_authority_family,
            bridge_continuation_mode,
            bridge_truth_context,
            bridge_continuation_family,
            signal_execution_family,
            &basis_families,
            seam_classification,
        );
        Self {
            entrypoint_key,
            surface,
            declaration_family_key,
            primary_authority_family,
            lower_owner_crate,
            route_family,
            receipt_class,
            envelope_class,
            relational_truth_claim,
            relational_authority_family,
            bridge_continuation_mode,
            bridge_truth_context,
            bridge_continuation_family,
            signal_execution_family,
            basis_families,
            seam_classification,
            row_digest,
        }
    }

    pub fn entrypoint_key(&self) -> &'static str {
        self.entrypoint_key
    }
    pub fn surface(&self) -> ForgeQueryDeclarationEntryCrossingSurface {
        self.surface
    }
    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }
    pub fn primary_authority_family(&self) -> ForgeQueryDeclarationPrimaryAuthorityFamily {
        self.primary_authority_family
    }
    pub fn lower_owner_crate(&self) -> ForgeQueryDeclarationEntryLowerOwnerCrate {
        self.lower_owner_crate
    }
    pub fn route_family(&self) -> Option<ForgeQueryLowerAuthorityRouteFamily> {
        self.route_family
    }
    pub fn receipt_class(&self) -> Option<ForgeQueryDeclarationReceiptClass> {
        self.receipt_class
    }
    pub fn envelope_class(&self) -> Option<ForgeQueryDeclarationEnvelopeClass> {
        self.envelope_class
    }
    pub fn relational_truth_claim(&self) -> Option<ForgeQueryDeclarationRelationalTruthClaim> {
        self.relational_truth_claim
    }
    pub fn relational_authority_family(
        &self,
    ) -> Option<ForgeQueryDeclarationRelationalAuthorityFamily> {
        self.relational_authority_family
    }
    pub fn bridge_continuation_mode(&self) -> Option<ForgeQueryDeclarationBridgeContinuationMode> {
        self.bridge_continuation_mode
    }
    pub fn bridge_truth_context(&self) -> Option<ForgeQueryDeclarationBridgeTruthContext> {
        self.bridge_truth_context
    }
    pub fn bridge_continuation_family(
        &self,
    ) -> Option<ForgeQueryDeclarationBridgeContinuationFamily> {
        self.bridge_continuation_family
    }
    pub fn signal_execution_family(&self) -> Option<ForgeQueryDeclarationSignalExecutionFamily> {
        self.signal_execution_family
    }
    pub fn basis_families(&self) -> &[BasisFamily] {
        &self.basis_families
    }
    pub fn seam_classification(&self) -> ForgeQueryDeclarationEntrySeamClassification {
        self.seam_classification
    }
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[rustfmt::skip]
const ENVELOPE_ENTRYPOINTS: &[&str] = &["envelope-routes-checked","envelope-routes","envelope-routes-from-progressed","envelope-routes-from-progressed-with-intent","declare-review-progress-describe-plan-receipt-and-envelope"];
#[rustfmt::skip]
const RELATIONAL_DIRECT: &[&str] = &["route-relational-truth-checked","route-relational-truth"];
#[rustfmt::skip]
const RELATIONAL_HIGHER: &[&str] = &["route-relational-truth-from-progressed","declare-review-progress-describe-plan-receipt-envelope-and-route-relational-truth"];
#[rustfmt::skip]
const BRIDGE_DIRECT: &[&str] = &["route-bridge-continuation-checked","route-bridge-continuation"];
#[rustfmt::skip]
const BRIDGE_HIGHER: &[&str] = &["route-bridge-continuation-from-progressed","declare-review-progress-describe-plan-receipt-envelope-and-route-bridge-continuation"];
#[rustfmt::skip]
const SIGNAL_DIRECT: &[&str] = &["signal-compatibility-checked","signal-compatibility"];
#[rustfmt::skip]
const SIGNAL_HIGHER: &[&str] = &["signal-compatibility-from-progressed","declare-review-progress-describe-plan-receipt-envelope-and-check-signal-compatibility"];

pub(crate) fn crossing_rows_for_family<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> Vec<ForgeQueryDeclarationEntryCrossingRow> {
    let taxonomy = I::Family::taxonomy();
    let mut rows = Vec::new();
    push_rows(
        &mut rows,
        handle,
        ENVELOPE_ENTRYPOINTS,
        ForgeQueryDeclarationEntryCrossingSurface::Envelope,
        I::Family::semantic_family_key(),
        taxonomy.primary_authority_family(),
        ForgeQueryDeclarationEntryLowerOwnerCrate::Query,
        None,
        Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing),
        Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing),
        None,
        None,
        None,
        None,
        None,
        None,
        &[],
        ForgeQueryDeclarationEntrySeamClassification::CanonicalReuse,
    );
    push_relational_rows::<D, C, I>(&mut rows, handle, taxonomy.primary_authority_family());
    push_bridge_rows::<D, C, I>(&mut rows, handle, taxonomy.primary_authority_family());
    push_signal_rows::<D, C, I>(&mut rows, handle, taxonomy.primary_authority_family());
    rows
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
fn push_rows<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>(rows: &mut Vec<ForgeQueryDeclarationEntryCrossingRow>, handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>, entrypoint_keys: &[&'static str], surface: ForgeQueryDeclarationEntryCrossingSurface, declaration_family_key: &'static str, primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily, lower_owner_crate: ForgeQueryDeclarationEntryLowerOwnerCrate, route_family: Option<ForgeQueryLowerAuthorityRouteFamily>, receipt_class: Option<ForgeQueryDeclarationReceiptClass>, envelope_class: Option<ForgeQueryDeclarationEnvelopeClass>, relational_truth_claim: Option<ForgeQueryDeclarationRelationalTruthClaim>, relational_authority_family: Option<ForgeQueryDeclarationRelationalAuthorityFamily>, bridge_continuation_mode: Option<ForgeQueryDeclarationBridgeContinuationMode>, bridge_truth_context: Option<ForgeQueryDeclarationBridgeTruthContext>, bridge_continuation_family: Option<ForgeQueryDeclarationBridgeContinuationFamily>, signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>, basis_families: &[BasisFamily], seam_classification: ForgeQueryDeclarationEntrySeamClassification) {
    for entrypoint_key in entrypoint_keys { rows.push(ForgeQueryDeclarationEntryCrossingRow::new(entrypoint_key, surface, declaration_family_key, primary_authority_family, lower_owner_crate, route_family, receipt_class, envelope_class, relational_truth_claim, relational_authority_family, bridge_continuation_mode, bridge_truth_context, bridge_continuation_family, signal_execution_family, basis_families.to_vec(), seam_classification, handle.handle_identity_digest(), handle.operating_context_identity_digest())); }
}

#[rustfmt::skip]
fn push_relational_rows<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>, I: ForgeQueryDeclarationInput<D>>(rows: &mut Vec<ForgeQueryDeclarationEntryCrossingRow>, handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>, primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily) {
    let (classification, direct, higher, claim, authority) = match I::Family::relational_truth_contract() {
        Some(contract) => (ForgeQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter, RELATIONAL_DIRECT, Some(RELATIONAL_HIGHER), contract.truth_claim(), contract.authority_family()),
        None => (ForgeQueryDeclarationEntrySeamClassification::ForbiddenDuplicate, RELATIONAL_DIRECT, None, ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth, ForgeQueryDeclarationRelationalAuthorityFamily::Runtime),
    };
    push_rows(rows, handle, direct, ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeRelational, Some(ForgeQueryLowerAuthorityRouteFamily::Relational), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), Some(claim), Some(authority), None, None, None, None, &[], classification);
    if let Some(higher) = higher { push_rows(rows, handle, higher, ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeRelational, Some(ForgeQueryLowerAuthorityRouteFamily::Relational), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), Some(claim), Some(authority), None, None, None, None, &[], classification); }
}

#[rustfmt::skip]
fn push_bridge_rows<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>, I: ForgeQueryDeclarationInput<D>>(rows: &mut Vec<ForgeQueryDeclarationEntryCrossingRow>, handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>, primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily) {
    let (classification, direct, higher, mode, truth_context, family) = match I::Family::bridge_continuation_contract() {
        Some(contract) => (ForgeQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter, BRIDGE_DIRECT, Some(BRIDGE_HIGHER), contract.request().mode(), contract.request().truth_context(), contract.family()),
        None => (ForgeQueryDeclarationEntrySeamClassification::ForbiddenDuplicate, BRIDGE_DIRECT, None, ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute, ForgeQueryDeclarationBridgeTruthContext::Current, ForgeQueryDeclarationBridgeContinuationFamily::RuntimeRoute),
    };
    push_rows(rows, handle, direct, ForgeQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeRuntimeBridge, Some(ForgeQueryLowerAuthorityRouteFamily::Bridge), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, Some(mode), Some(truth_context), Some(family), None, &[], classification);
    if let Some(higher) = higher { push_rows(rows, handle, higher, ForgeQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeRuntimeBridge, Some(ForgeQueryLowerAuthorityRouteFamily::Bridge), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, Some(mode), Some(truth_context), Some(family), None, &[], classification); }
}

#[rustfmt::skip]
fn push_signal_rows<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>, I: ForgeQueryDeclarationInput<D>>(rows: &mut Vec<ForgeQueryDeclarationEntryCrossingRow>, handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>, primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily) {
    match I::Family::taxonomy().signal_compatibility() {
        ForgeQuerySignalCompatibilityPosture::Compatible => if let Some(contract) = I::Family::signal_compatibility_contract() {
            let execution_family = if primary_authority_family == ForgeQueryDeclarationPrimaryAuthorityFamily::MixedAuthority { ForgeQueryDeclarationSignalExecutionFamily::MixedDerivedExecution } else { contract.execution_family() };
            for basis_family in contract.required_basis_families() {
                push_rows(rows, handle, SIGNAL_DIRECT, ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeSignal, Some(ForgeQueryLowerAuthorityRouteFamily::Signal), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(execution_family), &[*basis_family], ForgeQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter);
                push_rows(rows, handle, SIGNAL_HIGHER, ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeSignal, Some(ForgeQueryLowerAuthorityRouteFamily::Signal), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(execution_family), &[*basis_family], ForgeQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter);
            }
        },
        ForgeQuerySignalCompatibilityPosture::Deferred => push_rows(rows, handle, SIGNAL_DIRECT, ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeSignal, Some(ForgeQueryLowerAuthorityRouteFamily::Signal), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution), &[BasisFamily::CurrentHead], ForgeQueryDeclarationEntrySeamClassification::DeferredNeighbor),
        ForgeQuerySignalCompatibilityPosture::NotCompatible => push_rows(rows, handle, SIGNAL_DIRECT, ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, ForgeQueryDeclarationEntryLowerOwnerCrate::ForgeSignal, Some(ForgeQueryLowerAuthorityRouteFamily::Signal), Some(ForgeQueryDeclarationReceiptClass::CoveredCrossing), Some(ForgeQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution), &[BasisFamily::CurrentHead], ForgeQueryDeclarationEntrySeamClassification::ForbiddenDuplicate),
    }
}
