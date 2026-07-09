mod stream_execution;

#[cfg(test)]
pub(crate) use stream_execution::collect_crossed_boundaries;
pub(crate) use stream_execution::execute_subscriber_stream;
