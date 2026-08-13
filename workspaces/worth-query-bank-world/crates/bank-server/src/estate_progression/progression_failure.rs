use super::BankEstateProgressionDenial;

/// A failed lifecycle attempt, including the exact phase authority when it remains reusable.
#[derive(Debug)]
pub struct BankEstateProgressionFailure<Authority> {
    denial: BankEstateProgressionDenial,
    authority: Option<Box<Authority>>,
}

impl<Authority> BankEstateProgressionFailure<Authority> {
    pub(super) fn retained(denial: BankEstateProgressionDenial, authority: Authority) -> Self {
        Self {
            denial,
            authority: Some(Box::new(authority)),
        }
    }

    pub(super) const fn consumed(denial: BankEstateProgressionDenial) -> Self {
        Self {
            denial,
            authority: None,
        }
    }

    pub const fn denial(&self) -> &BankEstateProgressionDenial {
        &self.denial
    }

    pub fn into_parts(self) -> (BankEstateProgressionDenial, Option<Authority>) {
        (self.denial, self.authority.map(|authority| *authority))
    }

    pub fn into_denial(self) -> BankEstateProgressionDenial {
        self.denial
    }
}
