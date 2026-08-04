use std::time::{SystemTime, UNIX_EPOCH};

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryAuthorizationTimeDenial {
    #[cfg(test)]
    SourceUnavailable,
    BeforeUnixEpoch,
    TimelineValueOverflow,
    DurationNotRepresentable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryAuthorizationTimeSample {
    timeline: ApplicationCapabilityValidityTimeline,
    value: AspectValue,
}

pub(in crate::domain_computation) struct WorthQueryAuthorizationTimeInterval {
    pub(in crate::domain_computation) issued: WorthQueryAuthorizationTimeSample,
    pub(in crate::domain_computation) expires: AspectValue,
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

trait WorthQueryAuthorizationTimeSource: Send + Sync {
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeDenial>;
}

struct WorthQuerySystemAuthorizationTimeSource;

impl WorthQueryAuthorizationTimeSource for WorthQuerySystemAuthorizationTimeSource {
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeDenial> {
        Ok(SystemTime::now())
    }
}

pub(in crate::domain_computation) struct WorthQueryAuthorizationClock {
    source: Box<dyn WorthQueryAuthorizationTimeSource>,
}

impl WorthQueryAuthorizationClock {
    pub(in crate::domain_computation) fn system() -> Self {
        Self {
            source: Box::new(WorthQuerySystemAuthorizationTimeSource),
        }
    }

    pub(in crate::domain_computation) fn sample(
        &self,
        timeline: ApplicationCapabilityValidityTimeline,
    ) -> Result<WorthQueryAuthorizationTimeSample, WorthQueryAuthorizationTimeDenial> {
        sample_at(self.source.current_time()?, timeline)
    }

    pub(in crate::domain_computation) fn sample_interval(
        &self,
        timeline: ApplicationCapabilityValidityTimeline,
        duration: std::time::Duration,
    ) -> Result<WorthQueryAuthorizationTimeInterval, WorthQueryAuthorizationTimeDenial> {
        let issued = self.sample(timeline)?;
        let units = duration_units(duration, timeline)?;
        let AspectValue::UInt64(issued_units) = issued.value() else {
            return Err(WorthQueryAuthorizationTimeDenial::TimelineValueOverflow);
        };
        let expires = issued_units
            .checked_add(units)
            .map(AspectValue::UInt64)
            .ok_or(WorthQueryAuthorizationTimeDenial::TimelineValueOverflow)?;
        Ok(WorthQueryAuthorizationTimeInterval { issued, expires })
    }

    #[cfg(test)]
    pub(crate) fn scripted(samples: impl IntoIterator<Item = SystemTime>) -> Self {
        Self {
            source: Box::new(WorthQueryScriptedAuthorizationTimeSource {
                samples: std::sync::Mutex::new(samples.into_iter().collect()),
            }),
        }
    }
}

fn duration_units(
    duration: std::time::Duration,
    timeline: ApplicationCapabilityValidityTimeline,
) -> Result<u64, WorthQueryAuthorizationTimeDenial> {
    match timeline {
        ApplicationCapabilityValidityTimeline::UnixEpochSeconds => {
            if duration.subsec_nanos() != 0 {
                return Err(WorthQueryAuthorizationTimeDenial::DurationNotRepresentable);
            }
            Ok(duration.as_secs())
        }
        ApplicationCapabilityValidityTimeline::UnixEpochMilliseconds => {
            if duration.subsec_nanos() % 1_000_000 != 0 {
                return Err(WorthQueryAuthorizationTimeDenial::DurationNotRepresentable);
            }
            duration
                .as_millis()
                .try_into()
                .map_err(|_| WorthQueryAuthorizationTimeDenial::TimelineValueOverflow)
        }
    }
}

#[cfg(test)]
struct WorthQueryScriptedAuthorizationTimeSource {
    samples: std::sync::Mutex<std::collections::VecDeque<SystemTime>>,
}

#[cfg(test)]
impl WorthQueryAuthorizationTimeSource for WorthQueryScriptedAuthorizationTimeSource {
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeDenial> {
        self.samples
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .pop_front()
            .ok_or(WorthQueryAuthorizationTimeDenial::SourceUnavailable)
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
    fn scripted_clock_retains_exact_installed_timeline_samples() {
        let clock = WorthQueryAuthorizationClock::scripted([
            UNIX_EPOCH + Duration::from_millis(12_345),
            UNIX_EPOCH + Duration::from_millis(12_345),
        ]);
        let seconds = clock
            .sample(ApplicationCapabilityValidityTimeline::UnixEpochSeconds)
            .unwrap();
        let milliseconds = clock
            .sample(ApplicationCapabilityValidityTimeline::UnixEpochMilliseconds)
            .unwrap();
        assert_eq!(seconds.value(), &AspectValue::UInt64(12));
        assert_eq!(milliseconds.value(), &AspectValue::UInt64(12_345));
    }

    #[test]
    fn scripted_clock_exhaustion_and_pre_epoch_samples_fail_closed() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_secs(1))
            .expect("the platform supports a pre-epoch test time");
        let clock = WorthQueryAuthorizationClock::scripted([before_epoch]);
        assert_eq!(
            clock.sample(ApplicationCapabilityValidityTimeline::UnixEpochSeconds),
            Err(WorthQueryAuthorizationTimeDenial::BeforeUnixEpoch)
        );
        assert_eq!(
            clock.sample(ApplicationCapabilityValidityTimeline::UnixEpochSeconds),
            Err(WorthQueryAuthorizationTimeDenial::SourceUnavailable)
        );
    }
}
