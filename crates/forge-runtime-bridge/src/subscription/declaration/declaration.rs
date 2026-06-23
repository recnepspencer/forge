use std::collections::BTreeSet;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeclarationIdentity, BridgeSubscriptionDeclarationRejection,
    BridgeSubscriptionDeclarationRejectionKind, FrozenSubscriptionFamilyRegistration,
};
use super::{BridgeSubscriptionDeliveryIntentClass, NormalizedSubscriptionSliceIntent};

#[derive(Debug, Clone)]
pub struct BridgeSubscriptionDeclaration {
    declaration_identity: BridgeSubscriptionDeclarationIdentity,
    requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
    delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
    normalized_slice_intents: Arc<[NormalizedSubscriptionSliceIntent]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PartialEq for BridgeSubscriptionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.declaration_identity == other.declaration_identity
            && self.requested_family_kind == other.requested_family_kind
            && self.delivery_intent_class == other.delivery_intent_class
            && self.normalized_slice_intents == other.normalized_slice_intents
            && self.canonical_basis == other.canonical_basis
            && self.digest == other.digest
    }
}

impl Eq for BridgeSubscriptionDeclaration {}

impl BridgeSubscriptionDeclaration {
    pub(crate) fn new(
        requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
        delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
        normalized_slice_intents: Vec<NormalizedSubscriptionSliceIntent>,
        family_registration: &FrozenSubscriptionFamilyRegistration,
    ) -> Result<Self, BridgeSubscriptionDeclarationRejection> {
        let declaration_input_slice_intent_count = normalized_slice_intents.len();
        let normalized_slice_intents = canonicalize_slice_intents(
            requested_family_kind,
            delivery_intent_class,
            normalized_slice_intents,
            family_registration,
        )?;
        let declaration_deduplicated_slice_intent_count =
            declaration_input_slice_intent_count.saturating_sub(normalized_slice_intents.len());
        let delivery_intent_class =
            canonical_delivery_intent_class(delivery_intent_class, family_registration);
        let declaration_identity = declaration_identity_for_semantics(
            requested_family_kind,
            delivery_intent_class,
            &normalized_slice_intents,
            family_registration,
        );
        validate_family_slice_kinds(
            &declaration_identity,
            requested_family_kind,
            declaration_input_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            &normalized_slice_intents,
            family_registration,
        )?;

        let canonical_basis = declaration_basis(
            requested_family_kind,
            delivery_intent_class,
            &normalized_slice_intents,
            family_registration,
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            declaration_identity,
            requested_family_kind,
            delivery_intent_class,
            normalized_slice_intents,
            counters: BridgeSubscriptionCounters::from_declaration(
                declaration_input_slice_intent_count,
                canonical_basis.matches("|slice=").count(),
                declaration_deduplicated_slice_intent_count,
            ),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-declaration:sha256:{digest:x}")),
        })
    }

    pub fn declaration_identity(&self) -> &BridgeSubscriptionDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn requested_family_kind(&self) -> BridgeSubscriptionDeclarationFamilyKind {
        self.requested_family_kind
    }

    pub fn delivery_intent_class(&self) -> BridgeSubscriptionDeliveryIntentClass {
        self.delivery_intent_class
    }

    pub fn normalized_slice_intents(&self) -> &[NormalizedSubscriptionSliceIntent] {
        &self.normalized_slice_intents
    }

    pub fn normalized_slice_intent_count(&self) -> usize {
        self.normalized_slice_intents.len()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn canonicalize_slice_intents(
    requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
    delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
    normalized_slice_intents: Vec<NormalizedSubscriptionSliceIntent>,
    family_registration: &FrozenSubscriptionFamilyRegistration,
) -> Result<Arc<[NormalizedSubscriptionSliceIntent]>, BridgeSubscriptionDeclarationRejection> {
    if normalized_slice_intents.is_empty() {
        return Err(BridgeSubscriptionDeclarationRejection::new(
            declaration_identity_for_semantics(
                requested_family_kind,
                canonical_delivery_intent_class(delivery_intent_class, family_registration),
                &[],
                family_registration,
            ),
            requested_family_kind,
            BridgeSubscriptionDeclarationRejectionKind::EmptyDeclaration,
            0,
            0,
            0,
        ));
    }

    let input_count = normalized_slice_intents.len();
    let deduped = normalized_slice_intents
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if deduped.is_empty() {
        return Err(BridgeSubscriptionDeclarationRejection::new(
            declaration_identity_for_semantics(
                requested_family_kind,
                canonical_delivery_intent_class(delivery_intent_class, family_registration),
                &[],
                family_registration,
            ),
            requested_family_kind,
            BridgeSubscriptionDeclarationRejectionKind::CanonicalizationFailure,
            input_count,
            0,
            input_count,
        ));
    }
    Ok(deduped.into())
}

fn validate_family_slice_kinds(
    declaration_identity: &BridgeSubscriptionDeclarationIdentity,
    requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
    declaration_input_slice_intent_count: usize,
    declaration_deduplicated_slice_intent_count: usize,
    normalized_slice_intents: &[NormalizedSubscriptionSliceIntent],
    family_registration: &FrozenSubscriptionFamilyRegistration,
) -> Result<(), BridgeSubscriptionDeclarationRejection> {
    if normalized_slice_intents.iter().any(|intent| {
        !family_registration
            .supported_slice_kinds()
            .contains(intent.slice_kind())
    }) {
        return Err(BridgeSubscriptionDeclarationRejection::new(
            declaration_identity.clone(),
            requested_family_kind,
            BridgeSubscriptionDeclarationRejectionKind::UnsupportedSliceKindForFamily,
            declaration_input_slice_intent_count,
            normalized_slice_intents.len(),
            declaration_deduplicated_slice_intent_count,
        ));
    }
    Ok(())
}

fn canonical_delivery_intent_class(
    delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
    family_registration: &FrozenSubscriptionFamilyRegistration,
) -> BridgeSubscriptionDeliveryIntentClass {
    if family_registration.allows_delivery_intent_identity() {
        delivery_intent_class
    } else {
        BridgeSubscriptionDeliveryIntentClass::None
    }
}

fn declaration_basis(
    requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
    delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
    normalized_slice_intents: &[NormalizedSubscriptionSliceIntent],
    family_registration: &FrozenSubscriptionFamilyRegistration,
) -> Arc<str> {
    let mut basis = format!(
        "bridge-subscription-declaration|family={}",
        requested_family_kind.as_str(),
    );
    if family_registration.allows_delivery_intent_identity() {
        basis.push_str("|delivery-intent=");
        basis.push_str(delivery_intent_class.as_str());
    }
    basis.push_str("|normalized-slice-intent-count=");
    basis.push_str(&normalized_slice_intents.len().to_string());
    for intent in normalized_slice_intents {
        basis.push_str("|slice=");
        basis.push_str(intent.slice_target_identity().as_str());
    }
    Arc::from(basis)
}

fn declaration_identity_for_semantics(
    requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
    delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
    normalized_slice_intents: &[NormalizedSubscriptionSliceIntent],
    family_registration: &FrozenSubscriptionFamilyRegistration,
) -> BridgeSubscriptionDeclarationIdentity {
    let basis = declaration_basis(
        requested_family_kind,
        delivery_intent_class,
        normalized_slice_intents,
        family_registration,
    );
    let digest = Sha256::digest(basis.as_bytes());
    BridgeSubscriptionDeclarationIdentity::admit_bridge_owned(format!(
        "bridge-subscription-declaration-id:sha256:{digest:x}"
    ))
}
