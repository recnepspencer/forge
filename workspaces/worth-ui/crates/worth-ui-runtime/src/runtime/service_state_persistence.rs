/// Classification carried by each runtime-service state. It is not persistence,
/// history, aftermath, undo, redo, or inverse-operation authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiServiceStatePersistencePosture {
    Ephemeral,
    SessionRestoreCandidate,
    Effecting,
}

impl UiServiceStatePersistencePosture {
    #[cfg(test)]
    pub(in crate::runtime) const ALL: [Self; 3] = [
        Self::Ephemeral,
        Self::SessionRestoreCandidate,
        Self::Effecting,
    ];
}

#[cfg(test)]
mod tests {
    use super::UiServiceStatePersistencePosture;

    #[test]
    fn service_state_postures_are_classification_without_persistence_authority() {
        assert_eq!(
            UiServiceStatePersistencePosture::ALL,
            [
                UiServiceStatePersistencePosture::Ephemeral,
                UiServiceStatePersistencePosture::SessionRestoreCandidate,
                UiServiceStatePersistencePosture::Effecting,
            ]
        );
    }
}
