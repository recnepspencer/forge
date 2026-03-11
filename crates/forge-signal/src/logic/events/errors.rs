use crate::data::error::SignalError;
use crate::data::event_subscriber::SubscriberId;
use crate::logic::events::runtime::CompletedSubscriber;

/// Registration-time DAG validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriberRegistryError<D: Copy + Ord + std::fmt::Debug + 'static> {
    DuplicateSubscriberId {
        id: SubscriberId,
        first: &'static str,
        second: &'static str,
    },
    DuplicateProvider {
        data_id: D,
        first: &'static str,
        second: &'static str,
    },
    MissingProvider {
        subscriber: &'static str,
        data_id: D,
    },
    CycleDetected {
        cycle_chain: Vec<&'static str>,
    },
}

/// Flush-time failure for event bus execution.
#[derive(Debug)]
pub enum EventFlushError<D: Copy + Ord + std::fmt::Debug + 'static> {
    Registry(SubscriberRegistryError<D>),
    Subscriber {
        subscriber_id: SubscriberId,
        subscriber_name: &'static str,
        completed_subscribers: Vec<CompletedSubscriber>,
        failed_subscriber_requires: Vec<String>,
        failed_subscriber_provides: Vec<String>,
        failed_subscriber_staged: Vec<String>,
        source: SignalError,
    },
}

impl<D: Copy + Ord + std::fmt::Debug + 'static> std::fmt::Display for EventFlushError<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registry(err) => write!(f, "subscriber registry invalid during flush: {err:?}"),
            Self::Subscriber {
                subscriber_id,
                subscriber_name,
                completed_subscribers,
                failed_subscriber_requires,
                failed_subscriber_provides,
                failed_subscriber_staged,
                source,
            } => {
                write!(
                    f,
                    "subscriber {} (id={}) failed after {:?} with requires {:?}, provides {:?}, staged {:?}: {}",
                    subscriber_name,
                    subscriber_id.get(),
                    completed_subscribers
                        .iter()
                        .map(|subscriber| subscriber.name)
                        .collect::<Vec<_>>(),
                    failed_subscriber_requires,
                    failed_subscriber_provides,
                    failed_subscriber_staged,
                    source
                )
            }
        }
    }
}

impl<D: Copy + Ord + std::fmt::Debug + 'static> std::error::Error for EventFlushError<D> {}
