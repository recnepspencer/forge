/// Exact retained-memory accounting for a typed application effect payload.
///
/// The count includes `size_of::<Self>()` and the capacity of every allocation
/// owned recursively by the value. Shared or borrowed allocations that the
/// payload does not own are not admissible substitutes for this count.
///
/// Implementing this trait grants no emit or delivery authority. Installation
/// and execution still govern which operation may emit the payload and how many
/// retained bytes its installed resource envelope admits.
///
/// An effect cannot be declared around an unmeasured payload:
///
/// ```compile_fail
/// struct Schema;
/// struct UnmeasuredPayload(Vec<u8>);
///
/// worth_query_declaration::worth_query_effect!(
///     pub UnmeasuredEffect(UnmeasuredPayload) in Schema
/// );
/// ```
pub trait ApplicationEffectPayload: Send + Sync + 'static {
    fn retained_bytes(&self) -> u64;
}

impl ApplicationEffectPayload for String {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(std::mem::size_of::<Self>())
            .unwrap_or(u64::MAX)
            .saturating_add(u64::try_from(self.capacity()).unwrap_or(u64::MAX))
    }
}
