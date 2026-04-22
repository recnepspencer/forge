use std::collections::BTreeSet;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::mapping::SubscriptionSliceKind;

use super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeclarationIdentity, BridgeSubscriptionDeclarationRejection,
    BridgeSubscriptionDeclarationRejectionKind, FrozenSubscriptionFamilyRegistration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeliveryIntentClass {
    None,
    CanonicalMeaningfulChange,
}

impl BridgeSubscriptionDeliveryIntentClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CanonicalMeaningfulChange => "canonical_meaningful_change",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizedSubscriptionSliceIntentErrorKind {
    EmptyEntityIdentity,
    EmptyAspectLabel,
    EmptySurfaceLabel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedSubscriptionSliceIntentError {
    kind: NormalizedSubscriptionSliceIntentErrorKind,
}

impl NormalizedSubscriptionSliceIntentError {
    const fn new(kind: NormalizedSubscriptionSliceIntentErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> NormalizedSubscriptionSliceIntentErrorKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedSubscriptionSliceIntent {
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
    slice_kind: SubscriptionSliceKind,
}

impl NormalizedSubscriptionSliceIntent {
    pub fn try_new(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
    ) -> Result<Self, NormalizedSubscriptionSliceIntentError> {
        let entity_identity = entity_identity.into();
        if entity_identity.is_empty() {
            return Err(NormalizedSubscriptionSliceIntentError::new(
                NormalizedSubscriptionSliceIntentErrorKind::EmptyEntityIdentity,
            ));
        }
        let aspect_label = aspect_label.into();
        if aspect_label.is_empty() {
            return Err(NormalizedSubscriptionSliceIntentError::new(
                NormalizedSubscriptionSliceIntentErrorKind::EmptyAspectLabel,
            ));
        }
        let surface_label = surface_label.into();
        if surface_label.is_empty() {
            return Err(NormalizedSubscriptionSliceIntentError::new(
                NormalizedSubscriptionSliceIntentErrorKind::EmptySurfaceLabel,
            ));
        }
        Ok(Self {
            entity_identity,
            aspect_label,
            surface_label,
            slice_kind,
        })
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }

    pub fn slice_kind(&self) -> &SubscriptionSliceKind {
        &self.slice_kind
    }
}

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
        basis.push_str(intent.entity_identity());
        basis.push('/');
        basis.push_str(intent.aspect_label());
        basis.push('/');
        basis.push_str(intent.surface_label());
        basis.push('/');
        basis.push_str(subscription_slice_kind_label(intent.slice_kind()));
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
    BridgeSubscriptionDeclarationIdentity::new(format!(
        "bridge-subscription-declaration-id:sha256:{digest:x}"
    ))
}

fn subscription_slice_kind_label(slice_kind: &SubscriptionSliceKind) -> &'static str {
    match slice_kind {
        SubscriptionSliceKind::SignalField => "signal_field",
        SubscriptionSliceKind::SignalLens => "signal_lens",
        SubscriptionSliceKind::SignalRegion => "signal_region",
        SubscriptionSliceKind::SignalPartition => "signal_partition",
        SubscriptionSliceKind::SignalFacet => "signal_facet",
        SubscriptionSliceKind::RegisteredCoarseFallback => "registered_coarse_fallback",
    }
}

#[cfg(test)]
mod tests {
    use crate::mapping::SubscriptionSliceKind;

    use super::{
        BridgeSubscriptionDeclaration, BridgeSubscriptionDeliveryIntentClass,
        NormalizedSubscriptionSliceIntent, NormalizedSubscriptionSliceIntentErrorKind,
    };
    use crate::subscription::{
        phase_one_subscription_families, BridgeSubscriptionDeclarationFamilyKind,
        FrozenSubscriptionFamilyRegistry,
    };

    #[test]
    fn same_inputs_produce_identical_declaration_digest() {
        let registry = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let family = registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::DetailExact);
        let left = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            BridgeSubscriptionDeliveryIntentClass::None,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            family,
        )
        .expect("declaration should normalize");
        let right = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            BridgeSubscriptionDeliveryIntentClass::None,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            family,
        )
        .expect("declaration should normalize");

        assert_eq!(left, right);
        assert_eq!(left.digest(), right.digest());
    }

    #[test]
    fn slice_order_normalizes_canonically() {
        let registry = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let family =
            registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::CollectionMembership);
        let left = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            BridgeSubscriptionDeliveryIntentClass::None,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west-partition",
                    SubscriptionSliceKind::SignalPartition,
                )
                .expect("slice intent should validate"),
            ],
            family,
        )
        .expect("declaration should normalize");
        let right = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            BridgeSubscriptionDeliveryIntentClass::None,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west-partition",
                    SubscriptionSliceKind::SignalPartition,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("slice intent should validate"),
            ],
            family,
        )
        .expect("declaration should normalize");

        assert_eq!(left, right);
        assert_eq!(left.digest(), right.digest());
    }

    #[test]
    fn duplicate_slice_intents_collapse_canonically() {
        let registry = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let family = registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::DetailExact);
        let declaration = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            BridgeSubscriptionDeliveryIntentClass::None,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "name",
                    SubscriptionSliceKind::SignalField,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "name",
                    SubscriptionSliceKind::SignalField,
                )
                .expect("slice intent should validate"),
            ],
            family,
        )
        .expect("declaration should normalize");

        assert_eq!(declaration.normalized_slice_intent_count(), 1);
        assert_eq!(
            declaration
                .counters()
                .declaration_deduplicated_slice_intent_count(),
            1
        );
    }

    #[test]
    fn wrong_slice_kind_for_family_rejects_deterministically() {
        let registry = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let family = registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::DetailExact);
        let error = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            BridgeSubscriptionDeliveryIntentClass::None,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            family,
        )
        .expect_err("region slices should not be admitted for detail family");

        assert_eq!(
            error.rejection_kind(),
            crate::subscription::BridgeSubscriptionDeclarationRejectionKind::UnsupportedSliceKindForFamily
        );
        assert_eq!(error.counters().declaration_rejection_count(), 1);
    }

    #[test]
    fn non_identity_delivery_intent_is_canonicalized_away() {
        let registry = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let family =
            registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::CollectionMembership);
        let declaration = BridgeSubscriptionDeclaration::new(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            BridgeSubscriptionDeliveryIntentClass::CanonicalMeaningfulChange,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            family,
        )
        .expect("declaration should normalize");

        assert_eq!(
            declaration.delivery_intent_class(),
            BridgeSubscriptionDeliveryIntentClass::None
        );
    }

    #[test]
    fn slice_intent_rejects_empty_identity_bearing_fields() {
        let error = NormalizedSubscriptionSliceIntent::try_new(
            "",
            "profile",
            "name",
            SubscriptionSliceKind::SignalField,
        )
        .expect_err("empty entity identity should reject");

        assert_eq!(
            error.kind(),
            NormalizedSubscriptionSliceIntentErrorKind::EmptyEntityIdentity
        );
    }
}
