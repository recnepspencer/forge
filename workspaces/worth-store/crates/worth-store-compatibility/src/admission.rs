use crate::{
    ArtifactCompatibilityDenial, RestoreCompatibilityPlan, RestoreCompatibilityReceipt,
    RollingUpgradeAdmissionPlan, RollingWindowCompatibilityReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityAdmission;

pub const fn compatibility_admission() -> CompatibilityAdmission {
    CompatibilityAdmission
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RollingCompatibilityAdmissionCase {
    Admitted(RollingWindowCompatibilityReceipt),
    Denied(ArtifactCompatibilityDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollingCompatibilityAdmissionOutcome {
    case: RollingCompatibilityAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollingCompatibilityAdmissionView<'a> {
    Admitted(&'a RollingWindowCompatibilityReceipt),
    Denied(&'a ArtifactCompatibilityDenial),
}

impl RollingCompatibilityAdmissionOutcome {
    fn admitted(receipt: RollingWindowCompatibilityReceipt) -> Self {
        Self {
            case: RollingCompatibilityAdmissionCase::Admitted(receipt),
        }
    }

    fn denied(denial: ArtifactCompatibilityDenial) -> Self {
        Self {
            case: RollingCompatibilityAdmissionCase::Denied(denial),
        }
    }

    pub const fn view(&self) -> RollingCompatibilityAdmissionView<'_> {
        match &self.case {
            RollingCompatibilityAdmissionCase::Admitted(receipt) => {
                RollingCompatibilityAdmissionView::Admitted(receipt)
            }
            RollingCompatibilityAdmissionCase::Denied(denial) => {
                RollingCompatibilityAdmissionView::Denied(denial)
            }
        }
    }

    pub fn into_admitted(
        self,
    ) -> Result<RollingWindowCompatibilityReceipt, ArtifactCompatibilityDenial> {
        match self.case {
            RollingCompatibilityAdmissionCase::Admitted(receipt) => Ok(receipt),
            RollingCompatibilityAdmissionCase::Denied(denial) => Err(denial),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RestoreCompatibilityAdmissionCase {
    Admitted(RestoreCompatibilityReceipt),
    Denied(ArtifactCompatibilityDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestoreCompatibilityAdmissionOutcome {
    case: RestoreCompatibilityAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreCompatibilityAdmissionView<'a> {
    Admitted(&'a RestoreCompatibilityReceipt),
    Denied(&'a ArtifactCompatibilityDenial),
}

impl RestoreCompatibilityAdmissionOutcome {
    fn admitted(receipt: RestoreCompatibilityReceipt) -> Self {
        Self {
            case: RestoreCompatibilityAdmissionCase::Admitted(receipt),
        }
    }

    fn denied(denial: ArtifactCompatibilityDenial) -> Self {
        Self {
            case: RestoreCompatibilityAdmissionCase::Denied(denial),
        }
    }

    pub const fn view(&self) -> RestoreCompatibilityAdmissionView<'_> {
        match &self.case {
            RestoreCompatibilityAdmissionCase::Admitted(receipt) => {
                RestoreCompatibilityAdmissionView::Admitted(receipt)
            }
            RestoreCompatibilityAdmissionCase::Denied(denial) => {
                RestoreCompatibilityAdmissionView::Denied(denial)
            }
        }
    }

    pub fn into_admitted(self) -> Result<RestoreCompatibilityReceipt, ArtifactCompatibilityDenial> {
        match self.case {
            RestoreCompatibilityAdmissionCase::Admitted(receipt) => Ok(receipt),
            RestoreCompatibilityAdmissionCase::Denied(denial) => Err(denial),
        }
    }
}

impl CompatibilityAdmission {
    pub fn admit_rolling(
        self,
        plan: RollingUpgradeAdmissionPlan,
    ) -> RollingCompatibilityAdmissionOutcome {
        if !plan.window().contains(plan.window().write_version()) {
            return RollingCompatibilityAdmissionOutcome::denied(
                ArtifactCompatibilityDenial::WriteVersionOutsideCompatibilityWindow,
            );
        }
        RollingCompatibilityAdmissionOutcome::admitted(RollingWindowCompatibilityReceipt::new(plan))
    }

    pub fn admit_restore(
        self,
        plan: RestoreCompatibilityPlan,
    ) -> RestoreCompatibilityAdmissionOutcome {
        if !plan.window().contains(plan.target_version()) {
            return RestoreCompatibilityAdmissionOutcome::denied(
                ArtifactCompatibilityDenial::VersionOutsideCompatibilityWindow,
            );
        }
        RestoreCompatibilityAdmissionOutcome::admitted(RestoreCompatibilityReceipt::new(plan))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArtifactCompatibilityWindow, ArtifactFormatVersion, RollingUpgradePolicy};

    #[test]
    fn owner_admits_rolling_and_denies_out_of_window_restore() {
        let window = ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion(2),
            ArtifactFormatVersion(3),
            ArtifactFormatVersion(4),
        )
        .unwrap();
        let rolling = compatibility_admission().admit_rolling(RollingUpgradeAdmissionPlan::new(
            window,
            RollingUpgradePolicy::ReadOldWriteNew,
        ));
        assert!(rolling.into_admitted().is_ok());

        let restore = compatibility_admission().admit_restore(RestoreCompatibilityPlan::new(
            window,
            ArtifactFormatVersion(5),
        ));
        assert_eq!(
            restore.into_admitted(),
            Err(ArtifactCompatibilityDenial::VersionOutsideCompatibilityWindow)
        );
    }
}
