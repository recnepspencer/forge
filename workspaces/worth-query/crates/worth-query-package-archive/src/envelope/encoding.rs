use crate::binary_encoding::{BinaryEncodingMeasure, BinaryEncodingSink};
use crate::binary_output::BinaryOutput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::encoding::encode_package_archive;
use crate::limits::WorthQueryPackageArchiveLimits;
use worth_query_installation::facade::{
    WorthQueryPortableDomainPackageIdentity, WorthQueryPortablePackageRecordSet,
};

use super::limits::{require_complete_envelope_budget, require_signature_budget};
use super::{
    WorthQueryPackageArchiveChecksum, WorthQueryPackageEnvelopeLimits,
    WorthQueryPackageReleaseEnvelopeDescriptor, WorthQueryPackageReleaseRequirements,
    WorthQuerySignedPackageReleaseEnvelope, WorthQueryUnsignedPackageReleaseEnvelope,
    ENVELOPE_FIXED_HEADER_BYTES, ENVELOPE_MAGIC,
    WORTH_QUERY_PACKAGE_RELEASE_ENVELOPE_PROTOCOL_VERSION,
};

pub fn prepare_package_release_envelope(
    records: &WorthQueryPortablePackageRecordSet,
    descriptor: WorthQueryPackageReleaseEnvelopeDescriptor,
    archive_limits: WorthQueryPackageArchiveLimits,
    envelope_limits: WorthQueryPackageEnvelopeLimits,
) -> Result<WorthQueryUnsignedPackageReleaseEnvelope, Denial> {
    let envelope_limits = envelope_limits.narrowed();
    let maximum_archive_bytes = archive_limits
        .maximum_archive_bytes()
        .min(envelope_limits.maximum_archive_bytes());
    let archive_limits = archive_limits.with_maximum_archive_bytes(maximum_archive_bytes);
    let archive = encode_package_archive(records, archive_limits)?;
    require_archive_budget(archive.len(), envelope_limits)?;
    let expected_package_identity = records.manifest().package_identity().clone();
    let archive_checksum = WorthQueryPackageArchiveChecksum::derive(&archive);
    let requirements = WorthQueryPackageReleaseRequirements::derive(records);
    let signing_payload = encode_signing_payload(
        &archive,
        &expected_package_identity,
        archive_checksum,
        &requirements,
        &descriptor,
        envelope_limits,
    )?;
    Ok(WorthQueryUnsignedPackageReleaseEnvelope::new(
        archive,
        expected_package_identity,
        archive_checksum,
        descriptor,
        requirements,
        signing_payload,
    ))
}

pub fn encode_package_release_envelope(
    envelope: &WorthQuerySignedPackageReleaseEnvelope,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<Vec<u8>, Denial> {
    let limits = limits.narrowed();
    require_signature_budget(envelope.signature().bytes().len(), limits)?;
    require_complete_envelope_budget(
        envelope.signing_payload().len(),
        envelope.signature().bytes().len(),
        limits,
    )?;
    let capacity = envelope
        .signing_payload()
        .len()
        .checked_add(4)
        .and_then(|value| value.checked_add(envelope.signature().bytes().len()))
        .ok_or_else(|| Denial::new(Kind::EnvelopeByteBudgetExceeded))?;
    let mut output = BinaryOutput::with_capacity(capacity);
    output.raw_bytes(envelope.signing_payload());
    output.u32(
        u32::try_from(envelope.signature().bytes().len())
            .map_err(|_| Denial::new(Kind::EnvelopeSignatureByteBudgetExceeded))?,
    );
    output.raw_bytes(envelope.signature().bytes());
    Ok(output.into_bytes())
}

pub(crate) fn encode_signing_payload(
    archive: &[u8],
    expected_package_identity: &WorthQueryPortableDomainPackageIdentity,
    archive_checksum: WorthQueryPackageArchiveChecksum,
    requirements: &WorthQueryPackageReleaseRequirements,
    descriptor: &WorthQueryPackageReleaseEnvelopeDescriptor,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<Vec<u8>, Denial> {
    let limits = limits.narrowed();
    require_archive_budget(archive.len(), limits)?;
    require_requirement_budget(requirements, limits)?;
    require_text_budget(requirements, descriptor, limits)?;

    let mut body_measure = BinaryEncodingMeasure::default();
    write_body(
        &mut body_measure,
        archive,
        expected_package_identity,
        archive_checksum,
        requirements,
        descriptor,
    )?;
    let signing_payload_bytes = ENVELOPE_FIXED_HEADER_BYTES
        .checked_add(body_measure.bytes())
        .ok_or_else(|| Denial::new(Kind::EnvelopeByteBudgetExceeded))?;
    if signing_payload_bytes > limits.maximum_envelope_bytes() {
        return Err(Denial::new(Kind::EnvelopeByteBudgetExceeded));
    }
    let capacity = usize::try_from(signing_payload_bytes)
        .map_err(|_| Denial::new(Kind::EnvelopeByteBudgetExceeded))?;
    let mut output = BinaryOutput::with_capacity(capacity);
    write_signing_header(&mut output, body_measure.bytes())?;
    write_body(
        &mut output,
        archive,
        expected_package_identity,
        archive_checksum,
        requirements,
        descriptor,
    )?;
    Ok(output.into_bytes())
}

fn write_signing_header(
    output: &mut dyn BinaryEncodingSink,
    body_bytes: u64,
) -> Result<(), Denial> {
    output.raw_bytes(&ENVELOPE_MAGIC)?;
    output.u16(WORTH_QUERY_PACKAGE_RELEASE_ENVELOPE_PROTOCOL_VERSION)?;
    output.u64(body_bytes)
}

fn write_body(
    output: &mut dyn BinaryEncodingSink,
    archive: &[u8],
    expected_package_identity: &WorthQueryPortableDomainPackageIdentity,
    archive_checksum: WorthQueryPackageArchiveChecksum,
    requirements: &WorthQueryPackageReleaseRequirements,
    descriptor: &WorthQueryPackageReleaseEnvelopeDescriptor,
) -> Result<(), Denial> {
    let build_metadata = descriptor.build_metadata();
    let release_metadata = descriptor.release_metadata();
    let provenance = descriptor.provenance();
    let signer = descriptor.signer();
    output.raw_bytes(expected_package_identity.bytes())?;
    output.raw_bytes(archive_checksum.bytes())?;
    output.text(build_metadata.compiler_identity())?;
    output.text(build_metadata.compiler_version())?;
    output.text(build_metadata.toolchain_identity())?;
    output.text(build_metadata.toolchain_version())?;
    output.text(build_metadata.target_triple())?;
    output.text(release_metadata.release_name())?;
    output.text(release_metadata.release_version())?;
    output.text(provenance.source_repository())?;
    output.text(provenance.source_revision())?;
    output.text(provenance.source_reference())?;
    write_requirements(output, requirements)?;
    output.text(signer.signer_identity())?;
    output.text(signer.signature_protocol_identity().as_str())?;
    output.u32(signer.signature_protocol_version().get())?;
    output.u64(
        u64::try_from(archive.len())
            .map_err(|_| Denial::new(Kind::EnvelopeArchiveByteBudgetExceeded))?,
    )?;
    output.raw_bytes(archive)
}

fn write_requirements(
    output: &mut dyn BinaryEncodingSink,
    requirements: &WorthQueryPackageReleaseRequirements,
) -> Result<(), Denial> {
    output.u32(requirement_length(requirements.capabilities().len())?)?;
    for requirement in requirements.capabilities() {
        output.text(requirement.as_str())?;
    }
    output.u32(requirement_length(requirements.configuration().len())?)?;
    for requirement in requirements.configuration() {
        output.text(requirement.as_str())?;
    }
    output.u32(requirement_length(requirements.operating().len())?)?;
    for requirement in requirements.operating() {
        output.text(requirement.as_str())?;
    }
    output.u32(requirement_length(
        requirements.execution_providers().len(),
    )?)?;
    for requirement in requirements.execution_providers() {
        output.text(requirement.provider().as_str())?;
        output.text(requirement.access_product().as_str())?;
        output.text(requirement.allocator().as_str())?;
    }
    Ok(())
}

fn requirement_length(length: usize) -> Result<u32, Denial> {
    u32::try_from(length).map_err(|_| Denial::new(Kind::EnvelopeRequirementBudgetExceeded))
}

fn require_archive_budget(
    observed: usize,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<(), Denial> {
    if u64::try_from(observed).unwrap_or(u64::MAX) > limits.maximum_archive_bytes() {
        return Err(Denial::new(Kind::EnvelopeArchiveByteBudgetExceeded));
    }
    Ok(())
}

fn require_requirement_budget(
    requirements: &WorthQueryPackageReleaseRequirements,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<(), Denial> {
    if requirements.count()? > limits.maximum_requirements() {
        return Err(Denial::new(Kind::EnvelopeRequirementBudgetExceeded));
    }
    Ok(())
}

fn require_text_budget(
    requirements: &WorthQueryPackageReleaseRequirements,
    descriptor: &WorthQueryPackageReleaseEnvelopeDescriptor,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<(), Denial> {
    let build = descriptor.build_metadata();
    let release = descriptor.release_metadata();
    let provenance = descriptor.provenance();
    let signer = descriptor.signer();
    let fixed = [
        build.compiler_identity(),
        build.compiler_version(),
        build.toolchain_identity(),
        build.toolchain_version(),
        build.target_triple(),
        release.release_name(),
        release.release_version(),
        provenance.source_repository(),
        provenance.source_revision(),
        provenance.source_reference(),
        signer.signer_identity(),
        signer.signature_protocol_identity().as_str(),
    ];
    let mut observed = fixed.into_iter().try_fold(0_u64, add_text_bytes)?;
    observed = requirements
        .capabilities()
        .iter()
        .map(|value| value.as_str())
        .chain(
            requirements
                .configuration()
                .iter()
                .map(|value| value.as_str()),
        )
        .chain(requirements.operating().iter().map(|value| value.as_str()))
        .try_fold(observed, add_text_bytes)?;
    observed =
        requirements
            .execution_providers()
            .iter()
            .try_fold(observed, |bytes, requirement| {
                [
                    requirement.provider().as_str(),
                    requirement.access_product().as_str(),
                    requirement.allocator().as_str(),
                ]
                .into_iter()
                .try_fold(bytes, add_text_bytes)
            })?;
    if observed > limits.maximum_descriptive_text_bytes() {
        return Err(Denial::new(Kind::EnvelopeDescriptiveTextByteBudgetExceeded));
    }
    Ok(())
}

fn add_text_bytes(observed: u64, text: &str) -> Result<u64, Denial> {
    observed
        .checked_add(u64::try_from(text.len()).unwrap_or(u64::MAX))
        .ok_or_else(|| Denial::new(Kind::EnvelopeDescriptiveTextByteBudgetExceeded))
}
