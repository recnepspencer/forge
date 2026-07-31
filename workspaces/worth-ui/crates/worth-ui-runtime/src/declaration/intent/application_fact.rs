use core::marker::PhantomData;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::capability::{
    UiIntentBoolean, UiIntentPayloadFieldKind, UiIntentPayloadValueKind, UiIntentText,
    UiIntentUnsigned64, UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiIntentApplicationFact<K: UiIntentPayloadValueKind> {
    identity: Arc<str>,
    text_byte_budget: usize,
    kind: PhantomData<fn() -> K>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentApplicationFactIdentityError {
    InvalidIdentity,
    InvalidTextByteBudget { observed: usize, maximum: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentApplicationFactRegistrationError {
    DuplicateIdentity {
        identity: Box<str>,
    },
    TextBudgetExceeded {
        identity: Box<str>,
        observed: usize,
        maximum: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiIntentApplicationFactPlan {
    entries: BTreeMap<Arc<str>, UiIntentApplicationFactDefinition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiIntentApplicationFactDefinition {
    slot: UiIntentApplicationFactSlot,
    kind: UiIntentPayloadFieldKind,
    text_byte_budget: usize,
    initial: UiIntentApplicationFactValue,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiIntentApplicationFactSlot(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiIntentApplicationFactValue {
    Text(Arc<str>),
    Boolean(bool),
    Unsigned64(u64),
}

impl UiIntentApplicationFact<UiIntentText> {
    pub fn text(
        identity: impl Into<String>,
        byte_budget: usize,
    ) -> Result<Self, UiIntentApplicationFactIdentityError> {
        if byte_budget == 0 || byte_budget > UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT {
            return Err(
                UiIntentApplicationFactIdentityError::InvalidTextByteBudget {
                    observed: byte_budget,
                    maximum: UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT,
                },
            );
        }
        Self::new(identity, byte_budget)
    }
}

impl UiIntentApplicationFact<UiIntentBoolean> {
    pub fn boolean(
        identity: impl Into<String>,
    ) -> Result<Self, UiIntentApplicationFactIdentityError> {
        Self::new(identity, 0)
    }
}

impl UiIntentApplicationFact<UiIntentUnsigned64> {
    pub fn unsigned64(
        identity: impl Into<String>,
    ) -> Result<Self, UiIntentApplicationFactIdentityError> {
        Self::new(identity, 0)
    }
}

impl<K: UiIntentPayloadValueKind> UiIntentApplicationFact<K> {
    fn new(
        identity: impl Into<String>,
        text_byte_budget: usize,
    ) -> Result<Self, UiIntentApplicationFactIdentityError> {
        let identity = identity.into();
        if !valid_identity(&identity) {
            return Err(UiIntentApplicationFactIdentityError::InvalidIdentity);
        }
        Ok(Self {
            identity: Arc::from(identity),
            text_byte_budget,
            kind: PhantomData,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub const fn kind(&self) -> UiIntentPayloadFieldKind {
        K::KIND
    }
}

impl Default for UiIntentApplicationFactPlan {
    fn default() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
}

impl UiIntentApplicationFactPlan {
    pub(crate) fn register_text(
        &mut self,
        fact: UiIntentApplicationFact<UiIntentText>,
        value: impl Into<Arc<str>>,
    ) -> Result<(), UiIntentApplicationFactRegistrationError> {
        let value = value.into();
        if value.len() > fact.text_byte_budget {
            return Err(
                UiIntentApplicationFactRegistrationError::TextBudgetExceeded {
                    identity: fact.identity().into(),
                    observed: value.len(),
                    maximum: fact.text_byte_budget,
                },
            );
        }
        self.register(
            fact.identity,
            UiIntentPayloadFieldKind::Text,
            fact.text_byte_budget,
            UiIntentApplicationFactValue::Text(value),
        )
    }

    pub(crate) fn register_boolean(
        &mut self,
        fact: UiIntentApplicationFact<UiIntentBoolean>,
        value: bool,
    ) -> Result<(), UiIntentApplicationFactRegistrationError> {
        self.register(
            fact.identity,
            UiIntentPayloadFieldKind::Boolean,
            0,
            UiIntentApplicationFactValue::Boolean(value),
        )
    }

    pub(crate) fn register_unsigned64(
        &mut self,
        fact: UiIntentApplicationFact<UiIntentUnsigned64>,
        value: u64,
    ) -> Result<(), UiIntentApplicationFactRegistrationError> {
        self.register(
            fact.identity,
            UiIntentPayloadFieldKind::Unsigned64,
            0,
            UiIntentApplicationFactValue::Unsigned64(value),
        )
    }

    pub(crate) fn get(&self, identity: &str) -> Option<&UiIntentApplicationFactDefinition> {
        self.entries.get(identity)
    }

    pub(crate) fn entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (&Arc<str>, &UiIntentApplicationFactDefinition)> {
        self.entries.iter()
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        fold_u64(&mut digest, self.entries.len() as u64);
        for (identity, definition) in &self.entries {
            fold_text(&mut digest, identity);
            fold_text(&mut digest, definition.kind.digest_basis());
            fold_u64(&mut digest, definition.text_byte_budget as u64);
            definition.initial.fold_into(&mut digest);
        }
        digest
    }

    fn register(
        &mut self,
        identity: Arc<str>,
        kind: UiIntentPayloadFieldKind,
        text_byte_budget: usize,
        initial: UiIntentApplicationFactValue,
    ) -> Result<(), UiIntentApplicationFactRegistrationError> {
        if self.entries.contains_key(&identity) {
            return Err(
                UiIntentApplicationFactRegistrationError::DuplicateIdentity {
                    identity: identity.as_ref().into(),
                },
            );
        }
        let slot = UiIntentApplicationFactSlot::from_index(self.entries.len())
            .expect("the application fact plan cannot exceed addressable memory");
        self.entries.insert(
            identity,
            UiIntentApplicationFactDefinition {
                slot,
                kind,
                text_byte_budget,
                initial,
            },
        );
        Ok(())
    }
}

impl UiIntentApplicationFactDefinition {
    pub(crate) const fn slot(&self) -> UiIntentApplicationFactSlot {
        self.slot
    }

    pub(crate) const fn kind(&self) -> UiIntentPayloadFieldKind {
        self.kind
    }

    pub(crate) const fn text_byte_budget(&self) -> usize {
        self.text_byte_budget
    }

    pub(crate) const fn initial(&self) -> &UiIntentApplicationFactValue {
        &self.initial
    }
}

impl UiIntentApplicationFactSlot {
    fn from_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().map(Self)
    }

    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

fn valid_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
}

impl UiIntentApplicationFactValue {
    fn fold_into(&self, digest: &mut u64) {
        match self {
            Self::Text(value) => {
                fold_u64(digest, 0);
                fold_text(digest, value);
            }
            Self::Boolean(value) => {
                fold_u64(digest, 1);
                fold_u64(digest, u64::from(*value));
            }
            Self::Unsigned64(value) => {
                fold_u64(digest, 2);
                fold_u64(digest, *value);
            }
        }
    }
}

fn fold_text(digest: &mut u64, value: &str) {
    fold_u64(digest, value.len() as u64);
    for byte in value.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

fn fold_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}
