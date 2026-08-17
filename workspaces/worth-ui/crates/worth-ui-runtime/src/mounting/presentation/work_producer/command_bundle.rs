use worth_ui_host_contract::{
    UiMountedPaintCommand, UiMountedPaintCommandIdentity, UiSemanticTextSlot,
};

use crate::runtime::persistent_index::{UiPersistentOrdMap, UiPersistentRankedSequence};

#[cfg(test)]
thread_local! {
    static LAST_LOOKUP_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[derive(Clone, Default)]
pub(super) struct UiMountedPresentationCommandBundle {
    commands: UiPersistentOrdMap<UiMountedPresentationCommandKey, UiMountedPaintCommand>,
    order: UiPersistentRankedSequence<UiMountedPresentationCommandKey>,
    identities:
        UiPersistentOrdMap<UiMountedPresentationIdentityKey, UiMountedPresentationCommandKey>,
    positions: UiPersistentOrdMap<UiMountedPresentationIdentityKey, usize>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct UiMountedPresentationCommandKey {
    family: u8,
    slot: u16,
    collection: Option<[u8; 32]>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct UiMountedPresentationIdentityKey(u64, u64);

impl UiMountedPresentationCommandBundle {
    pub(super) fn from_commands(commands: &[UiMountedPaintCommand]) -> Self {
        let mut bundle = Self::default();
        for (position, command) in commands.iter().enumerate() {
            let key = UiMountedPresentationCommandKey::for_command(command);
            let identity = UiMountedPresentationIdentityKey::for_identity(command.identity());
            assert!(
                bundle.commands.get(&key).is_none(),
                "command keys are unique"
            );
            assert!(
                bundle.identities.get(&identity).is_none(),
                "command identity registry keys are unique"
            );
            bundle
                .order
                .insert(bundle.order.len(), key)
                .expect("bounded command bundle rank");
            bundle.commands.insert(key, command.clone());
            bundle.identities.insert(identity, key);
            bundle.positions.insert(identity, position);
        }
        bundle
    }

    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = &UiMountedPaintCommand> {
        self.order.iter().map(|key| {
            self.commands
                .get(key)
                .expect("command order names an indexed command")
        })
    }

    pub(super) fn get(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<&UiMountedPaintCommand> {
        let (command, _probes) = self.lookup(identity);
        #[cfg(test)]
        LAST_LOOKUP_PROBES.with(|observed| observed.set(_probes));
        command
    }

    pub(super) fn position(&self, identity: UiMountedPaintCommandIdentity) -> Option<usize> {
        self.positions
            .get(&UiMountedPresentationIdentityKey::for_identity(identity))
            .copied()
    }

    pub(super) fn replace(&mut self, replacement: UiMountedPaintCommand) -> bool {
        let key = UiMountedPresentationCommandKey::for_command(&replacement);
        let identity = UiMountedPresentationIdentityKey::for_identity(replacement.identity());
        if self.identities.get(&identity) != Some(&key) {
            return false;
        }
        let Some(predecessor) = self.commands.get(&key) else {
            return false;
        };
        if predecessor.identity() != replacement.identity()
            || predecessor.layer_semantic_order() != replacement.layer_semantic_order()
        {
            return false;
        }
        self.commands.insert(key, replacement);
        true
    }

    fn lookup(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> (Option<&UiMountedPaintCommand>, usize) {
        let lookup = UiMountedPresentationIdentityKey::for_identity(identity);
        let (key, identity_probes) = self.identities.get_with_probes(&lookup);
        let Some(key) = key else {
            return (None, identity_probes);
        };
        let (command, command_probes) = self.commands.get_with_probes(key);
        (
            command.filter(|command| command.identity() == identity),
            identity_probes + command_probes,
        )
    }

    #[cfg(test)]
    fn lookup_probes(&self, identity: UiMountedPaintCommandIdentity) -> usize {
        self.lookup(identity).1
    }
}

#[cfg(test)]
pub(super) fn take_last_lookup_probes() -> usize {
    LAST_LOOKUP_PROBES.with(|observed| observed.replace(0))
}

impl UiMountedPresentationIdentityKey {
    fn for_identity(identity: UiMountedPaintCommandIdentity) -> Self {
        use std::hash::{Hash, Hasher};
        let mut first = UiMountedPresentationIdentityHasher::new(0xcbf2_9ce4_8422_2325);
        let mut second = UiMountedPresentationIdentityHasher::new(0x6c62_272e_07bb_0142);
        identity.hash(&mut first);
        identity.hash(&mut second);
        Self(first.finish(), second.finish())
    }
}

struct UiMountedPresentationIdentityHasher(u64);

impl UiMountedPresentationIdentityHasher {
    const fn new(seed: u64) -> Self {
        Self(seed)
    }
}

impl std::hash::Hasher for UiMountedPresentationIdentityHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

impl UiMountedPresentationCommandKey {
    fn for_command(command: &UiMountedPaintCommand) -> Self {
        match command {
            UiMountedPaintCommand::FilledRect { .. } => Self {
                family: 0,
                slot: 0,
                collection: None,
            },
            UiMountedPaintCommand::SemanticText { mechanic, .. } => {
                let slot = match mechanic.slot() {
                    UiSemanticTextSlot::Value => 0,
                    UiSemanticTextSlot::CollectionValue {
                        selected_field_ordinal,
                    } => selected_field_ordinal.saturating_add(1),
                    UiSemanticTextSlot::Posture => u16::MAX,
                };
                Self {
                    family: 1,
                    slot,
                    collection: mechanic
                        .collection_row()
                        .map(|row| row.correlation_digest()),
                }
            }
        }
    }
}

#[cfg(test)]
pub(super) mod tests {
    use std::sync::Arc;

    use worth_ui_host_contract::{
        UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedCanonicalBoxInput,
        UiMountedContentGeneration, UiMountedCoordinateSpace, UiMountedFrameIdentity,
        UiMountedInstanceIdentity, UiMountedNodeReceiptIssuer, UiMountedPaintCommand,
        UiMountedRgba8, UiMountedSemanticTextCompletionInput, UiMountedSemanticTextMechanic,
        UiMountedTextForegroundSpan, UiMountedTextPaintSpanIdentity, UiMountedTransformProjection,
        UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
        WorthUiHostCapabilityObservationGeneration,
    };

    use super::UiMountedPresentationCommandBundle;

    #[test]
    fn first_middle_and_last_collection_commands_use_bounded_direct_lookup() {
        let commands = collection_commands(1_359);
        let bundle = UiMountedPresentationCommandBundle::from_commands(&commands);
        for index in [0, commands.len() / 2, commands.len() - 1] {
            let identity = commands[index].identity();
            assert_eq!(bundle.get(identity), Some(&commands[index]));
            assert_eq!(bundle.position(identity), Some(index));
            assert!(bundle.lookup_probes(identity) <= 32);
        }
    }

    pub(crate) fn collection_commands(count: usize) -> Vec<UiMountedPaintCommand> {
        let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
        let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let instance = UiMountedInstanceIdentity::mint_unbound().unwrap();
        let receipt = UiMountedNodeReceiptIssuer::mint_for(frame)
            .unwrap()
            .receipt_for(instance);
        let bounds = UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 24.0,
            coordinate_space: UiMountedCoordinateSpace::HostSurface,
        })
        .unwrap();
        (0..count)
            .map(|index| {
                let mechanic = UiMountedSemanticTextMechanic::complete_from_runtime_mounting(
                    UiMountedSemanticTextCompletionInput {
                        content_generation: UiMountedContentGeneration::mint_unbound().unwrap(),
                        frame,
                        surface,
                        binding,
                        mounted_instance: instance,
                        node_receipt: receipt,
                        allocation_basis: UiMountedAllocationBasis::new(
                            1,
                            2,
                            3,
                            UiMountedTransformProjection::Identity,
                        ),
                        bounds,
                        clip_bounds: bounds,
                        origin_x: 0.0,
                        origin_y: 0.0,
                        text: Arc::from("WORTH"),
                        layout: crate::mounting::qualified_text_test_support::inert_qualified_layout(
                            "WORTH",
                        )
                        .view(),
                        slot: worth_ui_host_contract::UiSemanticTextSlot::CollectionValue {
                            selected_field_ordinal: u16::try_from(index).unwrap(),
                        },
                        collection_row: Some(
                            worth_ui_host_contract::UiMountedCollectionRowCorrelation::from_runtime_mounting(
                                collection_identity(index),
                            ),
                        ),
                        foregrounds: Arc::from([
                            UiMountedTextForegroundSpan::from_runtime_mounting(
                                worth_ui_host_contract::UiTextOriginalRange::from_text_mechanics(
                                    0, 5,
                                )
                                .unwrap(),
                                UiMountedRgba8::new(255, 255, 255, 255),
                                UiMountedTextPaintSpanIdentity::from_runtime_mounting([1; 32]),
                            ),
                        ]),
                        profile: worth_ui_host_contract::UiSemanticTextProfile::BodyDefault,
                        layer_semantic_order: 7,
                        capability_generation: WorthUiHostCapabilityObservationGeneration::new(7),
                        capability_profile_digest: 11,
                    },
                )
                .unwrap();
                UiMountedPaintCommand::SemanticText {
                    identity: worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(
                        &mechanic,
                    ),
                    mechanic,
                }
            })
            .collect()
    }

    fn collection_identity(index: usize) -> [u8; 32] {
        let mut identity = [0_u8; 32];
        identity[..8].copy_from_slice(&u64::try_from(index).unwrap().to_le_bytes());
        identity
    }
}
