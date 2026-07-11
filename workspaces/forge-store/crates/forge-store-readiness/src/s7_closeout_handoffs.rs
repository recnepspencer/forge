#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutReadinessNonClaim {
    GlobalLayoutDiscipline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10BackupRepairReadinessNonClaim {
    BackupRestoreCorrectness,
    RepairWorkflowCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S11KeyLifecycleReadinessNonClaim {
    KeyLifecycleCorrectness,
    CryptographicErasureCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S12FullCertificationNonClaim {
    FullStoreCertification,
}

impl S8LayoutReadinessNonClaim {
    pub const fn required() -> [Self; 1] {
        [Self::GlobalLayoutDiscipline]
    }
}

impl S10BackupRepairReadinessNonClaim {
    pub const fn required() -> [Self; 2] {
        [
            Self::BackupRestoreCorrectness,
            Self::RepairWorkflowCorrectness,
        ]
    }
}

impl S11KeyLifecycleReadinessNonClaim {
    pub const fn required() -> [Self; 2] {
        [
            Self::KeyLifecycleCorrectness,
            Self::CryptographicErasureCorrectness,
        ]
    }
}

impl S12FullCertificationNonClaim {
    pub const fn required() -> [Self; 1] {
        [Self::FullStoreCertification]
    }
}
