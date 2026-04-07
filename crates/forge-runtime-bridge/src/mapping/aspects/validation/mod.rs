mod order;
mod set;
mod values;

pub(super) use order::canonical_aspect_registration_order;
pub(super) use set::validate_registration_set;
pub(super) use values::validate_registration_values;

#[cfg(test)]
pub(super) use set::registration_rank_group;

#[cfg(test)]
mod tests;
