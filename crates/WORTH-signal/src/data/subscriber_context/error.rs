/// Error returned when staged output violates context contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriberContextError<D: Copy + Ord + std::fmt::Debug + 'static> {
    DuplicateStagedDataId { data_id: D },
}
