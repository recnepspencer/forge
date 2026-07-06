//! Pre-decode gate surface: damage classification before logical decode.
//!
//! [`super::pre_decode_denial::PreDecodePhysicalDenial`] is observed at this boundary.
//! Cross-crate blob handoff uses [`super::damage_handoff::classify_physical_damage_for_handoff`]
//! without minting blob quarantine authority.