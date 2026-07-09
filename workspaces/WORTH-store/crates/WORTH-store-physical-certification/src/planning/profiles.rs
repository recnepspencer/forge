use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalSimulationProfile {
    DeveloperSmoke,
    CiCertification,
    LocalSoak,
    ReleaseCertification,
    HardwareQualification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationProfileSet {
    profiles: BTreeSet<PhysicalSimulationProfile>,
}

impl PhysicalSimulationProfileSet {
    pub fn developer_smoke_only() -> Self {
        Self::from_profiles([PhysicalSimulationProfile::DeveloperSmoke])
    }

    pub fn all() -> Self {
        Self::from_profiles([
            PhysicalSimulationProfile::DeveloperSmoke,
            PhysicalSimulationProfile::CiCertification,
            PhysicalSimulationProfile::LocalSoak,
            PhysicalSimulationProfile::ReleaseCertification,
            PhysicalSimulationProfile::HardwareQualification,
        ])
    }

    pub fn contains(&self, profile: PhysicalSimulationProfile) -> bool {
        self.profiles.contains(&profile)
    }

    pub fn iter(&self) -> impl Iterator<Item = PhysicalSimulationProfile> + '_ {
        self.profiles.iter().copied()
    }

    pub(crate) fn from_profiles(
        profiles: impl IntoIterator<Item = PhysicalSimulationProfile>,
    ) -> Self {
        Self {
            profiles: profiles.into_iter().collect(),
        }
    }
}

pub(crate) fn profile_token(profile: PhysicalSimulationProfile) -> &'static str {
    match profile {
        PhysicalSimulationProfile::DeveloperSmoke => "developer-smoke",
        PhysicalSimulationProfile::CiCertification => "ci-certification",
        PhysicalSimulationProfile::LocalSoak => "local-soak",
        PhysicalSimulationProfile::ReleaseCertification => "release-certification",
        PhysicalSimulationProfile::HardwareQualification => "hardware-qualification",
    }
}
