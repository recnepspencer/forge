use super::ledger_shape_assertions::assert_event_ledger_shape;
use super::subject::MetabossEventExtractionSubject;

pub(crate) fn assert_replay_preserves_event_ledger_identity(
    first: &MetabossEventExtractionSubject,
    second: &MetabossEventExtractionSubject,
) {
    assert_event_ledger_shape(first);
    assert_event_ledger_shape(second);
    assert_eq!(
        first.ledger().event_ledger_identity(),
        second.ledger().event_ledger_identity()
    );
    assert_eq!(
        first.ledger().downstream_consumption_identity(),
        second.ledger().downstream_consumption_identity()
    );
    assert_eq!(
        first.ledger().ordered_events().event_group_identities(),
        second.ledger().ordered_events().event_group_identities()
    );
    assert_eq!(
        first.ledger().ordered_events().point_event_identities(),
        second.ledger().ordered_events().point_event_identities()
    );
    assert_eq!(
        first.ledger().ordered_events().interval_event_identities(),
        second.ledger().ordered_events().interval_event_identities()
    );
    assert_eq!(
        first
            .ledger()
            .ordered_events()
            .relation_diagnostic_identities(),
        second
            .ledger()
            .ordered_events()
            .relation_diagnostic_identities()
    );
    assert_eq!(first.ledger().counters(), second.ledger().counters());
}
