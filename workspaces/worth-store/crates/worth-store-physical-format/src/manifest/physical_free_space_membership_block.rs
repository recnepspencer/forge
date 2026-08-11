use crate::record_framing::{decode_durable_frame, encode_durable_frame};
use crate::{
    DurableFrameKind, PhysicalRecordFormatDeclaration, RecordAllocationClass,
    RecordFreeSpaceManifestEntry,
};

use super::free_space_routing::{
    decode_reference, encode_reference, FreeSpaceBlockReference, FreeSpaceKey,
    FreeSpaceRoutingDenial,
};

const BLOCK_PREFIX_BYTES: usize = 40;
const REFERENCE_BYTES: usize = 56;
const ENTRY_BYTES: usize = 40;

#[cfg(test)]
#[path = "physical_free_space_membership_block/bounded_decode_tests.rs"]
mod bounded_decode_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceMembershipBlockDecodeLimits {
    pub leaf_entries: u64,
    pub branch_children: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedFreeSpaceMembershipBlockDecodeDenial {
    Format(FreeSpaceRoutingDenial),
    LeafEntries { observed: u64, admitted: u64 },
    BranchChildren { observed: u64, admitted: u64 },
}

impl From<FreeSpaceRoutingDenial> for BoundedFreeSpaceMembershipBlockDecodeDenial {
    fn from(value: FreeSpaceRoutingDenial) -> Self {
        Self::Format(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalFreeSpaceMembershipBlock {
    Leaf {
        tree_identity: u64,
        generation: u64,
        block: u64,
        entries: Vec<RecordFreeSpaceManifestEntry>,
    },
    Branch {
        tree_identity: u64,
        generation: u64,
        block: u64,
        level: u16,
        children: Vec<FreeSpaceBlockReference>,
    },
}

impl PhysicalFreeSpaceMembershipBlock {
    pub fn leaf(
        tree_identity: u64,
        generation: u64,
        block: u64,
        entries: Vec<RecordFreeSpaceManifestEntry>,
        capacity: u16,
    ) -> Option<Self> {
        (tree_identity != 0
            && generation != 0
            && block != 0
            && !entries.is_empty()
            && entries.len() <= usize::from(capacity)
            && entries
                .windows(2)
                .all(|pair| FreeSpaceKey::from(pair[0]) < FreeSpaceKey::from(pair[1])))
        .then_some(Self::Leaf {
            tree_identity,
            generation,
            block,
            entries,
        })
    }
    pub fn branch(
        tree_identity: u64,
        generation: u64,
        block: u64,
        level: u16,
        children: Vec<FreeSpaceBlockReference>,
        capacity: u16,
    ) -> Option<Self> {
        (tree_identity != 0
            && generation != 0
            && block != 0
            && level != 0
            && !children.is_empty()
            && children.len() <= usize::from(capacity)
            && children.iter().all(|child| {
                child.level().checked_add(1) == Some(level) && child.generation() <= generation
            })
            && children
                .windows(2)
                .all(|pair| pair[0].last() < pair[1].first()))
        .then_some(Self::Branch {
            tree_identity,
            generation,
            block,
            level,
            children,
        })
    }
    pub const fn tree_identity(&self) -> u64 {
        match self {
            Self::Leaf { tree_identity, .. } | Self::Branch { tree_identity, .. } => *tree_identity,
        }
    }
    pub const fn generation(&self) -> u64 {
        match self {
            Self::Leaf { generation, .. } | Self::Branch { generation, .. } => *generation,
        }
    }
    pub const fn block(&self) -> u64 {
        match self {
            Self::Leaf { block, .. } | Self::Branch { block, .. } => *block,
        }
    }
    pub const fn level(&self) -> u16 {
        match self {
            Self::Leaf { .. } => 0,
            Self::Branch { level, .. } => *level,
        }
    }
    pub fn reference(&self, checksum: u32) -> FreeSpaceBlockReference {
        let (first, last) = match self {
            Self::Leaf { entries, .. } => (
                FreeSpaceKey::from(*entries.first().expect("validated leaf")),
                FreeSpaceKey::from(*entries.last().expect("validated leaf")),
            ),
            Self::Branch { children, .. } => (
                children.first().expect("validated branch").first(),
                children.last().expect("validated branch").last(),
            ),
        };
        FreeSpaceBlockReference::new(
            self.generation(),
            self.block(),
            self.level(),
            checksum,
            first,
            last,
        )
        .expect("validated block")
    }
    pub fn encode(&self, format: PhysicalRecordFormatDeclaration) -> Vec<u8> {
        let (kind, count, width) = match self {
            Self::Leaf { entries, .. } => (1, entries.len(), ENTRY_BYTES),
            Self::Branch { children, .. } => (2, children.len(), REFERENCE_BYTES),
        };
        let mut payload = vec![0_u8; BLOCK_PREFIX_BYTES + count * width];
        payload[..8].copy_from_slice(&self.tree_identity().to_le_bytes());
        payload[8..16].copy_from_slice(&self.block().to_le_bytes());
        payload[16..18].copy_from_slice(&self.level().to_le_bytes());
        payload[18..20].copy_from_slice(&(count as u16).to_le_bytes());
        payload[20] = kind;
        payload[24..32].copy_from_slice(&self.generation().to_le_bytes());
        match self {
            Self::Leaf { entries, .. } => entries.iter().enumerate().for_each(|(index, entry)| {
                encode_entry(&mut payload[BLOCK_PREFIX_BYTES + index * width..], *entry);
            }),
            Self::Branch { children, .. } => {
                children.iter().enumerate().for_each(|(index, child)| {
                    encode_reference(&mut payload[BLOCK_PREFIX_BYTES + index * width..], *child);
                })
            }
        }
        encode_durable_frame(
            DurableFrameKind::FreeSpaceMembershipBlock,
            format,
            self.block(),
            &payload,
        )
    }
    pub fn decode(
        bytes: &[u8],
        capacity: u16,
    ) -> Result<(Self, PhysicalRecordFormatDeclaration), FreeSpaceRoutingDenial> {
        match Self::decode_bounded(
            bytes,
            capacity,
            FreeSpaceMembershipBlockDecodeLimits {
                leaf_entries: u64::MAX,
                branch_children: u64::MAX,
            },
        ) {
            Ok(decoded) => Ok(decoded),
            Err(BoundedFreeSpaceMembershipBlockDecodeDenial::Format(denial)) => Err(denial),
            Err(
                BoundedFreeSpaceMembershipBlockDecodeDenial::LeafEntries { .. }
                | BoundedFreeSpaceMembershipBlockDecodeDenial::BranchChildren { .. },
            ) => unreachable!("unbounded free-space decode cannot exceed cardinality"),
        }
    }

    pub fn decode_bounded(
        bytes: &[u8],
        capacity: u16,
        limits: FreeSpaceMembershipBlockDecodeLimits,
    ) -> Result<(Self, PhysicalRecordFormatDeclaration), BoundedFreeSpaceMembershipBlockDecodeDenial>
    {
        let (format, frame) =
            decode_durable_frame(bytes, DurableFrameKind::FreeSpaceMembershipBlock)
                .map_err(FreeSpaceRoutingDenial::Frame)?;
        if frame.payload.len() < BLOCK_PREFIX_BYTES
            || frame.payload[21..24] != [0; 3]
            || frame.payload[32..40] != [0; 8]
        {
            return Err(FreeSpaceRoutingDenial::Malformed.into());
        }
        let tree_identity = read_u64(frame.payload, 0);
        let block = read_u64(frame.payload, 8);
        let level = u16::from_le_bytes(frame.payload[16..18].try_into().unwrap());
        let count = u16::from_le_bytes(frame.payload[18..20].try_into().unwrap());
        let generation = read_u64(frame.payload, 24);
        if tree_identity == 0
            || generation == 0
            || block != frame.identity
            || count == 0
            || count > capacity
        {
            return Err(FreeSpaceRoutingDenial::IdentityOrCapacity.into());
        }
        let width = match frame.payload[20] {
            1 if level == 0 => ENTRY_BYTES,
            2 if level != 0 => REFERENCE_BYTES,
            _ => return Err(FreeSpaceRoutingDenial::Malformed.into()),
        };
        if frame.payload.len() != BLOCK_PREFIX_BYTES + usize::from(count) * width {
            return Err(FreeSpaceRoutingDenial::Malformed.into());
        }
        let observed = u64::from(count);
        if level == 0 && observed > limits.leaf_entries {
            return Err(BoundedFreeSpaceMembershipBlockDecodeDenial::LeafEntries {
                observed,
                admitted: limits.leaf_entries,
            });
        }
        if level != 0 && observed > limits.branch_children {
            return Err(
                BoundedFreeSpaceMembershipBlockDecodeDenial::BranchChildren {
                    observed,
                    admitted: limits.branch_children,
                },
            );
        }
        let body = &frame.payload[BLOCK_PREFIX_BYTES..];
        let decoded = if level == 0 {
            Self::leaf(
                tree_identity,
                generation,
                block,
                body.chunks_exact(width)
                    .map(decode_entry)
                    .collect::<Option<Vec<_>>>()
                    .ok_or(FreeSpaceRoutingDenial::Malformed)?,
                capacity,
            )
        } else {
            Self::branch(
                tree_identity,
                generation,
                block,
                level,
                body.chunks_exact(width)
                    .map(decode_reference)
                    .collect::<Option<Vec<_>>>()
                    .ok_or(FreeSpaceRoutingDenial::InvalidReference)?,
                capacity,
            )
        };
        decoded
            .map(|value| (value, format))
            .ok_or(FreeSpaceRoutingDenial::CanonicalOrder.into())
    }
    pub fn entries(&self) -> Option<&[RecordFreeSpaceManifestEntry]> {
        match self {
            Self::Leaf { entries, .. } => Some(entries),
            Self::Branch { .. } => None,
        }
    }
    pub fn children(&self) -> Option<&[FreeSpaceBlockReference]> {
        match self {
            Self::Branch { children, .. } => Some(children),
            Self::Leaf { .. } => None,
        }
    }
}

fn encode_entry(target: &mut [u8], entry: RecordFreeSpaceManifestEntry) {
    target[0] = entry.class() as u8;
    target[8..16].copy_from_slice(&entry.owner().to_le_bytes());
    target[16..24].copy_from_slice(&entry.first_unallocated().to_le_bytes());
    target[24..32].copy_from_slice(&entry.unallocated_count().to_le_bytes());
    target[32..40].copy_from_slice(&entry.generation().to_le_bytes());
}

fn decode_entry(bytes: &[u8]) -> Option<RecordFreeSpaceManifestEntry> {
    if bytes[1..8] != [0; 7] {
        return None;
    }
    let class = match bytes[0] {
        1 => RecordAllocationClass::InlinePage,
        2 => RecordAllocationClass::Extent,
        _ => return None,
    };
    RecordFreeSpaceManifestEntry::new(
        class,
        read_u64(bytes, 8),
        read_u64(bytes, 16),
        read_u64(bytes, 24),
        read_u64(bytes, 32),
    )
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}
