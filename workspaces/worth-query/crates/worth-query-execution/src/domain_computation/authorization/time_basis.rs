use std::time::{SystemTime, UNIX_EPOCH};

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryAuthorizationTimeDenial {
    BeforeUnixEpoch,
    TimelineValueOverflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryAuthorizationTimeSample {
    timeline: ApplicationCapabilityValidityTimeline,
    value: AspectValue,
}

impl WorthQueryAuthorizationTimeSample {
    pub(in crate::domain_computation) const fn timeline(
        &self,
    ) -> ApplicationCapabilityValidityTimeline {
        self.timeline
    }

    pub(in crate::domain_computation) const fn value(&self) -> &AspectValue {
        &self.value
    }
}

#[derive(Default)]
pub(in crate::domain_computation) struct WorthQueryAuthorizationClock;

impl WorthQueryAuthorizationClock {
    pub(in crate::domain_computation) fn sample(
        &self,
        timeline: ApplicationCapabilityValidityTimeline,
    ) -> Result<WorthQueryAuthorizationTimeSample, WorthQueryAuthorizationTimeDenial> {
        sample_at(SystemTime::now(), timeline)
    }
}

fn sample_at(
    now: SystemTime,
    timeline: ApplicationCapabilityValidityTimeline,
) -> Result<WorthQueryAuthorizationTimeSample, WorthQueryAuthorizationTimeDenial> {
    let elapsed = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| WorthQueryAuthorizationTimeDenial::BeforeUnixEpoch)?;
    let value = match timeline {
        ApplicationCapabilityValidityTimeline::UnixEpochSeconds => elapsed.as_secs(),
        ApplicationCapabilityValidityTimeline::UnixEpochMilliseconds => elapsed
            .as_millis()
            .try_into()
            .map_err(|_| WorthQueryAuthorizationTimeDenial::TimelineValueOverflow)?,
    };
    Ok(WorthQueryAuthorizationTimeSample {
        timeline,
        value: AspectValue::UInt64(value),
    })
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn trusted_clock_samples_the_installed_timeline_without_caller_input() {
        let now = UNIX_EPOCH + Duration::from_millis(12_345);
        let seconds =
            sample_at(now, ApplicationCapabilityValidityTimeline::UnixEpochSeconds).unwrap();
        let milliseconds = sample_at(
            now,
            ApplicationCapabilityValidityTimeline::UnixEpochMilliseconds,
        )
        .unwrap();
        assert_eq!(seconds.value(), &AspectValue::UInt64(12));
        assert_eq!(milliseconds.value(), &AspectValue::UInt64(12_345));
    }

    #[test]
    fn trusted_clock_rejects_pre_epoch_samples() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_secs(1))
            .expect("the platform supports a pre-epoch test time");
        assert_eq!(
            sample_at(
                before_epoch,
                ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
            ),
            Err(WorthQueryAuthorizationTimeDenial::BeforeUnixEpoch)
        );
    }
}
