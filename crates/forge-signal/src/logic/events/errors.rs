use crate::data::error::SignalError;
use crate::data::event_subscriber::SubscriberId;

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
                source,
            } => {
                write!(
                    f,
                    "subscriber {} (id={}) failed: {}",
                    subscriber_name,
                    subscriber_id.get(),
                    source
                )
            }
        }
    }
}

impl<D: Copy + Ord + std::fmt::Debug + 'static> std::error::Error for EventFlushError<D> {}
