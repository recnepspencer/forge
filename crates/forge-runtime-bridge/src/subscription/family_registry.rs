use std::collections::BTreeSet;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::SubscriptionSliceKind;

use super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeclarationFamily,
    BridgeSubscriptionDeclarationFamilyIdentity, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionFamilyRegistryIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenSubscriptionFamilyRegistration {
    family: BridgeSubscriptionDeclarationFamily,
}

impl FrozenSubscriptionFamilyRegistration {
    fn new(family: BridgeSubscriptionDeclarationFamily) -> Self {
        Self { family }
    }

    pub fn family_identity(&self) -> &BridgeSubscriptionDeclarationFamilyIdentity {
        self.family.family_identity()
    }

    pub fn family_kind(&self) -> BridgeSubscriptionDeclarationFamilyKind {
        self.family.family_kind()
    }

    pub fn family_label(&self) -> &str {
        self.family.family_label()
    }

    pub fn supported_slice_kinds(&self) -> &[SubscriptionSliceKind] {
        self.family.supported_slice_kinds()
    }

    pub fn allows_delivery_intent_identity(&self) -> bool {
        self.family.allows_delivery_intent_identity()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenSubscriptionFamilyRegistry {
    registry_identity: BridgeSubscriptionFamilyRegistryIdentity,
    registrations: Arc<[FrozenSubscriptionFamilyRegistration]>,
    detail_exact_index: usize,
    collection_membership_index: usize,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
}

impl FrozenSubscriptionFamilyRegistry {
    pub(crate) fn freeze(
        mut families: Vec<BridgeSubscriptionDeclarationFamily>,
    ) -> Result<Self, BridgeBuildError> {
        if families.is_empty() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                "Bridge subscription family registry requires at least one declaration family.",
            ));
        }

        families.sort_by(canonical_family_order);
        validate_family_set(&families)?;

        let registrations = Arc::<[FrozenSubscriptionFamilyRegistration]>::from(
            families
                .into_iter()
                .map(FrozenSubscriptionFamilyRegistration::new)
                .collect::<Vec<_>>(),
        );

        let detail_exact_index = registrations
            .iter()
            .position(|registration| {
                registration.family_kind() == BridgeSubscriptionDeclarationFamilyKind::DetailExact
            })
            .ok_or_else(|| {
                BridgeBuildError::new(
                    BridgeBuildErrorKind::BuilderConfigurationConflict,
                    "Bridge subscription family registry is missing the required `detail_exact` family.",
                )
            })?;
        let collection_membership_index = registrations
            .iter()
            .position(|registration| {
                registration.family_kind()
                    == BridgeSubscriptionDeclarationFamilyKind::CollectionMembership
            })
            .ok_or_else(|| {
                BridgeBuildError::new(
                    BridgeBuildErrorKind::BuilderConfigurationConflict,
                    "Bridge subscription family registry is missing the required `collection_membership` family.",
                )
            })?;

        let mut basis = format!(
            "bridge-subscription-family-registry|family-count={}",
            registrations.len()
        );
        for registration in registrations.iter() {
            basis.push_str("|family=");
            basis.push_str(registration.family_identity().as_str());
            basis.push_str("|kind=");
            basis.push_str(registration.family_kind().as_str());
            basis.push_str("|label=");
            basis.push_str(registration.family_label());
            basis.push_str("|delivery-intent-identity=");
            basis.push_str(if registration.allows_delivery_intent_identity() {
                "true"
            } else {
                "false"
            });
            for slice_kind in registration.supported_slice_kinds() {
                basis.push_str("|slice-kind=");
                basis.push_str(match slice_kind {
                    SubscriptionSliceKind::SignalField => "signal_field",
                    SubscriptionSliceKind::SignalLens => "signal_lens",
                    SubscriptionSliceKind::SignalRegion => "signal_region",
                    SubscriptionSliceKind::SignalPartition => "signal_partition",
                    SubscriptionSliceKind::SignalFacet => "signal_facet",
                    SubscriptionSliceKind::RegisteredCoarseFallback => "registered_coarse_fallback",
                });
            }
        }
        let canonical_basis = Arc::<str>::from(basis);
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let family_count = registrations.len();
        let family_supported_slice_kind_count = registrations
            .iter()
            .map(|registration| registration.supported_slice_kinds().len())
            .sum();
        Ok(Self {
            registry_identity: BridgeSubscriptionFamilyRegistryIdentity::new(format!(
                "bridge-subscription-family-registry:sha256:{digest:x}"
            )),
            registrations,
            detail_exact_index,
            collection_membership_index,
            counters: BridgeSubscriptionCounters::from_frozen_registry(
                family_count,
                family_supported_slice_kind_count,
            ),
            canonical_basis,
        })
    }

    pub fn registry_identity(&self) -> &BridgeSubscriptionFamilyRegistryIdentity {
        &self.registry_identity
    }

    pub fn registrations(&self) -> &[FrozenSubscriptionFamilyRegistration] {
        &self.registrations
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub(crate) fn family_for_kind(
        &self,
        family_kind: BridgeSubscriptionDeclarationFamilyKind,
    ) -> &FrozenSubscriptionFamilyRegistration {
        let index = match family_kind {
            BridgeSubscriptionDeclarationFamilyKind::DetailExact => self.detail_exact_index,
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership => {
                self.collection_membership_index
            }
        };
        &self.registrations[index]
    }
}

pub(crate) fn freeze_subscription_family_registry(
) -> Result<FrozenSubscriptionFamilyRegistry, BridgeBuildError> {
    FrozenSubscriptionFamilyRegistry::freeze(phase_one_subscription_families()?)
}

pub(crate) fn phase_one_subscription_families(
) -> Result<Vec<BridgeSubscriptionDeclarationFamily>, BridgeBuildError> {
    Ok(vec![
        BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new(
                "subscription-family:collection-membership",
            ),
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            "collection_membership",
            vec![
                SubscriptionSliceKind::SignalPartition,
                SubscriptionSliceKind::SignalRegion,
            ],
            false,
        )?,
        BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            "detail_exact",
            vec![
                SubscriptionSliceKind::SignalField,
                SubscriptionSliceKind::SignalLens,
            ],
            false,
        )?,
    ])
}

fn canonical_family_order(
    left: &BridgeSubscriptionDeclarationFamily,
    right: &BridgeSubscriptionDeclarationFamily,
) -> std::cmp::Ordering {
    left.family_kind()
        .cmp(&right.family_kind())
        .then_with(|| left.family_identity().cmp(right.family_identity()))
}

fn validate_family_set(
    families: &[BridgeSubscriptionDeclarationFamily],
) -> Result<(), BridgeBuildError> {
    let mut family_kinds = BTreeSet::new();
    let mut family_identities = BTreeSet::new();
    let mut family_labels = BTreeSet::new();

    for family in families {
        let supported_slice_kinds = family.supported_slice_kinds();
        if supported_slice_kinds.is_empty() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family `{}` admitted zero supported slice kinds.",
                    family.family_kind().as_str()
                ),
            ));
        }
        if supported_slice_kinds
            .windows(2)
            .any(|window| window[0] >= window[1])
        {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family `{}` contained non-canonical or duplicate supported slice kinds.",
                    family.family_kind().as_str()
                ),
            ));
        }
        if !family_kinds.insert(family.family_kind()) {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family registry contained duplicate family kind `{}`.",
                    family.family_kind().as_str()
                ),
            ));
        }
        if !family_identities.insert(family.family_identity().clone()) {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family registry contained duplicate family identity `{}`.",
                    family.family_identity().as_str()
                ),
            ));
        }
        if !family_labels.insert(Arc::<str>::from(family.family_label())) {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family registry contained duplicate family label `{}`.",
                    family.family_label()
                ),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::mapping::SubscriptionSliceKind;

    use super::{
        phase_one_subscription_families, BridgeSubscriptionDeclarationFamily,
        BridgeSubscriptionDeclarationFamilyIdentity, BridgeSubscriptionDeclarationFamilyKind,
        FrozenSubscriptionFamilyRegistry,
    };

    #[test]
    fn frozen_registry_order_is_canonical_and_stable() {
        let left = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let mut reversed =
            phase_one_subscription_families().expect("phase 1 families should build");
        reversed.reverse();
        let right = FrozenSubscriptionFamilyRegistry::freeze(reversed)
            .expect("reversed families should freeze");

        assert_eq!(left, right);
        assert_eq!(left.registry_identity(), right.registry_identity());
    }

    #[test]
    fn duplicate_family_metadata_is_rejected() {
        let duplicate = BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new(
                "subscription-family:detail-duplicate",
            ),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            "detail_duplicate",
            vec![SubscriptionSliceKind::SignalField],
            false,
        )
        .expect("duplicate family should build");
        let existing_detail = phase_one_subscription_families()
            .expect("phase 1 families should build")
            .into_iter()
            .find(|family| {
                family.family_kind() == BridgeSubscriptionDeclarationFamilyKind::DetailExact
            })
            .expect("detail family should exist");
        let error = FrozenSubscriptionFamilyRegistry::freeze(vec![existing_detail, duplicate])
            .expect_err("duplicate family kind should be rejected");

        assert_eq!(
            error.kind(),
            crate::error::BridgeBuildErrorKind::BuilderConfigurationConflict
        );
    }

    #[test]
    fn registry_identity_changes_when_family_semantics_change() {
        let baseline = FrozenSubscriptionFamilyRegistry::freeze(
            phase_one_subscription_families().expect("phase 1 families should build"),
        )
        .expect("phase 1 families should freeze");
        let modified = FrozenSubscriptionFamilyRegistry::freeze(vec![
            BridgeSubscriptionDeclarationFamily::new(
                BridgeSubscriptionDeclarationFamilyIdentity::new(
                    "subscription-family:collection-membership",
                ),
                BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
                "collection_membership",
                vec![
                    SubscriptionSliceKind::SignalPartition,
                    SubscriptionSliceKind::SignalRegion,
                ],
                false,
            )
            .expect("collection family should build"),
            BridgeSubscriptionDeclarationFamily::new(
                BridgeSubscriptionDeclarationFamilyIdentity::new(
                    "subscription-family:detail-exact",
                ),
                BridgeSubscriptionDeclarationFamilyKind::DetailExact,
                "detail_exact",
                vec![
                    SubscriptionSliceKind::SignalField,
                    SubscriptionSliceKind::SignalLens,
                    SubscriptionSliceKind::SignalFacet,
                ],
                false,
            )
            .expect("detail family should build"),
        ])
        .expect("modified families should freeze");

        assert_ne!(baseline.registry_identity(), modified.registry_identity());
    }

    #[test]
    fn family_constructor_canonicalizes_slice_kind_order() {
        let left = BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            "detail_exact",
            vec![
                SubscriptionSliceKind::SignalLens,
                SubscriptionSliceKind::SignalField,
            ],
            false,
        )
        .expect("family should build");
        let right = BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            "detail_exact",
            vec![
                SubscriptionSliceKind::SignalField,
                SubscriptionSliceKind::SignalLens,
            ],
            false,
        )
        .expect("family should build");

        assert_eq!(left.supported_slice_kinds(), right.supported_slice_kinds());
    }

    #[test]
    fn family_constructor_rejects_duplicate_slice_kinds() {
        let error = BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            "detail_exact",
            vec![
                SubscriptionSliceKind::SignalField,
                SubscriptionSliceKind::SignalField,
            ],
            false,
        )
        .expect_err("duplicate slice kinds should reject");

        assert_eq!(
            error.kind(),
            crate::error::BridgeBuildErrorKind::BuilderConfigurationConflict
        );
    }
}
