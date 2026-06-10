use forge_server::{
    ForgeServerBinaryCounterSet, ForgeServerCompatibilityExport, ForgeServerCompatibilityStream,
    ForgeServerExternalCounterSet,
};

pub(crate) fn assert_external_counter(
    counters: &ForgeServerExternalCounterSet,
    name: &str,
    expected: u64,
) {
    assert_eq!(
        counters.counter(name),
        Some(expected),
        "expected external counter `{name}` to equal {expected}",
    );
}

pub(crate) fn assert_binary_counter(
    counters: &ForgeServerBinaryCounterSet,
    name: &str,
    expected: u64,
) {
    assert_eq!(
        counters.counter(name),
        Some(expected),
        "expected binary counter `{name}` to equal {expected}",
    );
}

pub(crate) fn finish_stream(
    mut stream: ForgeServerCompatibilityStream,
) -> ForgeServerCompatibilityExport {
    loop {
        match stream.next_chunk().expect("stream chunk should serialize") {
            Some(_) => continue,
            None => break,
        }
    }
    stream
        .finish()
        .expect("stream should finish after full consumption")
}
