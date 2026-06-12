use super::denial::MixedSurfaceKillBoxDenial;
use super::family_run::MixedSurfaceFamilyRun;
use super::receipt::MixedSurfaceKillBoxReceipt;
use crate::workload_platform::geometry_binding::BoundGeometryWorkload;
use crate::workload_platform::surface_support::{SurfaceFamily, SurfaceSupportWorkload};

pub struct MixedSurfaceKillBoxWorkload {
    bound_geometry: BoundGeometryWorkload,
    declaration: String,
    families: Vec<SurfaceFamily>,
}

impl MixedSurfaceKillBoxWorkload {
    pub fn for_bound_geometry(bound_geometry: BoundGeometryWorkload) -> Self {
        Self {
            bound_geometry,
            declaration: "mixed surface kill box workload".to_string(),
            families: SurfaceFamily::ALL.to_vec(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_surface_family_matrix<I>(mut self, families: I) -> Self
    where
        I: IntoIterator<Item = SurfaceFamily>,
    {
        self.families = families.into_iter().collect();
        self
    }

    pub fn certify(self) -> Result<MixedSurfaceKillBoxReceipt, MixedSurfaceKillBoxDenial> {
        if self.declaration.trim().is_empty() {
            return Err(MixedSurfaceKillBoxDenial::MissingDeclaration);
        }
        self.require_complete_unique_family_matrix()?;
        let stable_geometry_binding_identity = self
            .bound_geometry
            .receipts()
            .stage_identity()
            .receipt_identity();
        let runs = self.certify_family_runs()?;
        self.require_stable_geometry_identity(&runs, &stable_geometry_binding_identity)?;
        Ok(MixedSurfaceKillBoxReceipt::new(
            self.declaration,
            stable_geometry_binding_identity,
            runs,
        ))
    }

    fn require_complete_unique_family_matrix(&self) -> Result<(), MixedSurfaceKillBoxDenial> {
        for family in SurfaceFamily::ALL {
            let count = self
                .families
                .iter()
                .filter(|candidate| **candidate == family)
                .count();
            if count == 0 {
                return Err(MixedSurfaceKillBoxDenial::MissingFamilyRun { family });
            }
            if count > 1 {
                return Err(MixedSurfaceKillBoxDenial::DuplicateFamilyRun { family });
            }
        }
        Ok(())
    }

    fn certify_family_runs(&self) -> Result<Vec<MixedSurfaceFamilyRun>, MixedSurfaceKillBoxDenial> {
        let mut runs = Vec::with_capacity(self.families.len());
        for family in self.families.iter().copied() {
            let support = SurfaceSupportWorkload::for_bound_geometry(self.bound_geometry.clone())
                .declared(format!(
                    "{} surface support for {}",
                    self.declaration,
                    family.human_label()
                ))
                .with_surface_family(family)
                .certify();
            match support {
                Ok(certified) => {
                    if family != SurfaceFamily::Plane {
                        return Err(MixedSurfaceKillBoxDenial::GeneratedFeatureSmugglingAttempt);
                    }
                    runs.push(MixedSurfaceFamilyRun::from_certified_plane(
                        certified,
                        &format!("{} plane response", self.declaration),
                    ));
                }
                Err(unsupported) => {
                    runs.push(MixedSurfaceFamilyRun::from_unsupported(
                        unsupported,
                        &format!("{} unsupported response", self.declaration),
                    )?);
                }
            }
        }
        Ok(runs)
    }

    fn require_stable_geometry_identity(
        &self,
        runs: &[MixedSurfaceFamilyRun],
        stable_geometry_binding_identity: &str,
    ) -> Result<(), MixedSurfaceKillBoxDenial> {
        for run in runs {
            if run.upstream_geometry_binding_identity() != stable_geometry_binding_identity {
                return Err(MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence {
                    family: run.family(),
                });
            }
        }
        Ok(())
    }
}
