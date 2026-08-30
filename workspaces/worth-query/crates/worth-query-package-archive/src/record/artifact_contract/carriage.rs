use worth_query_installation::facade::{
    WorthQueryArtifactBorrowPosture as BorrowPosture,
    WorthQueryArtifactCarriageContract as Carriage,
    WorthQueryArtifactCloneBoundary as CloneBoundary,
    WorthQueryArtifactCloneMechanism as CloneMechanism,
    WorthQueryArtifactClonePosture as ClonePosture,
    WorthQueryArtifactLifecycleContract as Lifecycle, WorthQueryArtifactMovePosture as MovePosture,
    WorthQueryArtifactProviderTransferPosture as ProviderTransfer,
    WorthQueryArtifactSerializationPosture as Serialization,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(super) fn write_carriage(
    output: &mut dyn BinaryEncodingSink,
    contract: Carriage,
) -> Result<(), Denial> {
    output.u16(move_tag(contract.movement()))?;
    output.u16(borrow_tag(contract.borrowing()))?;
    write_clone(output, contract.clone_posture())?;
    output.u16(provider_transfer_tag(contract.provider_transfer()))?;
    output.u16(serialization_tag(contract.serialization()))
}

pub(super) fn decode_carriage(input: &mut BinaryInput<'_>) -> Result<Carriage, Denial> {
    Ok(Carriage::new(
        move_from_tag(input.u16()?)?,
        borrow_from_tag(input.u16()?)?,
        decode_clone(input)?,
        provider_transfer_from_tag(input.u16()?)?,
        serialization_from_tag(input.u16()?)?,
    ))
}

pub(super) fn write_lifecycle(
    output: &mut dyn BinaryEncodingSink,
    lifecycle: Lifecycle,
) -> Result<(), Denial> {
    output.u16(lifecycle_tag(lifecycle))
}

pub(super) fn decode_lifecycle(input: &mut BinaryInput<'_>) -> Result<Lifecycle, Denial> {
    match input.u16()? {
        1 => Ok(Lifecycle::Transient),
        2 => Ok(Lifecycle::ArenaScoped),
        3 => Ok(Lifecycle::Retained),
        4 => Ok(Lifecycle::ReconstructibleDerived),
        5 => Ok(Lifecycle::ExternallyDurable),
        6 => Ok(Lifecycle::ReconstructibleAsAuthoritative),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_clone(output: &mut dyn BinaryEncodingSink, posture: ClonePosture) -> Result<(), Denial> {
    match posture {
        ClonePosture::Forbidden => output.u16(1),
        ClonePosture::Declared {
            mechanism,
            boundary,
        } => {
            output.u16(2)?;
            output.u16(clone_mechanism_tag(mechanism))?;
            output.u16(clone_boundary_tag(boundary))
        }
    }
}

fn decode_clone(input: &mut BinaryInput<'_>) -> Result<ClonePosture, Denial> {
    match input.u16()? {
        1 => Ok(ClonePosture::Forbidden),
        2 => Ok(ClonePosture::Declared {
            mechanism: clone_mechanism_from_tag(input.u16()?)?,
            boundary: clone_boundary_from_tag(input.u16()?)?,
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn move_tag(value: MovePosture) -> u16 {
    match value {
        MovePosture::Required => 1,
        MovePosture::Forbidden => 2,
    }
}

fn move_from_tag(tag: u16) -> Result<MovePosture, Denial> {
    match tag {
        1 => Ok(MovePosture::Required),
        2 => Ok(MovePosture::Forbidden),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn borrow_tag(value: BorrowPosture) -> u16 {
    match value {
        BorrowPosture::Forbidden => 1,
        BorrowPosture::SharedReadOnly => 2,
    }
}

fn borrow_from_tag(tag: u16) -> Result<BorrowPosture, Denial> {
    match tag {
        1 => Ok(BorrowPosture::Forbidden),
        2 => Ok(BorrowPosture::SharedReadOnly),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn clone_mechanism_tag(value: CloneMechanism) -> u16 {
    match value {
        CloneMechanism::DeepClone => 1,
        CloneMechanism::ProviderDefinedCopy => 2,
    }
}

fn clone_mechanism_from_tag(tag: u16) -> Result<CloneMechanism, Denial> {
    match tag {
        1 => Ok(CloneMechanism::DeepClone),
        2 => Ok(CloneMechanism::ProviderDefinedCopy),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn clone_boundary_tag(value: CloneBoundary) -> u16 {
    match value {
        CloneBoundary::ConcurrentObserver => 1,
        CloneBoundary::Isolation => 2,
        CloneBoundary::Retry => 3,
        CloneBoundary::Temporal => 4,
        CloneBoundary::ProviderTransfer => 5,
    }
}

fn clone_boundary_from_tag(tag: u16) -> Result<CloneBoundary, Denial> {
    match tag {
        1 => Ok(CloneBoundary::ConcurrentObserver),
        2 => Ok(CloneBoundary::Isolation),
        3 => Ok(CloneBoundary::Retry),
        4 => Ok(CloneBoundary::Temporal),
        5 => Ok(CloneBoundary::ProviderTransfer),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn provider_transfer_tag(value: ProviderTransfer) -> u16 {
    match value {
        ProviderTransfer::Forbidden => 1,
        ProviderTransfer::MoveOwnership => 2,
    }
}

fn provider_transfer_from_tag(tag: u16) -> Result<ProviderTransfer, Denial> {
    match tag {
        1 => Ok(ProviderTransfer::Forbidden),
        2 => Ok(ProviderTransfer::MoveOwnership),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn serialization_tag(value: Serialization) -> u16 {
    match value {
        Serialization::Forbidden => 1,
        Serialization::CanonicalProjectionOnly => 2,
        Serialization::DomainPayloadWithSchema => 3,
    }
}

fn serialization_from_tag(tag: u16) -> Result<Serialization, Denial> {
    match tag {
        1 => Ok(Serialization::Forbidden),
        2 => Ok(Serialization::CanonicalProjectionOnly),
        3 => Ok(Serialization::DomainPayloadWithSchema),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn lifecycle_tag(value: Lifecycle) -> u16 {
    match value {
        Lifecycle::Transient => 1,
        Lifecycle::ArenaScoped => 2,
        Lifecycle::Retained => 3,
        Lifecycle::ReconstructibleDerived => 4,
        Lifecycle::ExternallyDurable => 5,
        Lifecycle::ReconstructibleAsAuthoritative => 6,
    }
}
