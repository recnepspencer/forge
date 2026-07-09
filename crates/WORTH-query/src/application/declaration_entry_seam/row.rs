use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationBridgeContinuationFamily,
    WorthQueryDeclarationBridgeContinuationMode, WorthQueryDeclarationBridgeTruthContext,
    WorthQueryDeclarationEnvelopeClass, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDeclarationReceiptClass, WorthQueryDeclarationRelationalAuthorityFamily,
    WorthQueryDeclarationRelationalTruthClaim, WorthQueryDeclarationSignalExecutionFamily,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryLowerAuthorityRouteFamily, WorthQuerySignalCompatibilityPosture,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    classification::{
        WorthQueryDeclarationEntryLowerOwnerCrate, WorthQueryDeclarationEntrySeamClassification,
    },
    digest::derive_crossing_row_digest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryCrossingSurface {
    Envelope,
    RelationalTruthRouting,
    BridgeContinuationRouting,
    SignalCompatibility,
}

impl WorthQueryDeclarationEntryCrossingSurface {
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
pub struct WorthQueryDeclarationEntryCrossingRow {
    entrypoint_key: &'static str,
    surface: WorthQueryDeclarationEntryCrossingSurface,
    declaration_family_key: &'static str,
    primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
    lower_owner_crate: WorthQueryDeclarationEntryLowerOwnerCrate,
    route_family: Option<WorthQueryLowerAuthorityRouteFamily>,
    receipt_class: Option<WorthQueryDeclarationReceiptClass>,
    envelope_class: Option<WorthQueryDeclarationEnvelopeClass>,
    relational_truth_claim: Option<WorthQueryDeclarationRelationalTruthClaim>,
    relational_authority_family: Option<WorthQueryDeclarationRelationalAuthorityFamily>,
    bridge_continuation_mode: Option<WorthQueryDeclarationBridgeContinuationMode>,
    bridge_truth_context: Option<WorthQueryDeclarationBridgeTruthContext>,
    bridge_continuation_family: Option<WorthQueryDeclarationBridgeContinuationFamily>,
    signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
    basis_families: Vec<BasisFamily>,
    seam_classification: WorthQueryDeclarationEntrySeamClassification,
    row_digest: String,
}

impl WorthQueryDeclarationEntryCrossingRow {
    #[allow(clippy::too_many_arguments)]
    fn new(
        entrypoint_key: &'static str,
        surface: WorthQueryDeclarationEntryCrossingSurface,
        declaration_family_key: &'static str,
        primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
        lower_owner_crate: WorthQueryDeclarationEntryLowerOwnerCrate,
        route_family: Option<WorthQueryLowerAuthorityRouteFamily>,
        receipt_class: Option<WorthQueryDeclarationReceiptClass>,
        envelope_class: Option<WorthQueryDeclarationEnvelopeClass>,
        relational_truth_claim: Option<WorthQueryDeclarationRelationalTruthClaim>,
        relational_authority_family: Option<WorthQueryDeclarationRelationalAuthorityFamily>,
        bridge_continuation_mode: Option<WorthQueryDeclarationBridgeContinuationMode>,
        bridge_truth_context: Option<WorthQueryDeclarationBridgeTruthContext>,
        bridge_continuation_family: Option<WorthQueryDeclarationBridgeContinuationFamily>,
        signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
        basis_families: Vec<BasisFamily>,
        seam_classification: WorthQueryDeclarationEntrySeamClassification,
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
    pub fn surface(&self) -> WorthQueryDeclarationEntryCrossingSurface {
        self.surface
    }
    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }
    pub fn primary_authority_family(&self) -> WorthQueryDeclarationPrimaryAuthorityFamily {
        self.primary_authority_family
    }
    pub fn lower_owner_crate(&self) -> WorthQueryDeclarationEntryLowerOwnerCrate {
        self.lower_owner_crate
    }
    pub fn route_family(&self) -> Option<WorthQueryLowerAuthorityRouteFamily> {
        self.route_family
    }
    pub fn receipt_class(&self) -> Option<WorthQueryDeclarationReceiptClass> {
        self.receipt_class
    }
    pub fn envelope_class(&self) -> Option<WorthQueryDeclarationEnvelopeClass> {
        self.envelope_class
    }
    pub fn relational_truth_claim(&self) -> Option<WorthQueryDeclarationRelationalTruthClaim> {
        self.relational_truth_claim
    }
    pub fn relational_authority_family(
        &self,
    ) -> Option<WorthQueryDeclarationRelationalAuthorityFamily> {
        self.relational_authority_family
    }
    pub fn bridge_continuation_mode(&self) -> Option<WorthQueryDeclarationBridgeContinuationMode> {
        self.bridge_continuation_mode
    }
    pub fn bridge_truth_context(&self) -> Option<WorthQueryDeclarationBridgeTruthContext> {
        self.bridge_truth_context
    }
    pub fn bridge_continuation_family(
        &self,
    ) -> Option<WorthQueryDeclarationBridgeContinuationFamily> {
        self.bridge_continuation_family
    }
    pub fn signal_execution_family(&self) -> Option<WorthQueryDeclarationSignalExecutionFamily> {
        self.signal_execution_family
    }
    pub fn basis_families(&self) -> &[BasisFamily] {
        &self.basis_families
    }
    pub fn seam_classification(&self) -> WorthQueryDeclarationEntrySeamClassification {
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
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> Vec<WorthQueryDeclarationEntryCrossingRow> {
    let taxonomy = I::Family::taxonomy();
    let mut rows = Vec::new();
    push_rows(
        &mut rows,
        handle,
        ENVELOPE_ENTRYPOINTS,
        WorthQueryDeclarationEntryCrossingSurface::Envelope,
        I::Family::semantic_family_key(),
        taxonomy.primary_authority_family(),
        WorthQueryDeclarationEntryLowerOwnerCrate::Query,
        None,
        Some(WorthQueryDeclarationReceiptClass::CoveredCrossing),
        Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing),
        None,
        None,
        None,
        None,
        None,
        None,
        &[],
        WorthQueryDeclarationEntrySeamClassification::CanonicalReuse,
    );
    push_relational_rows::<D, C, I>(&mut rows, handle, taxonomy.primary_authority_family());
    push_bridge_rows::<D, C, I>(&mut rows, handle, taxonomy.primary_authority_family());
    push_signal_rows::<D, C, I>(&mut rows, handle, taxonomy.primary_authority_family());
    rows
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
fn push_rows<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>(rows: &mut Vec<WorthQueryDeclarationEntryCrossingRow>, handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>, entrypoint_keys: &[&'static str], surface: WorthQueryDeclarationEntryCrossingSurface, declaration_family_key: &'static str, primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily, lower_owner_crate: WorthQueryDeclarationEntryLowerOwnerCrate, route_family: Option<WorthQueryLowerAuthorityRouteFamily>, receipt_class: Option<WorthQueryDeclarationReceiptClass>, envelope_class: Option<WorthQueryDeclarationEnvelopeClass>, relational_truth_claim: Option<WorthQueryDeclarationRelationalTruthClaim>, relational_authority_family: Option<WorthQueryDeclarationRelationalAuthorityFamily>, bridge_continuation_mode: Option<WorthQueryDeclarationBridgeContinuationMode>, bridge_truth_context: Option<WorthQueryDeclarationBridgeTruthContext>, bridge_continuation_family: Option<WorthQueryDeclarationBridgeContinuationFamily>, signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>, basis_families: &[BasisFamily], seam_classification: WorthQueryDeclarationEntrySeamClassification) {
    for entrypoint_key in entrypoint_keys { rows.push(WorthQueryDeclarationEntryCrossingRow::new(entrypoint_key, surface, declaration_family_key, primary_authority_family, lower_owner_crate, route_family, receipt_class, envelope_class, relational_truth_claim, relational_authority_family, bridge_continuation_mode, bridge_truth_context, bridge_continuation_family, signal_execution_family, basis_families.to_vec(), seam_classification, handle.handle_identity_digest(), handle.operating_context_identity_digest())); }
}

#[rustfmt::skip]
fn push_relational_rows<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>, I: WorthQueryDeclarationInput<D>>(rows: &mut Vec<WorthQueryDeclarationEntryCrossingRow>, handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>, primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily) {
    let (classification, direct, higher, claim, authority) = match I::Family::relational_truth_contract() {
        Some(contract) => (WorthQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter, RELATIONAL_DIRECT, Some(RELATIONAL_HIGHER), contract.truth_claim(), contract.authority_family()),
        None => (WorthQueryDeclarationEntrySeamClassification::ForbiddenDuplicate, RELATIONAL_DIRECT, None, WorthQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth, WorthQueryDeclarationRelationalAuthorityFamily::Runtime),
    };
    push_rows(rows, handle, direct, WorthQueryDeclarationEntryCrossingSurface::RelationalTruthRouting, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHRelational, Some(WorthQueryLowerAuthorityRouteFamily::Relational), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), Some(claim), Some(authority), None, None, None, None, &[], classification);
    if let Some(higher) = higher { push_rows(rows, handle, higher, WorthQueryDeclarationEntryCrossingSurface::RelationalTruthRouting, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHRelational, Some(WorthQueryLowerAuthorityRouteFamily::Relational), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), Some(claim), Some(authority), None, None, None, None, &[], classification); }
}

#[rustfmt::skip]
fn push_bridge_rows<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>, I: WorthQueryDeclarationInput<D>>(rows: &mut Vec<WorthQueryDeclarationEntryCrossingRow>, handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>, primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily) {
    let (classification, direct, higher, mode, truth_context, family) = match I::Family::bridge_continuation_contract() {
        Some(contract) => (WorthQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter, BRIDGE_DIRECT, Some(BRIDGE_HIGHER), contract.request().mode(), contract.request().truth_context(), contract.family()),
        None => (WorthQueryDeclarationEntrySeamClassification::ForbiddenDuplicate, BRIDGE_DIRECT, None, WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute, WorthQueryDeclarationBridgeTruthContext::Current, WorthQueryDeclarationBridgeContinuationFamily::RuntimeRoute),
    };
    push_rows(rows, handle, direct, WorthQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHRuntimeBridge, Some(WorthQueryLowerAuthorityRouteFamily::Bridge), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, Some(mode), Some(truth_context), Some(family), None, &[], classification);
    if let Some(higher) = higher { push_rows(rows, handle, higher, WorthQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHRuntimeBridge, Some(WorthQueryLowerAuthorityRouteFamily::Bridge), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, Some(mode), Some(truth_context), Some(family), None, &[], classification); }
}

#[rustfmt::skip]
fn push_signal_rows<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>, I: WorthQueryDeclarationInput<D>>(rows: &mut Vec<WorthQueryDeclarationEntryCrossingRow>, handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>, primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily) {
    match I::Family::taxonomy().signal_compatibility() {
        WorthQuerySignalCompatibilityPosture::Compatible => if let Some(contract) = I::Family::signal_compatibility_contract() {
            let execution_family = if primary_authority_family == WorthQueryDeclarationPrimaryAuthorityFamily::MixedAuthority { WorthQueryDeclarationSignalExecutionFamily::MixedDerivedExecution } else { contract.execution_family() };
            for basis_family in contract.required_basis_families() {
                push_rows(rows, handle, SIGNAL_DIRECT, WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHSignal, Some(WorthQueryLowerAuthorityRouteFamily::Signal), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(execution_family), &[*basis_family], WorthQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter);
                push_rows(rows, handle, SIGNAL_HIGHER, WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHSignal, Some(WorthQueryLowerAuthorityRouteFamily::Signal), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(execution_family), &[*basis_family], WorthQueryDeclarationEntrySeamClassification::QueryBoundaryAdapter);
            }
        },
        WorthQuerySignalCompatibilityPosture::Deferred => push_rows(rows, handle, SIGNAL_DIRECT, WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHSignal, Some(WorthQueryLowerAuthorityRouteFamily::Signal), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(WorthQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution), &[BasisFamily::CurrentHead], WorthQueryDeclarationEntrySeamClassification::DeferredNeighbor),
        WorthQuerySignalCompatibilityPosture::NotCompatible => push_rows(rows, handle, SIGNAL_DIRECT, WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility, I::Family::semantic_family_key(), primary_authority_family, WorthQueryDeclarationEntryLowerOwnerCrate::WORTHSignal, Some(WorthQueryLowerAuthorityRouteFamily::Signal), Some(WorthQueryDeclarationReceiptClass::CoveredCrossing), Some(WorthQueryDeclarationEnvelopeClass::CoveredCrossing), None, None, None, None, None, Some(WorthQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution), &[BasisFamily::CurrentHead], WorthQueryDeclarationEntrySeamClassification::ForbiddenDuplicate),
    }
}
