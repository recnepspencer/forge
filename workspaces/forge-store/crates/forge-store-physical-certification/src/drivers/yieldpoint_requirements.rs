use crate::PhysicalDriverKind;

use super::yieldpoint::{
    canonical_yieldpoint_name_for_seam, PhysicalBoundarySeam, PhysicalBoundaryYieldpoint,
    YieldpointDeclaration,
};
use super::DriverAdmissionDenial;

pub(crate) fn require_driver_yieldpoints(
    driver: PhysicalDriverKind,
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
) -> Result<Vec<YieldpointDeclaration>, DriverAdmissionDenial> {
    if yieldpoints.is_empty() {
        return Err(DriverAdmissionDenial::NoYieldpointsDeclared(driver));
    }
    let mut declarations = Vec::with_capacity(yieldpoints.len());
    let mut seen = std::collections::BTreeSet::new();
    for yieldpoint in yieldpoints {
        if !seen.insert(yieldpoint.name().to_owned()) {
            return Err(DriverAdmissionDenial::DuplicateYieldpointName(
                yieldpoint.name().to_owned(),
            ));
        }
        let declaration = yieldpoint.declare()?;
        require_driver_owned_yieldpoint(driver, declaration.yieldpoint().seam())?;
        declarations.push(declaration);
    }
    require_relevant_yieldpoint(driver, &declarations)?;
    Ok(declarations)
}

fn require_driver_owned_yieldpoint(
    driver: PhysicalDriverKind,
    seam: PhysicalBoundarySeam,
) -> Result<(), DriverAdmissionDenial> {
    if !driver_owns_yieldpoint_seam(driver, seam) {
        return Err(DriverAdmissionDenial::IrrelevantYieldpointForDriver { driver, seam });
    }
    Ok(())
}

fn driver_owns_yieldpoint_seam(driver: PhysicalDriverKind, seam: PhysicalBoundarySeam) -> bool {
    match driver {
        PhysicalDriverKind::ProductionBoundaryYieldpoint => {
            matches!(seam, PhysicalBoundarySeam::ProductionStorage(_))
        }
        PhysicalDriverKind::FreshRuntimeRecovery => {
            seam == PhysicalBoundarySeam::FreshRuntimeRecovery
        }
        PhysicalDriverKind::MemoryPressureBoundary => seam == PhysicalBoundarySeam::MemoryPressure,
        PhysicalDriverKind::IoPressureBoundary => seam == PhysicalBoundarySeam::IoPressure,
        PhysicalDriverKind::OfflineVerifierBoundary => {
            matches!(seam, PhysicalBoundarySeam::OfflineVerifier(_))
        }
        PhysicalDriverKind::ShortcutRejectionBoundary => {
            seam == PhysicalBoundarySeam::ShortcutRejection
        }
        PhysicalDriverKind::FutureExtensionSlot => {
            seam == PhysicalBoundarySeam::FutureExtensionSlot
        }
    }
}

fn require_relevant_yieldpoint(
    driver: PhysicalDriverKind,
    declarations: &[YieldpointDeclaration],
) -> Result<(), DriverAdmissionDenial> {
    let required = required_yieldpoint_seam_for_driver(driver);
    if !declarations
        .iter()
        .any(|candidate| candidate.yieldpoint().seam() == required)
    {
        return Err(DriverAdmissionDenial::MissingRelevantYieldpoint {
            driver,
            yieldpoint: canonical_yieldpoint_name_for_seam(required),
        });
    }
    Ok(())
}

fn required_yieldpoint_seam_for_driver(driver: PhysicalDriverKind) -> PhysicalBoundarySeam {
    match driver {
        PhysicalDriverKind::ProductionBoundaryYieldpoint => {
            PhysicalBoundarySeam::ProductionStorage(
                forge_store_physical_backend::ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
            )
        }
        PhysicalDriverKind::FreshRuntimeRecovery => PhysicalBoundarySeam::FreshRuntimeRecovery,
        PhysicalDriverKind::MemoryPressureBoundary => PhysicalBoundarySeam::MemoryPressure,
        PhysicalDriverKind::IoPressureBoundary => PhysicalBoundarySeam::IoPressure,
        PhysicalDriverKind::OfflineVerifierBoundary => PhysicalBoundarySeam::OfflineVerifier(
            forge_store_offline_verifier::OfflineVerifierBoundarySeam::LayoutWalkBeforeRuntimeRecovery,
        ),
        PhysicalDriverKind::ShortcutRejectionBoundary => PhysicalBoundarySeam::ShortcutRejection,
        PhysicalDriverKind::FutureExtensionSlot => PhysicalBoundarySeam::FutureExtensionSlot,
    }
}
