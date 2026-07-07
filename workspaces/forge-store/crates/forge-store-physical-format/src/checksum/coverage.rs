use crate::{
    ChecksumCompatibilityFieldPosture, ChecksumCoverageDisposition, ChecksumCoverageEncoding,
    ChecksumCoverageMapDenial, ChecksumCoverageRegion, ChecksumFieldHandling,
    ChecksumGenerationFieldPosture, ChecksumHeaderField, ChecksumLengthFieldPosture,
    ChecksumPaddingPosture, ChecksumPayloadRegion, ChecksumReservedFieldPosture,
    ChecksumUnknownFieldPosture, PhysicalFormatVersion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCoverageAuthoritySource {
    ExplicitStoreLaw,
    SerdeMapOrder,
    RustStructLayout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumCoverageMap {
    physical_format_version: PhysicalFormatVersion,
    covered_header_fields: Vec<ChecksumHeaderField>,
    excluded_header_fields: Vec<ChecksumHeaderField>,
    checksum_field_handling: ChecksumFieldHandling,
    mutable_publication_fields: Vec<ChecksumHeaderField>,
    reserved_fields: ChecksumReservedFieldPosture,
    generation_fields: ChecksumGenerationFieldPosture,
    length_fields: ChecksumLengthFieldPosture,
    payload_region: ChecksumPayloadRegion,
    padding_bytes: ChecksumPaddingPosture,
    compatibility_fields: ChecksumCompatibilityFieldPosture,
    unknown_field_posture: ChecksumUnknownFieldPosture,
    coverage_encoding: ChecksumCoverageEncoding,
}

impl ChecksumCoverageMap {
    pub fn builder(version: PhysicalFormatVersion) -> ChecksumCoverageMapBuilder {
        ChecksumCoverageMapBuilder::new(version)
    }

    pub fn s1_page_and_frame_crc32c() -> Result<Self, ChecksumCoverageMapDenial> {
        Self::builder(PhysicalFormatVersion::s1_initial())
            .covered_header_fields(s1_required_covered_header_fields())
            .excluded_header_fields([ChecksumHeaderField::ChecksumField])
            .checksum_field_handling(ChecksumFieldHandling::ExcludedDuringComputation)
            .mutable_publication_fields([ChecksumHeaderField::PublicationState])
            .reserved_fields(ChecksumReservedFieldPosture::CoveredAsZeroedAndPreserved)
            .generation_fields(ChecksumGenerationFieldPosture::CoveredAsPhysicalGeneration)
            .length_fields(ChecksumLengthFieldPosture::CoveredAsSerializedPayloadLength)
            .payload_region(ChecksumPayloadRegion::SerializedPayloadBytes)
            .padding_bytes(ChecksumPaddingPosture::ExcludedAndMustRemainZeroed)
            .compatibility_fields(ChecksumCompatibilityFieldPosture::CoveredAndDenyUnknown)
            .unknown_field_posture(ChecksumUnknownFieldPosture::DenyUntilReadmitted)
            .coverage_encoding(ChecksumCoverageEncoding::SerializedBytes)
            .define()
    }

    pub const fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.physical_format_version
    }

    pub fn covered_header_fields(&self) -> &[ChecksumHeaderField] {
        &self.covered_header_fields
    }

    pub fn excluded_header_fields(&self) -> &[ChecksumHeaderField] {
        &self.excluded_header_fields
    }

    pub const fn checksum_field_handling(&self) -> ChecksumFieldHandling {
        self.checksum_field_handling
    }

    pub fn mutable_publication_fields(&self) -> &[ChecksumHeaderField] {
        &self.mutable_publication_fields
    }

    pub const fn reserved_fields(&self) -> ChecksumReservedFieldPosture {
        self.reserved_fields
    }

    pub const fn generation_fields(&self) -> ChecksumGenerationFieldPosture {
        self.generation_fields
    }

    pub const fn length_fields(&self) -> ChecksumLengthFieldPosture {
        self.length_fields
    }

    pub const fn payload_region(&self) -> ChecksumPayloadRegion {
        self.payload_region
    }

    pub const fn padding_bytes(&self) -> ChecksumPaddingPosture {
        self.padding_bytes
    }

    pub const fn compatibility_fields(&self) -> ChecksumCompatibilityFieldPosture {
        self.compatibility_fields
    }

    pub const fn unknown_field_posture(&self) -> ChecksumUnknownFieldPosture {
        self.unknown_field_posture
    }

    pub const fn coverage_encoding(&self) -> ChecksumCoverageEncoding {
        self.coverage_encoding
    }

    pub fn disposition_for_region(
        &self,
        region: ChecksumCoverageRegion,
    ) -> ChecksumCoverageDisposition {
        match region {
            ChecksumCoverageRegion::HeaderField(field) => self.disposition_for_header_field(field),
            ChecksumCoverageRegion::PayloadRegion => ChecksumCoverageDisposition::Covered,
            ChecksumCoverageRegion::PaddingBytes => ChecksumCoverageDisposition::Excluded,
            ChecksumCoverageRegion::CompatibilityFields => ChecksumCoverageDisposition::Covered,
            ChecksumCoverageRegion::LaterPhysicalFamily => ChecksumCoverageDisposition::Skipped,
            ChecksumCoverageRegion::UnknownFutureField => ChecksumCoverageDisposition::Denied,
        }
    }

    pub fn disposition_for_header_field(
        &self,
        field: ChecksumHeaderField,
    ) -> ChecksumCoverageDisposition {
        if self.mutable_publication_fields.contains(&field) {
            ChecksumCoverageDisposition::Preserved
        } else if self.excluded_header_fields.contains(&field) {
            ChecksumCoverageDisposition::Excluded
        } else if self.covered_header_fields.contains(&field) {
            ChecksumCoverageDisposition::Covered
        } else {
            ChecksumCoverageDisposition::Denied
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChecksumCoverageMapBuilder {
    authority_source: ChecksumCoverageAuthoritySource,
    physical_format_version: PhysicalFormatVersion,
    covered_header_fields: Option<Vec<ChecksumHeaderField>>,
    excluded_header_fields: Option<Vec<ChecksumHeaderField>>,
    checksum_field_handling: Option<ChecksumFieldHandling>,
    mutable_publication_fields: Option<Vec<ChecksumHeaderField>>,
    reserved_fields: Option<ChecksumReservedFieldPosture>,
    generation_fields: Option<ChecksumGenerationFieldPosture>,
    length_fields: Option<ChecksumLengthFieldPosture>,
    payload_region: Option<ChecksumPayloadRegion>,
    padding_bytes: Option<ChecksumPaddingPosture>,
    compatibility_fields: Option<ChecksumCompatibilityFieldPosture>,
    unknown_field_posture: Option<ChecksumUnknownFieldPosture>,
    coverage_encoding: Option<ChecksumCoverageEncoding>,
}

impl ChecksumCoverageMapBuilder {
    pub const fn new(version: PhysicalFormatVersion) -> Self {
        Self {
            authority_source: ChecksumCoverageAuthoritySource::ExplicitStoreLaw,
            physical_format_version: version,
            covered_header_fields: None,
            excluded_header_fields: None,
            checksum_field_handling: None,
            mutable_publication_fields: None,
            reserved_fields: None,
            generation_fields: None,
            length_fields: None,
            payload_region: None,
            padding_bytes: None,
            compatibility_fields: None,
            unknown_field_posture: None,
            coverage_encoding: None,
        }
    }

    pub const fn authority_source(mut self, source: ChecksumCoverageAuthoritySource) -> Self {
        self.authority_source = source;
        self
    }

    pub fn covered_header_fields(
        mut self,
        fields: impl IntoIterator<Item = ChecksumHeaderField>,
    ) -> Self {
        self.covered_header_fields = Some(fields.into_iter().collect());
        self
    }

    pub fn excluded_header_fields(
        mut self,
        fields: impl IntoIterator<Item = ChecksumHeaderField>,
    ) -> Self {
        self.excluded_header_fields = Some(fields.into_iter().collect());
        self
    }

    pub const fn checksum_field_handling(mut self, handling: ChecksumFieldHandling) -> Self {
        self.checksum_field_handling = Some(handling);
        self
    }

    pub fn mutable_publication_fields(
        mut self,
        fields: impl IntoIterator<Item = ChecksumHeaderField>,
    ) -> Self {
        self.mutable_publication_fields = Some(fields.into_iter().collect());
        self
    }

    pub const fn reserved_fields(mut self, posture: ChecksumReservedFieldPosture) -> Self {
        self.reserved_fields = Some(posture);
        self
    }

    pub const fn generation_fields(mut self, posture: ChecksumGenerationFieldPosture) -> Self {
        self.generation_fields = Some(posture);
        self
    }

    pub const fn length_fields(mut self, posture: ChecksumLengthFieldPosture) -> Self {
        self.length_fields = Some(posture);
        self
    }

    pub const fn payload_region(mut self, region: ChecksumPayloadRegion) -> Self {
        self.payload_region = Some(region);
        self
    }

    pub const fn padding_bytes(mut self, posture: ChecksumPaddingPosture) -> Self {
        self.padding_bytes = Some(posture);
        self
    }

    pub const fn compatibility_fields(
        mut self,
        posture: ChecksumCompatibilityFieldPosture,
    ) -> Self {
        self.compatibility_fields = Some(posture);
        self
    }

    pub const fn unknown_field_posture(mut self, posture: ChecksumUnknownFieldPosture) -> Self {
        self.unknown_field_posture = Some(posture);
        self
    }

    pub const fn coverage_encoding(mut self, encoding: ChecksumCoverageEncoding) -> Self {
        self.coverage_encoding = Some(encoding);
        self
    }

    pub fn define(self) -> Result<ChecksumCoverageMap, ChecksumCoverageMapDenial> {
        reject_non_store_authority(self.authority_source)?;
        if self.physical_format_version != PhysicalFormatVersion::s1_initial()
            && !self.physical_format_version.is_reserved_future()
        {
            return Err(ChecksumCoverageMapDenial::UnsupportedFormatVersion);
        }
        let covered_header_fields = self
            .covered_header_fields
            .ok_or(ChecksumCoverageMapDenial::MissingCoveredHeaderFields)?;
        require_s1_header_fields(&covered_header_fields)?;

        Ok(ChecksumCoverageMap {
            physical_format_version: self.physical_format_version,
            covered_header_fields,
            excluded_header_fields: self
                .excluded_header_fields
                .ok_or(ChecksumCoverageMapDenial::MissingExcludedHeaderFields)?,
            checksum_field_handling: self
                .checksum_field_handling
                .ok_or(ChecksumCoverageMapDenial::MissingChecksumFieldHandling)?,
            mutable_publication_fields: self
                .mutable_publication_fields
                .ok_or(ChecksumCoverageMapDenial::MissingMutablePublicationFields)?,
            reserved_fields: self
                .reserved_fields
                .ok_or(ChecksumCoverageMapDenial::MissingReservedFields)?,
            generation_fields: self
                .generation_fields
                .ok_or(ChecksumCoverageMapDenial::MissingGenerationFields)?,
            length_fields: self
                .length_fields
                .ok_or(ChecksumCoverageMapDenial::MissingLengthFields)?,
            payload_region: self
                .payload_region
                .ok_or(ChecksumCoverageMapDenial::MissingPayloadRegion)?,
            padding_bytes: self
                .padding_bytes
                .ok_or(ChecksumCoverageMapDenial::MissingPaddingBytes)?,
            compatibility_fields: self
                .compatibility_fields
                .ok_or(ChecksumCoverageMapDenial::MissingCompatibilityFields)?,
            unknown_field_posture: self
                .unknown_field_posture
                .ok_or(ChecksumCoverageMapDenial::MissingUnknownFieldPosture)?,
            coverage_encoding: self
                .coverage_encoding
                .ok_or(ChecksumCoverageMapDenial::MissingCoverageEncoding)?,
        })
    }
}

fn reject_non_store_authority(
    source: ChecksumCoverageAuthoritySource,
) -> Result<(), ChecksumCoverageMapDenial> {
    match source {
        ChecksumCoverageAuthoritySource::ExplicitStoreLaw => Ok(()),
        ChecksumCoverageAuthoritySource::SerdeMapOrder => {
            Err(ChecksumCoverageMapDenial::SerializerOrderRejected)
        }
        ChecksumCoverageAuthoritySource::RustStructLayout => {
            Err(ChecksumCoverageMapDenial::RustLayoutRejected)
        }
    }
}

pub fn s1_required_covered_header_fields() -> [ChecksumHeaderField; 8] {
    [
        ChecksumHeaderField::Magic,
        ChecksumHeaderField::FormatVersion,
        ChecksumHeaderField::HeaderLength,
        ChecksumHeaderField::HeaderKind,
        ChecksumHeaderField::Generation,
        ChecksumHeaderField::PayloadLength,
        ChecksumHeaderField::ReservedFields,
        ChecksumHeaderField::CompatibilityFields,
    ]
}

fn require_s1_header_fields(
    fields: &[ChecksumHeaderField],
) -> Result<(), ChecksumCoverageMapDenial> {
    for required in s1_required_covered_header_fields() {
        if !fields.contains(&required) {
            return Err(ChecksumCoverageMapDenial::MissingRequiredHeaderField(
                required,
            ));
        }
    }
    Ok(())
}
