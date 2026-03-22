mod stream_execution;

pub(crate) use stream_execution::execute_subscriber_stream;
#[cfg(test)]
pub(crate) use stream_execution::collect_crossed_boundaries;
