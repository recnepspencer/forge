use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::capture::{EventCategory, EventRecord, EventStreamRecord};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedEvent<TargetId = String> {
    pub category: EventCategory,
    pub target: Option<TargetId>,
    pub detail: Option<String>,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSubscription<TargetId = String> {
    pub categories: BTreeSet<EventCategory>,
    pub target: Option<TargetId>,
}

impl<TargetId> EventSubscription<TargetId> {
    pub fn new() -> Self {
        Self {
            categories: BTreeSet::new(),
            target: None,
        }
    }

    pub fn category(mut self, category: EventCategory) -> Self {
        self.categories.insert(category);
        self
    }

    pub fn target(mut self, target: TargetId) -> Self {
        self.target = Some(target);
        self
    }
}

impl<TargetId> Default for EventSubscription<TargetId> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn flatten_event_streams<TargetId: Clone>(
    streams: &[EventStreamRecord<TargetId>],
) -> Vec<EventRecord<TargetId>> {
    streams
        .iter()
        .flat_map(|stream| stream.events.iter().cloned())
        .collect()
}

pub fn filter_events<TargetId: Clone, Predicate>(
    events: &[EventRecord<TargetId>],
    predicate: Predicate,
) -> Vec<EventRecord<TargetId>>
where
    Predicate: Fn(&EventRecord<TargetId>) -> bool,
{
    events
        .iter()
        .filter(|event| predicate(event))
        .cloned()
        .collect()
}

pub fn select_events<TargetId: Clone + PartialEq>(
    events: &[EventRecord<TargetId>],
    subscription: &EventSubscription<TargetId>,
) -> Vec<EventRecord<TargetId>> {
    filter_events(events, |event| {
        let category_match =
            subscription.categories.is_empty() || subscription.categories.contains(&event.category);
        let target_match = match (&subscription.target, &event.target) {
            (Some(expected), Some(actual)) => expected == actual,
            (Some(_), None) => false,
            (None, _) => true,
        };
        category_match && target_match
    })
}

pub fn group_events_by_category<TargetId: Clone>(
    events: &[EventRecord<TargetId>],
) -> BTreeMap<EventCategory, Vec<EventRecord<TargetId>>> {
    let mut grouped = BTreeMap::new();
    for event in events {
        grouped
            .entry(event.category)
            .or_insert_with(Vec::new)
            .push(event.clone());
    }
    grouped
}

pub fn project_events<TargetId: Clone>(
    events: &[EventRecord<TargetId>],
) -> Vec<ProjectedEvent<TargetId>> {
    events
        .iter()
        .map(|event| ProjectedEvent {
            category: event.category,
            target: event.target.clone(),
            detail: event.detail.clone(),
            fields: event.fields.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use crate::capture::{EventCategory, EventRecord, EventStreamRecord, RecordSchemaVersion};
    use crate::identity::{event_stream_id, run_id, scenario_id};

    use super::{
        filter_events, flatten_event_streams, group_events_by_category, project_events,
        select_events, EventSubscription,
    };

    #[test]
    fn event_projection_helpers_shape_event_views() {
        let event = EventRecord {
            schema_version: RecordSchemaVersion::V1,
            adapter_name: "double".to_string(),
            category: EventCategory::ExecutionFinished,
            target: Some("target".to_string()),
            detail: Some("done".to_string()),
            time_marker: None,
            feed_batch: None,
            fields: BTreeMap::from([("status".to_string(), json!("ok"))]),
        };
        let stream = EventStreamRecord {
            schema_version: RecordSchemaVersion::V1,
            event_stream_id: event_stream_id(
                &run_id(&scenario_id("fixture"), "profile", "request"),
                "main",
            ),
            run_id: run_id(&scenario_id("fixture"), "profile", "request"),
            adapter_name: "double".to_string(),
            stream_name: "main".to_string(),
            time_marker: None,
            feed_batch: None,
            events: vec![event.clone()],
            attachments: Vec::new(),
        };

        let flattened = flatten_event_streams(&[stream]);
        let filtered = filter_events(&flattened, |entry| entry.target.is_some());
        let selected = select_events(
            &filtered,
            &EventSubscription::new()
                .category(EventCategory::ExecutionFinished)
                .target("target".to_string()),
        );
        let grouped = group_events_by_category(&filtered);
        let projected = project_events(&filtered);

        assert_eq!(flattened.len(), 1);
        assert_eq!(filtered.len(), 1);
        assert_eq!(selected.len(), 1);
        assert_eq!(
            grouped
                .get(&EventCategory::ExecutionFinished)
                .unwrap()
                .len(),
            1
        );
        assert_eq!(projected[0].detail.as_deref(), Some("done"));
    }
}
