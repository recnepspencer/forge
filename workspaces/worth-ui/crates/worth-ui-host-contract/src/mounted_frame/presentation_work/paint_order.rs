use super::UiMountedPaintCommandIdentity;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiMountedPaintOrderIdentity {
    command: UiMountedPaintCommandIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedPaintOrderIntegrity {
    digest: u64,
    length: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPaintOrderEdit {
    Remove(UiMountedPaintOrderIdentity),
    PlaceAfter {
        identity: UiMountedPaintOrderIdentity,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    },
}

impl UiMountedPaintOrderIdentity {
    #[doc(hidden)]
    pub const fn for_command(command: UiMountedPaintCommandIdentity) -> Self {
        Self { command }
    }

    pub const fn command(self) -> UiMountedPaintCommandIdentity {
        self.command
    }
}

impl UiMountedPaintOrderIntegrity {
    pub fn for_order(order: &[UiMountedPaintOrderIdentity]) -> Self {
        let length = u32::try_from(order.len()).unwrap_or(u32::MAX);
        let digest = order
            .iter()
            .copied()
            .scan(None, |previous, current| {
                let edge = edge_digest(*previous, current);
                *previous = Some(current);
                Some(edge)
            })
            .fold(length_digest(length), |digest, edge| digest ^ edge);
        Self { digest, length }
    }

    pub fn admits(self, order: &[UiMountedPaintOrderIdentity]) -> bool {
        self == Self::for_order(order)
    }

    #[doc(hidden)]
    pub fn remove_edge(
        self,
        predecessor: Option<UiMountedPaintOrderIdentity>,
        identity: UiMountedPaintOrderIdentity,
        successor: Option<UiMountedPaintOrderIdentity>,
    ) -> Option<Self> {
        let length = self.length.checked_sub(1)?;
        let mut digest = self.digest ^ length_digest(self.length) ^ length_digest(length);
        digest ^= edge_digest(predecessor, identity);
        if let Some(successor) = successor {
            digest ^= edge_digest(Some(identity), successor);
            digest ^= edge_digest(predecessor, successor);
        }
        Some(Self { digest, length })
    }

    #[doc(hidden)]
    pub fn insert_edge(
        self,
        predecessor: Option<UiMountedPaintOrderIdentity>,
        identity: UiMountedPaintOrderIdentity,
        successor: Option<UiMountedPaintOrderIdentity>,
    ) -> Option<Self> {
        let length = self.length.checked_add(1)?;
        let mut digest = self.digest ^ length_digest(self.length) ^ length_digest(length);
        if let Some(successor) = successor {
            digest ^= edge_digest(predecessor, successor);
            digest ^= edge_digest(Some(identity), successor);
        }
        digest ^= edge_digest(predecessor, identity);
        Some(Self { digest, length })
    }
}

fn length_digest(length: u32) -> u64 {
    0x517c_c1b7_2722_0a95_u64 ^ u64::from(length).wrapping_mul(0x9e37_79b1_85eb_ca87)
}

fn edge_digest(
    predecessor: Option<UiMountedPaintOrderIdentity>,
    identity: UiMountedPaintOrderIdentity,
) -> u64 {
    let predecessor = predecessor
        .map(|value| value.command.order_fingerprint())
        .unwrap_or(0xd6e8_feb8_6659_fd93);
    let current = identity.command.order_fingerprint();
    predecessor
        .rotate_left(17)
        .wrapping_mul(0x94d0_49bb_1331_11eb)
        ^ current.wrapping_mul(0xff51_afd7_ed55_8ccd)
}

impl UiMountedPaintOrderEdit {
    #[doc(hidden)]
    pub const fn place_after(
        identity: UiMountedPaintOrderIdentity,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    ) -> Self {
        Self::PlaceAfter {
            identity,
            predecessor,
        }
    }

    #[doc(hidden)]
    pub const fn remove(identity: UiMountedPaintOrderIdentity) -> Self {
        Self::Remove(identity)
    }

    pub const fn identity(self) -> UiMountedPaintOrderIdentity {
        match self {
            Self::Remove(identity) | Self::PlaceAfter { identity, .. } => identity,
        }
    }

    pub const fn predecessor(self) -> Option<UiMountedPaintOrderIdentity> {
        match self {
            Self::Remove(_) => None,
            Self::PlaceAfter { predecessor, .. } => predecessor,
        }
    }

    pub const fn is_removal(self) -> bool {
        matches!(self, Self::Remove(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacency_integrity_updates_only_the_edited_edges() {
        let commands = [
            crate::UiMountedPaintCommandIdentity::filled_rect(&mechanic(1)),
            crate::UiMountedPaintCommandIdentity::filled_rect(&mechanic(2)),
            crate::UiMountedPaintCommandIdentity::filled_rect(&mechanic(3)),
        ]
        .map(UiMountedPaintOrderIdentity::for_command);
        let initial = UiMountedPaintOrderIntegrity::for_order(&commands);
        let removed = initial
            .remove_edge(Some(commands[0]), commands[1], Some(commands[2]))
            .unwrap();
        assert_eq!(
            removed,
            UiMountedPaintOrderIntegrity::for_order(&[commands[0], commands[2]])
        );
        let restored = removed
            .insert_edge(Some(commands[0]), commands[1], Some(commands[2]))
            .unwrap();
        assert_eq!(restored, initial);
    }

    fn mechanic(slot: u64) -> crate::UiMountedFilledRectMechanic {
        use crate::{
            UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedCanonicalBoxInput,
            UiMountedCoordinateSpace, UiMountedFilledRectCompletionInput, UiMountedFrameIdentity,
            UiMountedInstanceIdentity, UiMountedNodeReceiptIssuer, UiMountedRgba8,
            UiMountedTransformProjection, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
        };
        let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
        let instance = UiMountedInstanceIdentity::mint_unbound().unwrap();
        let bounds = UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
            x: slot as f32,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            coordinate_space: UiMountedCoordinateSpace::HostSurface,
        })
        .unwrap();
        crate::UiMountedFilledRectMechanic::complete_from_runtime_mounting(
            UiMountedFilledRectCompletionInput {
                frame,
                surface: UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
                binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
                mounted_instance: instance,
                node_receipt: UiMountedNodeReceiptIssuer::mint_for(frame)
                    .unwrap()
                    .receipt_for(instance),
                allocation_basis: UiMountedAllocationBasis::new(
                    1,
                    1,
                    1,
                    UiMountedTransformProjection::Identity,
                ),
                bounds,
                color: UiMountedRgba8::new(1, 2, 3, 255),
                layer_semantic_order: 0,
                clip_bounds: bounds,
            },
        )
        .unwrap()
    }
}
