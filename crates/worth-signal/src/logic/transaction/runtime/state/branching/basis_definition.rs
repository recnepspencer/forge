use super::super::runtime_state::SignalRuntime;

pub(crate) fn signal_definition_basis<D, I, E, Ctx, T>(
    runtime: &SignalRuntime<D, I, E, Ctx, T>,
) -> u64
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime
        .schema_registry
        .registry_digest()
        .as_bytes()
        .iter()
        .take(8)
        .fold(0_u64, |value, byte| {
            value.wrapping_mul(257).wrapping_add(u64::from(*byte))
        })
}
