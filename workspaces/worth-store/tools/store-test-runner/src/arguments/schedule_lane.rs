#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct CiScheduleLane(u8);

impl CiScheduleLane {
    pub(crate) const COUNT: u8 = 16;

    pub(crate) fn parse(raw: &str) -> Result<Self, String> {
        let lane = raw.parse::<u8>().map_err(|_| {
            format!("--ci-schedule-lane requires an integer from 0 through 15, got `{raw}`")
        })?;
        if lane < Self::COUNT {
            Ok(Self(lane))
        } else {
            Err(format!(
                "--ci-schedule-lane requires an integer from 0 through 15, got `{raw}`"
            ))
        }
    }

    #[cfg(any(test, feature = "physical-work-evidence"))]
    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}
