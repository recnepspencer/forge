mod event;
mod frame;
#[cfg(test)]
mod tests;

pub(crate) use event::PersistedMembershipActivation;
pub(crate) use frame::{decode_activation, encode_activation, has_activation_magic};
