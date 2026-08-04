use super::super::wal_source_syntax::ParsedRustSource;

pub(super) fn inspect(group_port: &str, runtime_owner: &str) -> Result<(), String> {
    inspect_continuation(group_port)?;
    inspect_runtime_release_owners(runtime_owner)
}

fn inspect_continuation(source: &str) -> Result<(), String> {
    let source = ParsedRustSource::parse(source, "WAL group continuation owner")?;
    let continuation = source.function("continue_prepared_group")?;
    continuation.require_in_order(&[
        "path:Admitted",
        "method:reserve_and_append_group",
        "path:Reserved",
        "method:append_reserved_group",
    ])?;

    let append = source.function("append_reserved_group")?;
    append.deny("method:reserve_and_append_group")?;
    append.require_exact("method:finish_group", 1)?;
    append.require_in_order(&[
        "method:pop_front",
        "method:append_group_member",
        "method:is_empty",
        "method:release_group_before_effect",
        "call:from_members",
        "method:release_after_no_effect",
        "path:Reserved",
        "method:finish_group",
        "call:seal",
    ])
}

fn inspect_runtime_release_owners(source: &str) -> Result<(), String> {
    let source = ParsedRustSource::parse(source, "WAL runtime lifecycle owner")?;
    source
        .function("complete_member")?
        .deny("assign:in_flight=false")?;
    source
        .function("finish_group")?
        .require_exact("assign:in_flight=false", 1)?;
    source
        .function("release_group_before_effect")?
        .require_exact("assign:in_flight=false", 1)?;
    let seal = source.function("seal_for_inspection")?;
    seal.require_exact("assign:in_flight=false", 1)?;
    seal.require_exact("assign:sealed=true", 1)
}
