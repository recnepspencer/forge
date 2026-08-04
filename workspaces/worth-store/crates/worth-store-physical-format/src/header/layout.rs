use crate::{
    PhysicalFrameKind, PhysicalGeneration, PhysicalHeaderKind, PhysicalHeaderReservedFields,
    PhysicalPageKind, PhysicalPublicationState,
};

pub const PHYSICAL_HEADER_LENGTH: u16 = 48;

pub(crate) const OWNER_PRIMARY_OFFSET: usize = 18;
pub(crate) const OWNER_SECONDARY_OFFSET: usize = 26;
pub(crate) const OWNER_TERTIARY_OFFSET: usize = 34;
pub(crate) const RESERVED_CHECKSUM_OFFSET: usize = 36;
pub(crate) const RESERVED_TAIL_OFFSET: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalPageHeader {
    kind: PhysicalPageKind,
    generation: PhysicalGeneration,
    publication: PhysicalPublicationState,
    payload_length: u32,
    reserved_fields: PhysicalHeaderReservedFields,
}

impl PhysicalPageHeader {
    pub(crate) const fn new(
        kind: PhysicalPageKind,
        generation: PhysicalGeneration,
        publication: PhysicalPublicationState,
        payload_length: u32,
        reserved_fields: PhysicalHeaderReservedFields,
    ) -> Self {
        Self {
            kind,
            generation,
            publication,
            payload_length,
            reserved_fields,
        }
    }

    pub const fn kind(self) -> PhysicalPageKind {
        self.kind
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn publication(self) -> PhysicalPublicationState {
        self.publication
    }

    pub const fn payload_length(self) -> u32 {
        self.payload_length
    }

    pub const fn reserved_fields(self) -> PhysicalHeaderReservedFields {
        self.reserved_fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFrameHeader {
    kind: PhysicalFrameKind,
    generation: PhysicalGeneration,
    publication: PhysicalPublicationState,
    payload_length: u32,
    reserved_fields: PhysicalHeaderReservedFields,
}

impl PhysicalFrameHeader {
    pub(crate) const fn new(
        kind: PhysicalFrameKind,
        generation: PhysicalGeneration,
        publication: PhysicalPublicationState,
        payload_length: u32,
        reserved_fields: PhysicalHeaderReservedFields,
    ) -> Self {
        Self {
            kind,
            generation,
            publication,
            payload_length,
            reserved_fields,
        }
    }

    pub const fn kind(self) -> PhysicalFrameKind {
        self.kind
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn publication(self) -> PhysicalPublicationState {
        self.publication
    }

    pub const fn payload_length(self) -> u32 {
        self.payload_length
    }

    pub const fn reserved_fields(self) -> PhysicalHeaderReservedFields {
        self.reserved_fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDecodedHeader {
    Page(PhysicalPageHeader),
    Frame(PhysicalFrameHeader),
}

impl PhysicalDecodedHeader {
    pub const fn kind(self) -> PhysicalHeaderKind {
        match self {
            Self::Page(header) => PhysicalHeaderKind::Page(header.kind()),
            Self::Frame(header) => PhysicalHeaderKind::Frame(header.kind()),
        }
    }

    pub const fn payload_length(self) -> u32 {
        match self {
            Self::Page(header) => header.payload_length(),
            Self::Frame(header) => header.payload_length(),
        }
    }

    pub const fn generation(self) -> PhysicalGeneration {
        match self {
            Self::Page(header) => header.generation(),
            Self::Frame(header) => header.generation(),
        }
    }

    pub const fn publication(self) -> PhysicalPublicationState {
        match self {
            Self::Page(header) => header.publication(),
            Self::Frame(header) => header.publication(),
        }
    }

    pub const fn reserved_fields(self) -> PhysicalHeaderReservedFields {
        match self {
            Self::Page(header) => header.reserved_fields(),
            Self::Frame(header) => header.reserved_fields(),
        }
    }
}
