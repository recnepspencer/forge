use crate::record_framing::{decode_durable_frame, encode_durable_frame};
use crate::{
    DurableFrameDenial, DurableFrameKind, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordFormatDeclaration, PhysicalSegmentId,
    RecordSegmentPageManifestEntry,
};

const BLOCK_PREFIX_BYTES: usize = 40;
const REFERENCE_BYTES: usize = 56;
const LEAF_ENTRY_BYTES: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentPageKey {
    segment: PhysicalSegmentId,
    page: PhysicalPageId,
}

impl SegmentPageKey {
    pub const fn new(segment: PhysicalSegmentId, page: PhysicalPageId) -> Self {
        Self { segment, page }
    }

    pub const fn segment(self) -> PhysicalSegmentId {
        self.segment
    }

    pub const fn page(self) -> PhysicalPageId {
        self.page
    }
}

impl From<RecordSegmentPageManifestEntry> for SegmentPageKey {
    fn from(entry: RecordSegmentPageManifestEntry) -> Self {
        Self::new(entry.page_cell().segment_id(), entry.page())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentManifestBlockReference {
    generation: u64,
    block: u64,
    level: u16,
    checksum: u32,
    first: SegmentPageKey,
    last: SegmentPageKey,
}

impl SegmentManifestBlockReference {
    pub fn new(
        generation: u64,
        block: u64,
        level: u16,
        checksum: u32,
        first: SegmentPageKey,
        last: SegmentPageKey,
    ) -> Option<Self> {
        (generation != 0 && block != 0 && first <= last).then_some(Self {
            generation,
            block,
            level,
            checksum,
            first,
            last,
        })
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn block(self) -> u64 {
        self.block
    }

    pub const fn level(self) -> u16 {
        self.level
    }
    pub const fn checksum(self) -> u32 {
        self.checksum
    }

    pub const fn first(self) -> SegmentPageKey {
        self.first
    }

    pub const fn last(self) -> SegmentPageKey {
        self.last
    }

    pub fn contains(self, key: SegmentPageKey) -> bool {
        self.first <= key && key <= self.last
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalSegmentMembershipBlock {
    Leaf {
        tree_identity: u64,
        generation: u64,
        block: u64,
        entries: Vec<RecordSegmentPageManifestEntry>,
    },
    Branch {
        tree_identity: u64,
        generation: u64,
        block: u64,
        level: u16,
        children: Vec<SegmentManifestBlockReference>,
    },
}

impl PhysicalSegmentMembershipBlock {
    pub fn leaf(
        tree_identity: u64,
        generation: u64,
        block: u64,
        entries: Vec<RecordSegmentPageManifestEntry>,
        capacity: u16,
    ) -> Option<Self> {
        (tree_identity != 0
            && generation != 0
            && block != 0
            && !entries.is_empty()
            && entries.len() <= usize::from(capacity)
            && entries
                .windows(2)
                .all(|pair| SegmentPageKey::from(pair[0]) < SegmentPageKey::from(pair[1])))
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
        children: Vec<SegmentManifestBlockReference>,
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

    pub fn entries(&self) -> Option<&[RecordSegmentPageManifestEntry]> {
        match self {
            Self::Leaf { entries, .. } => Some(entries),
            Self::Branch { .. } => None,
        }
    }

    pub fn children(&self) -> Option<&[SegmentManifestBlockReference]> {
        match self {
            Self::Branch { children, .. } => Some(children),
            Self::Leaf { .. } => None,
        }
    }

    pub fn reference(&self, checksum: u32) -> SegmentManifestBlockReference {
        let (first, last) = match self {
            Self::Leaf { entries, .. } => (
                SegmentPageKey::from(*entries.first().expect("validated leaf")),
                SegmentPageKey::from(*entries.last().expect("validated leaf")),
            ),
            Self::Branch { children, .. } => (
                children.first().expect("validated branch").first(),
                children.last().expect("validated branch").last(),
            ),
        };
        SegmentManifestBlockReference::new(
            self.generation(),
            self.block(),
            self.level(),
            checksum,
            first,
            last,
        )
        .expect("validated membership block")
    }

    pub fn encode(&self, format: PhysicalRecordFormatDeclaration) -> Vec<u8> {
        let (kind, count, entry_bytes) = match self {
            Self::Leaf { entries, .. } => (1_u8, entries.len(), LEAF_ENTRY_BYTES),
            Self::Branch { children, .. } => (2_u8, children.len(), REFERENCE_BYTES),
        };
        let mut payload = vec![0_u8; BLOCK_PREFIX_BYTES + count * entry_bytes];
        payload[..8].copy_from_slice(&self.tree_identity().to_le_bytes());
        payload[8..16].copy_from_slice(&self.block().to_le_bytes());
        payload[16..18].copy_from_slice(&self.level().to_le_bytes());
        payload[18..20].copy_from_slice(&(count as u16).to_le_bytes());
        payload[20] = kind;
        payload[24..32].copy_from_slice(&self.generation().to_le_bytes());
        match self {
            Self::Leaf { entries, .. } => entries.iter().enumerate().for_each(|(index, entry)| {
                encode_entry(
                    &mut payload[BLOCK_PREFIX_BYTES + index * entry_bytes..],
                    *entry,
                );
            }),
            Self::Branch { children, .. } => {
                children.iter().enumerate().for_each(|(index, child)| {
                    encode_reference(
                        &mut payload[BLOCK_PREFIX_BYTES + index * entry_bytes..],
                        *child,
                    );
                });
            }
        }
        encode_durable_frame(
            DurableFrameKind::SegmentMembershipBlock,
            format,
            self.block(),
            &payload,
        )
    }

    pub fn decode(
        bytes: &[u8],
        capacity: u16,
    ) -> Result<(Self, PhysicalRecordFormatDeclaration), SegmentMembershipBlockDenial> {
        let (format, frame) = decode_durable_frame(bytes, DurableFrameKind::SegmentMembershipBlock)
            .map_err(SegmentMembershipBlockDenial::Frame)?;
        if frame.payload.len() < BLOCK_PREFIX_BYTES
            || frame.payload[21..24] != [0; 3]
            || frame.payload[32..40] != [0; 8]
        {
            return Err(SegmentMembershipBlockDenial::MalformedPrefix);
        }
        let tree_identity = u64::from_le_bytes(frame.payload[..8].try_into().unwrap());
        let block = u64::from_le_bytes(frame.payload[8..16].try_into().unwrap());
        let level = u16::from_le_bytes(frame.payload[16..18].try_into().unwrap());
        let count = u16::from_le_bytes(frame.payload[18..20].try_into().unwrap());
        let generation = u64::from_le_bytes(frame.payload[24..32].try_into().unwrap());
        if tree_identity == 0
            || generation == 0
            || block == 0
            || block != frame.identity
            || count == 0
            || count > capacity
        {
            return Err(SegmentMembershipBlockDenial::IdentityOrCapacity);
        }
        let entry_bytes = match frame.payload[20] {
            1 if level == 0 => LEAF_ENTRY_BYTES,
            2 if level != 0 => REFERENCE_BYTES,
            _ => return Err(SegmentMembershipBlockDenial::LevelOrKind),
        };
        if frame.payload.len() != BLOCK_PREFIX_BYTES + usize::from(count) * entry_bytes {
            return Err(SegmentMembershipBlockDenial::MalformedLength);
        }
        let body = &frame.payload[BLOCK_PREFIX_BYTES..];
        let decoded = if level == 0 {
            let entries = body
                .chunks_exact(entry_bytes)
                .map(decode_entry)
                .collect::<Option<Vec<_>>>()
                .ok_or(SegmentMembershipBlockDenial::InvalidEntry)?;
            Self::leaf(tree_identity, generation, block, entries, capacity)
        } else {
            let children = body
                .chunks_exact(entry_bytes)
                .map(decode_reference)
                .collect::<Option<Vec<_>>>()
                .ok_or(SegmentMembershipBlockDenial::InvalidReference)?;
            Self::branch(tree_identity, generation, block, level, children, capacity)
        };
        decoded
            .map(|block| (block, format))
            .ok_or(SegmentMembershipBlockDenial::CanonicalOrder)
    }
}

fn encode_entry(target: &mut [u8], entry: RecordSegmentPageManifestEntry) {
    target[..8].copy_from_slice(&entry.page_cell().segment_id().get().to_le_bytes());
    target[8..16].copy_from_slice(&entry.page().get().to_le_bytes());
    target[16..24].copy_from_slice(&entry.page_generation().to_le_bytes());
    target[24..32].copy_from_slice(&entry.data_generation().to_le_bytes());
    target[32..36].copy_from_slice(&entry.data_page_count().to_le_bytes());
    target[36..40].copy_from_slice(&entry.frame_index().to_le_bytes());
}

fn decode_entry(bytes: &[u8]) -> Option<RecordSegmentPageManifestEntry> {
    let segment =
        PhysicalSegmentId::from_raw(u64::from_le_bytes(bytes[..8].try_into().ok()?)).ok()?;
    let page = PhysicalPageId::from_raw(u64::from_le_bytes(bytes[8..16].try_into().ok()?)).ok()?;
    let page_generation =
        PhysicalGeneration::from_raw(u64::from_le_bytes(bytes[16..24].try_into().ok()?)).ok()?;
    let data_generation =
        PhysicalGeneration::from_raw(u64::from_le_bytes(bytes[24..32].try_into().ok()?)).ok()?;
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    RecordSegmentPageManifestEntry::new(
        authority
            .page_cell(segment, page)
            .with_page_generation(page_generation),
        authority
            .segment_cell(segment)
            .with_segment_generation(data_generation),
        u32::from_le_bytes(bytes[32..36].try_into().ok()?),
        u32::from_le_bytes(bytes[36..40].try_into().ok()?),
    )
}

pub(crate) fn encode_reference(target: &mut [u8], reference: SegmentManifestBlockReference) {
    target[..8].copy_from_slice(&reference.generation().to_le_bytes());
    target[8..16].copy_from_slice(&reference.block().to_le_bytes());
    target[16..18].copy_from_slice(&reference.level().to_le_bytes());
    target[20..24].copy_from_slice(&reference.checksum().to_le_bytes());
    encode_key(&mut target[24..40], reference.first());
    encode_key(&mut target[40..56], reference.last());
}

pub(crate) fn decode_reference(bytes: &[u8]) -> Option<SegmentManifestBlockReference> {
    if bytes[18..20] != [0; 2] {
        return None;
    }
    SegmentManifestBlockReference::new(
        u64::from_le_bytes(bytes[..8].try_into().ok()?),
        u64::from_le_bytes(bytes[8..16].try_into().ok()?),
        u16::from_le_bytes(bytes[16..18].try_into().ok()?),
        u32::from_le_bytes(bytes[20..24].try_into().ok()?),
        decode_key(&bytes[24..40])?,
        decode_key(&bytes[40..56])?,
    )
}

fn encode_key(target: &mut [u8], key: SegmentPageKey) {
    target[..8].copy_from_slice(&key.segment().get().to_le_bytes());
    target[8..16].copy_from_slice(&key.page().get().to_le_bytes());
}

fn decode_key(bytes: &[u8]) -> Option<SegmentPageKey> {
    Some(SegmentPageKey::new(
        PhysicalSegmentId::from_raw(u64::from_le_bytes(bytes[..8].try_into().ok()?)).ok()?,
        PhysicalPageId::from_raw(u64::from_le_bytes(bytes[8..16].try_into().ok()?)).ok()?,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentMembershipBlockDenial {
    Frame(DurableFrameDenial),
    MalformedPrefix,
    IdentityOrCapacity,
    LevelOrKind,
    MalformedLength,
    InvalidEntry,
    InvalidReference,
    CanonicalOrder,
}
