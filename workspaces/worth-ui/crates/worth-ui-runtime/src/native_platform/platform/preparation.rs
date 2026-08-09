use std::sync::atomic::{AtomicU64, Ordering};

use super::{UiPreparedNativePlatform, WorthUiNativePlatform};
use crate::native_platform::{UiNativePlatformPreparationDenial, UiNativePlatformProfile};

static PREPARATION_IDENTITIES: PreparationIdentityIssuer = PreparationIdentityIssuer::new();

struct PreparationIdentityIssuer {
    next: AtomicU64,
}

impl WorthUiNativePlatform {
    pub fn prepare(
        profile: UiNativePlatformProfile,
    ) -> Result<UiPreparedNativePlatform, UiNativePlatformPreparationDenial> {
        profile.validate()?;
        Ok(UiPreparedNativePlatform {
            profile,
            preparation_identity: PREPARATION_IDENTITIES.issue()?,
        })
    }
}

impl PreparationIdentityIssuer {
    const fn new() -> Self {
        Self {
            next: AtomicU64::new(1),
        }
    }

    fn issue(&self) -> Result<u64, UiNativePlatformPreparationDenial> {
        self.next
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| UiNativePlatformPreparationDenial::PreparationIdentityExhausted)
    }
}

#[cfg(test)]
mod tests {
    use super::PreparationIdentityIssuer;

    #[test]
    fn preparation_identity_exhaustion_never_reissues_an_identity() {
        let issuer = PreparationIdentityIssuer {
            next: std::sync::atomic::AtomicU64::new(u64::MAX - 1),
        };
        assert_eq!(issuer.issue().unwrap(), u64::MAX - 1);
        assert!(issuer.issue().is_err());
        assert!(issuer.issue().is_err());
    }
}
