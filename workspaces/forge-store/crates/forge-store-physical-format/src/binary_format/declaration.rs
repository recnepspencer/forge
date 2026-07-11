use super::alignment::expected_alignment_for_site;
use super::field_widths::expected_width_for_kind;
use crate::{
    PhysicalAlignmentClass, PhysicalAlignmentSite, PhysicalBinaryFormatError, PhysicalByteOrder,
    PhysicalByteOrderDeclaration, PhysicalFieldWidth, PhysicalFieldWidthKind, PhysicalFormatMagic,
    PhysicalFormatVersion, PhysicalForwardCompatibilityDeclaration,
    PhysicalForwardCompatibilityPolicy, PhysicalPageSizeClass, PhysicalReservedFieldPolicy,
    PhysicalReservedFieldPolicyDeclaration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFormatAuthoritySource {
    ExplicitStoreLaw,
    SerdeMapOrder,
    RustStructLayout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFormatIdentity {
    magic: PhysicalFormatMagic,
    version: PhysicalFormatVersion,
    byte_order: PhysicalByteOrder,
    page_size: PhysicalPageSizeClass,
}

impl PhysicalFormatIdentity {
    pub const fn magic(&self) -> PhysicalFormatMagic {
        self.magic
    }

    pub const fn version(&self) -> PhysicalFormatVersion {
        self.version
    }

    pub const fn byte_order(&self) -> PhysicalByteOrder {
        self.byte_order
    }

    pub const fn page_size(&self) -> PhysicalPageSizeClass {
        self.page_size
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalFormatDeclaration {
    magic: PhysicalFormatMagic,
    version: PhysicalFormatVersion,
    byte_order: PhysicalByteOrder,
    field_widths: Vec<PhysicalFieldWidth>,
    page_size: PhysicalPageSizeClass,
    alignments: Vec<PhysicalAlignmentClass>,
    reserved_field_policy: PhysicalReservedFieldPolicy,
    forward_compatibility: PhysicalForwardCompatibilityPolicy,
}

impl PhysicalFormatDeclaration {
    pub fn builder() -> PhysicalFormatDeclarationBuilder {
        PhysicalFormatDeclarationBuilder::new()
    }

    pub fn physical_format_canonical() -> Result<Self, PhysicalBinaryFormatError> {
        Self::builder()
            .magic(PhysicalFormatMagic::store_format_magic())
            .version(PhysicalFormatVersion::initial_format_version())
            .byte_order(PhysicalByteOrder::LittleEndian)
            .field_width(PhysicalFieldWidth::segment_id_u64())
            .field_width(PhysicalFieldWidth::page_id_u64())
            .field_width(PhysicalFieldWidth::generation_u64())
            .field_width(PhysicalFieldWidth::header_length_u16())
            .field_width(PhysicalFieldWidth::payload_length_u32())
            .page_size(PhysicalPageSizeClass::KiB16)
            .alignment(PhysicalAlignmentClass::page_start_4k())
            .alignment(PhysicalAlignmentClass::frame_start_8())
            .alignment(PhysicalAlignmentClass::slot_directory_offset_8())
            .alignment(PhysicalAlignmentClass::extent_start_4k())
            .alignment(PhysicalAlignmentClass::manifest_record_8())
            .reserved_field_policy(PhysicalReservedFieldPolicy::zeroed_and_preserved())
            .forward_compatibility(PhysicalForwardCompatibilityPolicy::reject_unknown_kind())
            .define()
    }

    pub fn identity(&self) -> PhysicalFormatIdentity {
        PhysicalFormatIdentity {
            magic: self.magic,
            version: self.version,
            byte_order: self.byte_order,
            page_size: self.page_size,
        }
    }

    pub const fn byte_order(&self) -> PhysicalByteOrder {
        self.byte_order
    }

    pub fn field_widths(&self) -> &[PhysicalFieldWidth] {
        &self.field_widths
    }

    pub const fn page_size(&self) -> PhysicalPageSizeClass {
        self.page_size
    }

    pub fn alignments(&self) -> &[PhysicalAlignmentClass] {
        &self.alignments
    }

    pub const fn reserved_field_policy(&self) -> PhysicalReservedFieldPolicy {
        self.reserved_field_policy
    }

    pub const fn forward_compatibility(&self) -> PhysicalForwardCompatibilityPolicy {
        self.forward_compatibility
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalFormatDeclarationBuilder {
    authority_source: PhysicalFormatAuthoritySource,
    magic: Option<PhysicalFormatMagic>,
    version: Option<PhysicalFormatVersion>,
    byte_order: Option<PhysicalByteOrderDeclaration>,
    field_widths: Vec<PhysicalFieldWidth>,
    page_size: Option<PhysicalPageSizeClass>,
    alignments: Vec<PhysicalAlignmentClass>,
    reserved_field_policy: Option<PhysicalReservedFieldPolicyDeclaration>,
    forward_compatibility: Option<PhysicalForwardCompatibilityDeclaration>,
}

impl PhysicalFormatDeclarationBuilder {
    pub fn new() -> Self {
        Self {
            authority_source: PhysicalFormatAuthoritySource::ExplicitStoreLaw,
            magic: None,
            version: None,
            byte_order: None,
            field_widths: Vec::new(),
            page_size: None,
            alignments: Vec::new(),
            reserved_field_policy: None,
            forward_compatibility: None,
        }
    }

    pub const fn authority_source(mut self, source: PhysicalFormatAuthoritySource) -> Self {
        self.authority_source = source;
        self
    }

    pub const fn magic(mut self, magic: PhysicalFormatMagic) -> Self {
        self.magic = Some(magic);
        self
    }

    pub const fn version(mut self, version: PhysicalFormatVersion) -> Self {
        self.version = Some(version);
        self
    }

    pub fn byte_order(self, byte_order: PhysicalByteOrder) -> Self {
        self.byte_order_declaration(byte_order.into())
    }

    pub const fn byte_order_declaration(
        mut self,
        declaration: PhysicalByteOrderDeclaration,
    ) -> Self {
        self.byte_order = Some(declaration);
        self
    }

    pub fn field_width(mut self, width: PhysicalFieldWidth) -> Self {
        self.field_widths.push(width);
        self
    }

    pub const fn page_size(mut self, page_size: PhysicalPageSizeClass) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn alignment(mut self, alignment: PhysicalAlignmentClass) -> Self {
        self.alignments.push(alignment);
        self
    }

    pub fn reserved_field_policy(self, policy: PhysicalReservedFieldPolicy) -> Self {
        self.reserved_field_policy_declaration(policy.into())
    }

    pub const fn reserved_field_policy_declaration(
        mut self,
        declaration: PhysicalReservedFieldPolicyDeclaration,
    ) -> Self {
        self.reserved_field_policy = Some(declaration);
        self
    }

    pub fn forward_compatibility(self, policy: PhysicalForwardCompatibilityPolicy) -> Self {
        self.forward_compatibility_declaration(policy.into())
    }

    pub const fn forward_compatibility_declaration(
        mut self,
        declaration: PhysicalForwardCompatibilityDeclaration,
    ) -> Self {
        self.forward_compatibility = Some(declaration);
        self
    }

    pub fn define(self) -> Result<PhysicalFormatDeclaration, PhysicalBinaryFormatError> {
        reject_non_store_authority(self.authority_source)?;
        let magic = self.magic.ok_or(PhysicalBinaryFormatError::MissingMagic)?;
        if magic != PhysicalFormatMagic::store_format_magic() {
            return Err(PhysicalBinaryFormatError::MagicMismatch);
        }
        let version = self
            .version
            .ok_or(PhysicalBinaryFormatError::MissingVersion)?;
        if version != PhysicalFormatVersion::initial_format_version() {
            return Err(PhysicalBinaryFormatError::VersionMismatch);
        }
        let byte_order = required_byte_order(self.byte_order)?;
        let page_size = self
            .page_size
            .ok_or(PhysicalBinaryFormatError::MissingPageSize)?;
        required_field_widths(&self.field_widths)?;
        required_alignments(&self.alignments)?;
        let reserved_field_policy = required_reserved_policy(self.reserved_field_policy)?;
        let forward_compatibility = required_forward_policy(self.forward_compatibility)?;

        Ok(PhysicalFormatDeclaration {
            magic,
            version,
            byte_order,
            field_widths: self.field_widths,
            page_size,
            alignments: self.alignments,
            reserved_field_policy,
            forward_compatibility,
        })
    }
}

impl Default for PhysicalFormatDeclarationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn reject_non_store_authority(
    source: PhysicalFormatAuthoritySource,
) -> Result<(), PhysicalBinaryFormatError> {
    match source {
        PhysicalFormatAuthoritySource::ExplicitStoreLaw => Ok(()),
        PhysicalFormatAuthoritySource::SerdeMapOrder => {
            Err(PhysicalBinaryFormatError::SerdeOrderRejected)
        }
        PhysicalFormatAuthoritySource::RustStructLayout => {
            Err(PhysicalBinaryFormatError::RustLayoutRejected)
        }
    }
}

fn required_byte_order(
    declaration: Option<PhysicalByteOrderDeclaration>,
) -> Result<PhysicalByteOrder, PhysicalBinaryFormatError> {
    match declaration.ok_or(PhysicalBinaryFormatError::MissingByteOrder)? {
        PhysicalByteOrderDeclaration::Explicit(PhysicalByteOrder::LittleEndian) => {
            Ok(PhysicalByteOrder::LittleEndian)
        }
        PhysicalByteOrderDeclaration::HostEndian => {
            Err(PhysicalBinaryFormatError::HostEndianRejected)
        }
    }
}

fn required_reserved_policy(
    declaration: Option<PhysicalReservedFieldPolicyDeclaration>,
) -> Result<PhysicalReservedFieldPolicy, PhysicalBinaryFormatError> {
    match declaration.ok_or(PhysicalBinaryFormatError::MissingReservedFieldPolicy)? {
        PhysicalReservedFieldPolicyDeclaration::Known(policy) => Ok(policy),
        PhysicalReservedFieldPolicyDeclaration::Unknown => {
            Err(PhysicalBinaryFormatError::UnknownReservedFieldPolicy)
        }
    }
}

fn required_forward_policy(
    declaration: Option<PhysicalForwardCompatibilityDeclaration>,
) -> Result<PhysicalForwardCompatibilityPolicy, PhysicalBinaryFormatError> {
    match declaration.ok_or(PhysicalBinaryFormatError::MissingForwardCompatibilityPolicy)? {
        PhysicalForwardCompatibilityDeclaration::Known(
            PhysicalForwardCompatibilityPolicy::RejectUnknownKind,
        ) => Ok(PhysicalForwardCompatibilityPolicy::RejectUnknownKind),
        PhysicalForwardCompatibilityDeclaration::Known(
            PhysicalForwardCompatibilityPolicy::PreserveUnknownBytes,
        ) => Err(PhysicalBinaryFormatError::ForwardPreservationNotAdmission),
        PhysicalForwardCompatibilityDeclaration::Known(
            PhysicalForwardCompatibilityPolicy::MigrationReserved,
        ) => Err(PhysicalBinaryFormatError::ForwardMigrationNotAdmission),
        PhysicalForwardCompatibilityDeclaration::Unsupported => {
            Err(PhysicalBinaryFormatError::UnsupportedForwardCompatibilityPolicy)
        }
    }
}

fn required_field_widths(widths: &[PhysicalFieldWidth]) -> Result<(), PhysicalBinaryFormatError> {
    for kind in PhysicalFieldWidthKind::required_for_physical_format() {
        let width =
            find_width(kind, widths).ok_or(PhysicalBinaryFormatError::MissingFieldWidth(kind))?;
        if width != expected_width_for_kind(kind) {
            return Err(PhysicalBinaryFormatError::FieldWidthMismatch(kind));
        }
    }
    Ok(())
}

pub(crate) fn find_width(
    kind: PhysicalFieldWidthKind,
    widths: &[PhysicalFieldWidth],
) -> Option<PhysicalFieldWidth> {
    widths.iter().copied().find(|width| width.kind() == kind)
}

fn required_alignments(
    alignments: &[PhysicalAlignmentClass],
) -> Result<(), PhysicalBinaryFormatError> {
    for site in PhysicalAlignmentSite::required_for_physical_format() {
        let alignment = find_alignment(site, alignments)
            .ok_or(PhysicalBinaryFormatError::MissingAlignment(site))?;
        if alignment != expected_alignment_for_site(site) {
            return Err(PhysicalBinaryFormatError::AlignmentMismatch(site));
        }
    }
    Ok(())
}

pub(crate) fn find_alignment(
    site: PhysicalAlignmentSite,
    alignments: &[PhysicalAlignmentClass],
) -> Option<PhysicalAlignmentClass> {
    alignments
        .iter()
        .copied()
        .find(|alignment| alignment.site() == site)
}
