use worth_store::physical_runtime::{
    PhysicalWorkCausalRecord, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition,
};

use super::{
    causal_route::PhysicalWorkCausalRouteEvidence, terminal_identity::TerminalIdentityIndex,
    PhysicalWorkBackendRoleEvidence, PhysicalWorkReconciliationBasis,
    PhysicalWorkReconciliationRecordEvidence,
};

pub(super) fn reconcile(
    basis: &PhysicalWorkReconciliationBasis,
    record: PhysicalWorkCausalRecord,
    terminal: &mut TerminalIdentityIndex,
) -> Result<PhysicalWorkReconciliationRecordEvidence, String> {
    let identity = record.identity();
    require_runtime_binding(basis, identity)?;
    let backend_operation = record
        .backend_operation()
        .ok_or_else(|| "media-reaching physical work omitted its backend receipt".to_owned())?;
    let backend_role = record
        .backend_role()
        .ok_or_else(|| "media-reaching physical work omitted its backend role".to_owned())?;
    let backend_role = PhysicalWorkBackendRoleEvidence::try_from(backend_role)?;
    require_backend_role(record.operation(), backend_role)?;
    require_successful_fate(record)?;
    let route = PhysicalWorkCausalRouteEvidence::from_record(record)?;
    require_signal_family(record.operation(), route.signal_family)?;
    Ok(PhysicalWorkReconciliationRecordEvidence {
        store: identity.store(),
        runtime: identity.runtime(),
        generation: identity.generation().lifecycle(),
        operation: identity.operation().get(),
        family: record.operation(),
        backend_operation: backend_operation.value(),
        backend_role,
        effect_fate: record.effect_fate(),
        recovery: record.recovery(),
        route,
        terminal: terminal.take(identity)?,
    })
}

fn require_backend_role(
    operation: PhysicalWorkOperationFamily,
    role: PhysicalWorkBackendRoleEvidence,
) -> Result<(), String> {
    let exact = match operation {
        PhysicalWorkOperationFamily::ArtifactMetadataRead => {
            role == PhysicalWorkBackendRoleEvidence::ReadMetadata
        }
        PhysicalWorkOperationFamily::ArtifactRangeRead => {
            role == PhysicalWorkBackendRoleEvidence::PositionedRead
        }
        PhysicalWorkOperationFamily::ArtifactRangeWrite => {
            role == PhysicalWorkBackendRoleEvidence::PositionedWrite
        }
        PhysicalWorkOperationFamily::WalAppend => {
            role == PhysicalWorkBackendRoleEvidence::PositionedWrite
        }
        PhysicalWorkOperationFamily::DurabilityBarrier => {
            role == PhysicalWorkBackendRoleEvidence::SynchronizeFileState
        }
        PhysicalWorkOperationFamily::ArtifactPublication => matches!(
            role,
            PhysicalWorkBackendRoleEvidence::PositionedWrite
                | PhysicalWorkBackendRoleEvidence::SynchronizeFileState
                | PhysicalWorkBackendRoleEvidence::SynchronizeDirectoryPublication
                | PhysicalWorkBackendRoleEvidence::AtomicReplace
        ),
        PhysicalWorkOperationFamily::CheckpointCapture => matches!(
            role,
            PhysicalWorkBackendRoleEvidence::CreateNew
                | PhysicalWorkBackendRoleEvidence::PositionedWrite
                | PhysicalWorkBackendRoleEvidence::SynchronizeFileState
                | PhysicalWorkBackendRoleEvidence::SynchronizeDirectoryPublication
                | PhysicalWorkBackendRoleEvidence::AtomicReplace
        ),
        PhysicalWorkOperationFamily::WalReclamation => {
            role == PhysicalWorkBackendRoleEvidence::Delete
        }
        PhysicalWorkOperationFamily::RootPublication => matches!(
            role,
            PhysicalWorkBackendRoleEvidence::SynchronizeFileState
                | PhysicalWorkBackendRoleEvidence::SynchronizeDirectoryPublication
                | PhysicalWorkBackendRoleEvidence::AtomicReplace
        ),
    };
    if exact {
        Ok(())
    } else {
        Err("physical work route carried the wrong backend media role".to_owned())
    }
}

fn require_signal_family(
    operation: PhysicalWorkOperationFamily,
    family: worth_store::physical_runtime::PhysicalWorkSignalFamily,
) -> Result<(), String> {
    if operation.required_signal_family() != family {
        return Err("physical work route carried the wrong Signal family".to_owned());
    }
    Ok(())
}

fn require_runtime_binding(
    basis: &PhysicalWorkReconciliationBasis,
    identity: PhysicalWorkIdentity,
) -> Result<(), String> {
    if identity.store() != basis.store
        || identity.runtime() != basis.runtime
        || identity.generation().lifecycle() != basis.generation
    {
        return Err("media-reaching physical work escaped its Store runtime generation".to_owned());
    }
    Ok(())
}

fn require_successful_fate(record: PhysicalWorkCausalRecord) -> Result<(), String> {
    let exact = match record.operation() {
        PhysicalWorkOperationFamily::ArtifactMetadataRead
        | PhysicalWorkOperationFamily::ArtifactRangeRead => {
            record.effect_fate() == PhysicalWorkEffectFate::ReadCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::NoEffect
        }
        PhysicalWorkOperationFamily::ArtifactRangeWrite
        | PhysicalWorkOperationFamily::WalAppend => {
            record.effect_fate() == PhysicalWorkEffectFate::WriteCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::ContinueSettlement
        }
        PhysicalWorkOperationFamily::ArtifactPublication => {
            record.effect_fate() == PhysicalWorkEffectFate::PublicationCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::ContinueSettlement
        }
        PhysicalWorkOperationFamily::DurabilityBarrier => {
            record.effect_fate() == PhysicalWorkEffectFate::PublicationCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::ContinueSettlement
        }
        PhysicalWorkOperationFamily::CheckpointCapture => {
            record.effect_fate() == PhysicalWorkEffectFate::CheckpointCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::ContinueSettlement
        }
        PhysicalWorkOperationFamily::WalReclamation => {
            record.effect_fate() == PhysicalWorkEffectFate::WalReclamationCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::ContinueSettlement
        }
        PhysicalWorkOperationFamily::RootPublication => {
            record.effect_fate() == PhysicalWorkEffectFate::PublicationCompleted
                && record.recovery() == PhysicalWorkRecoveryDisposition::ContinueSettlement
        }
    };
    if !exact {
        return Err(format!(
            "media-reaching {:?} retained inexact fate {:?}/{:?}",
            record.operation(),
            record.effect_fate(),
            record.recovery()
        ));
    }
    Ok(())
}
