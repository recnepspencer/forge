use forge_query::facade::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDomainEntryMarker,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KernelRouteCheckedSummary {
    label: &'static str,
    declaration_family_key: &'static str,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: Option<String>,
    route_class: Option<String>,
    route_families: Vec<String>,
    route_intent: Option<String>,
    denial_cause: Option<&'static str>,
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KernelReceiptCheckedSummary {
    label: &'static str,
    receipt_class: String,
    receipt_kind: String,
    declaration_family_key: &'static str,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    route_cause: Option<&'static str>,
    receipt_cause: Option<&'static str>,
    reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KernelEnvelopeCheckedSummary {
    label: &'static str,
    envelope_class: String,
    evidence_origin: String,
    declaration_family_key: &'static str,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    route_cause: Option<&'static str>,
    receipt_cause: Option<&'static str>,
    reason: Option<String>,
}

pub(crate) fn route_checked_summary<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> KernelRouteCheckedSummary {
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => KernelRouteCheckedSummary {
            label: "planned",
            declaration_family_key: plan.declaration_family_key(),
            handle_identity_digest: plan.handle_identity_digest().to_string(),
            operating_context_identity_digest: plan.operating_context_identity_digest().to_string(),
            declaration_digest: plan.declaration_digest().to_string(),
            progression_digest: plan.progression_digest().to_string(),
            route_plan_digest: Some(plan.route_plan_digest().to_string()),
            route_class: Some(format!("{:?}", plan.class())),
            route_families: plan
                .route_families()
                .iter()
                .map(|family| family.as_str().to_string())
                .collect(),
            route_intent: plan
                .route_intent()
                .map(|intent| intent.as_str().to_string()),
            denial_cause: None,
            reason: plan.explain().route_contract_reason().to_string(),
        },
        ForgeQueryDeclarationRoutePlanChecked::Deferred(plan) => {
            let progressed = plan.progressed_declaration();
            let evidence = plan.foundational_evidence();
            KernelRouteCheckedSummary {
                label: "deferred",
                declaration_family_key: plan.declaration_family_key(),
                handle_identity_digest: evidence.handle_identity_digest().to_string(),
                operating_context_identity_digest: evidence
                    .operating_context_identity_digest()
                    .to_string(),
                declaration_digest: format!(
                    "{:?}",
                    progressed.canonical_declaration().declaration_digest()
                ),
                progression_digest: progressed.progression_digest().to_string(),
                route_plan_digest: None,
                route_class: None,
                route_families: Vec::new(),
                route_intent: plan
                    .route_intent()
                    .map(|intent| intent.as_str().to_string()),
                denial_cause: None,
                reason: plan.reason().to_string(),
            }
        }
        ForgeQueryDeclarationRoutePlanChecked::Denied(plan) => {
            let progressed = plan.progressed_declaration();
            let evidence = plan.foundational_evidence();
            KernelRouteCheckedSummary {
                label: "denied",
                declaration_family_key: plan.declaration_family_key(),
                handle_identity_digest: evidence.handle_identity_digest().to_string(),
                operating_context_identity_digest: evidence
                    .operating_context_identity_digest()
                    .to_string(),
                declaration_digest: format!(
                    "{:?}",
                    progressed.canonical_declaration().declaration_digest()
                ),
                progression_digest: progressed.progression_digest().to_string(),
                route_plan_digest: None,
                route_class: None,
                route_families: Vec::new(),
                route_intent: plan
                    .route_intent()
                    .map(|intent| intent.as_str().to_string()),
                denial_cause: Some(plan.cause().as_str()),
                reason: plan.reason().to_string(),
            }
        }
        ForgeQueryDeclarationRoutePlanChecked::Failed(plan) => {
            let progressed = plan.progressed_declaration();
            let evidence = plan.foundational_evidence();
            KernelRouteCheckedSummary {
                label: "failed",
                declaration_family_key: plan.declaration_family_key(),
                handle_identity_digest: evidence.handle_identity_digest().to_string(),
                operating_context_identity_digest: evidence
                    .operating_context_identity_digest()
                    .to_string(),
                declaration_digest: format!(
                    "{:?}",
                    progressed.canonical_declaration().declaration_digest()
                ),
                progression_digest: progressed.progression_digest().to_string(),
                route_plan_digest: None,
                route_class: None,
                route_families: Vec::new(),
                route_intent: plan
                    .route_intent()
                    .map(|intent| intent.as_str().to_string()),
                denial_cause: None,
                reason: plan.reason().to_string(),
            }
        }
    }
}

pub(crate) fn receipt_checked_summary<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryDeclarationReceiptChecked<D, I>,
) -> KernelReceiptCheckedSummary {
    match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(receipt) => KernelReceiptCheckedSummary {
            label: "issued",
            receipt_class: format!("{:?}", receipt.class()),
            receipt_kind: format!("{:?}", receipt.kind()),
            declaration_family_key: receipt.declaration_family_key(),
            declaration_digest: receipt.declaration_digest().to_string(),
            progression_digest: receipt.progression_digest().map(ToOwned::to_owned),
            route_plan_digest: receipt.route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", receipt.receipt_digest()),
            route_cause: receipt.route_denial_cause().map(|cause| cause.as_str()),
            receipt_cause: None,
            reason: None,
        },
        ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => KernelReceiptCheckedSummary {
            label: "deferred",
            receipt_class: format!("{:?}", receipt.receipt().class()),
            receipt_kind: format!("{:?}", receipt.receipt().kind()),
            declaration_family_key: receipt.receipt().declaration_family_key(),
            declaration_digest: receipt.receipt().declaration_digest().to_string(),
            progression_digest: receipt
                .receipt()
                .progression_digest()
                .map(ToOwned::to_owned),
            route_plan_digest: receipt.receipt().route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", receipt.receipt().receipt_digest()),
            route_cause: receipt
                .receipt()
                .route_denial_cause()
                .map(|cause| cause.as_str()),
            receipt_cause: None,
            reason: Some(receipt.reason().to_string()),
        },
        ForgeQueryDeclarationReceiptChecked::Denied(receipt) => KernelReceiptCheckedSummary {
            label: "denied",
            receipt_class: format!("{:?}", receipt.receipt().class()),
            receipt_kind: format!("{:?}", receipt.receipt().kind()),
            declaration_family_key: receipt.receipt().declaration_family_key(),
            declaration_digest: receipt.receipt().declaration_digest().to_string(),
            progression_digest: receipt
                .receipt()
                .progression_digest()
                .map(ToOwned::to_owned),
            route_plan_digest: receipt.receipt().route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", receipt.receipt().receipt_digest()),
            route_cause: receipt.route_cause().map(|cause| cause.as_str()),
            receipt_cause: receipt.receipt_cause().map(|cause| cause.as_str()),
            reason: Some(receipt.reason().to_string()),
        },
        ForgeQueryDeclarationReceiptChecked::Failed(receipt) => KernelReceiptCheckedSummary {
            label: "failed",
            receipt_class: format!("{:?}", receipt.receipt().class()),
            receipt_kind: format!("{:?}", receipt.receipt().kind()),
            declaration_family_key: receipt.receipt().declaration_family_key(),
            declaration_digest: receipt.receipt().declaration_digest().to_string(),
            progression_digest: receipt
                .receipt()
                .progression_digest()
                .map(ToOwned::to_owned),
            route_plan_digest: receipt.receipt().route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", receipt.receipt().receipt_digest()),
            route_cause: receipt
                .receipt()
                .route_denial_cause()
                .map(|cause| cause.as_str()),
            receipt_cause: None,
            reason: Some(receipt.reason().to_string()),
        },
    }
}

pub(crate) fn envelope_checked_summary<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> KernelEnvelopeCheckedSummary {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => KernelEnvelopeCheckedSummary {
            label: "enveloped",
            envelope_class: format!("{:?}", envelope.class()),
            evidence_origin: format!("{:?}", envelope.evidence_origin()),
            declaration_family_key: envelope.declaration_family_key(),
            declaration_digest: envelope.declaration_digest().to_string(),
            progression_digest: envelope.progression_digest().map(ToOwned::to_owned),
            route_plan_digest: envelope.route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", envelope.receipt_digest()),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            route_cause: envelope.route_denial_cause().map(|cause| cause.as_str()),
            receipt_cause: envelope.receipt_denial_cause().map(|cause| cause.as_str()),
            reason: None,
        },
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => KernelEnvelopeCheckedSummary {
            label: "deferred",
            envelope_class: format!("{:?}", envelope.envelope().class()),
            evidence_origin: format!("{:?}", envelope.envelope().evidence_origin()),
            declaration_family_key: envelope.envelope().declaration_family_key(),
            declaration_digest: envelope.envelope().declaration_digest().to_string(),
            progression_digest: envelope
                .envelope()
                .progression_digest()
                .map(ToOwned::to_owned),
            route_plan_digest: envelope
                .envelope()
                .route_plan_digest()
                .map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", envelope.envelope().receipt_digest()),
            envelope_digest: format!("{:?}", envelope.envelope().envelope_digest()),
            route_cause: envelope
                .envelope()
                .route_denial_cause()
                .map(|cause| cause.as_str()),
            receipt_cause: envelope
                .envelope()
                .receipt_denial_cause()
                .map(|cause| cause.as_str()),
            reason: Some(envelope.reason().to_string()),
        },
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => KernelEnvelopeCheckedSummary {
            label: "denied",
            envelope_class: format!("{:?}", envelope.envelope().class()),
            evidence_origin: format!("{:?}", envelope.envelope().evidence_origin()),
            declaration_family_key: envelope.envelope().declaration_family_key(),
            declaration_digest: envelope.envelope().declaration_digest().to_string(),
            progression_digest: envelope
                .envelope()
                .progression_digest()
                .map(ToOwned::to_owned),
            route_plan_digest: envelope
                .envelope()
                .route_plan_digest()
                .map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", envelope.envelope().receipt_digest()),
            envelope_digest: format!("{:?}", envelope.envelope().envelope_digest()),
            route_cause: envelope.route_cause().map(|cause| cause.as_str()),
            receipt_cause: envelope.receipt_cause().map(|cause| cause.as_str()),
            reason: Some(envelope.reason().to_string()),
        },
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => KernelEnvelopeCheckedSummary {
            label: "failed",
            envelope_class: format!("{:?}", envelope.envelope().class()),
            evidence_origin: format!("{:?}", envelope.envelope().evidence_origin()),
            declaration_family_key: envelope.envelope().declaration_family_key(),
            declaration_digest: envelope.envelope().declaration_digest().to_string(),
            progression_digest: envelope
                .envelope()
                .progression_digest()
                .map(ToOwned::to_owned),
            route_plan_digest: envelope
                .envelope()
                .route_plan_digest()
                .map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", envelope.envelope().receipt_digest()),
            envelope_digest: format!("{:?}", envelope.envelope().envelope_digest()),
            route_cause: envelope
                .envelope()
                .route_denial_cause()
                .map(|cause| cause.as_str()),
            receipt_cause: envelope
                .envelope()
                .receipt_denial_cause()
                .map(|cause| cause.as_str()),
            reason: Some(envelope.reason().to_string()),
        },
    }
}
