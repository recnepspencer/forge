use super::super::read_repository_document;
use super::wal_source_syntax::ParsedRustSource;

const CONSTRUCTION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                            instance/construction.rs";
const BOOTSTRAP: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         instance/durability_bootstrap.rs";
const OWNER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/\
                     admission/platform_basis_join.rs";
const PUBLICATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/checkpoint/publication.rs";
const COMPACTION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/mutation/idempotency/binding_compaction.rs";
const CHECKPOINT_REOPEN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                durability/checkpoint/reopen/binding_compaction.rs";
const WAL_REOPEN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/wal/inventory/reopen.rs";
const REGISTRY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       durability/mutation/idempotency/registry.rs";
const FATE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/\
                   mutation/idempotency/fate/persisted.rs";

#[test]
fn fresh_process_idempotency_reopen_has_one_ordered_streaming_authority_path() {
    inspect(&sources()).unwrap();
}

#[test]
fn reopen_contract_rejects_authority_and_ordering_bypasses() {
    let source = sources();

    let mut bypass = source.clone();
    bypass.construction = mutate_once(
        &bypass.construction,
        "let reopened = durability_reopen.install(durability);",
        "let reopened = durability;",
    );
    assert!(inspect(&bypass).is_err());

    let mut reordered = source.clone();
    reordered.bootstrap = mutate_once(
        &reordered.bootstrap,
        "let members = inventory.take_members();\n    let rebuilt = rebuild_idempotency(",
        "let rebuilt = rebuild_idempotency(",
    );
    assert!(inspect(&reordered).is_err());

    let mut second_inventory = source.clone();
    second_inventory
        .bootstrap
        .push_str("\nfn bypass() { reopen_wal_inventory(); }\n");
    assert!(inspect(&second_inventory).is_err());

    let mut raw_authority = source.clone();
    raw_authority.owner = mutate_once(
        &raw_authority.owner,
        "impl PhysicalDurabilityRuntimeOwner {",
        "impl PhysicalDurabilityRuntimeOwner {\nfn idempotency_authority(&self) {}",
    );
    assert!(inspect(&raw_authority).is_err());
}

#[test]
fn reopen_contract_rejects_buffering_inspection_and_fate_bypasses() {
    let source = sources();

    let mut buffered_compaction = source.clone();
    buffered_compaction.compaction = mutate_once(
        &buffered_compaction.compaction,
        "prior_generation: PhysicalNamespaceDurableCheckpointGeneration,",
        "prior_generation: PhysicalNamespaceDurableCheckpointGeneration,\nrecords: Vec<Box<[u8]>> ,",
    );
    assert!(inspect(&buffered_compaction).is_err());

    let mut buffered_reopen = source.clone();
    buffered_reopen.checkpoint_reopen = mutate_once(
        &buffered_reopen.checkpoint_reopen,
        "let mut records_read = 0_u64;",
        "let mut records_read = 0_u64;\nlet records: Vec<Box<[u8]>> = Vec::new();",
    );
    assert!(inspect(&buffered_reopen).is_err());

    let mut weaker_wal_inspection = source.clone();
    weaker_wal_inspection.wal_reopen = mutate_once(
        &weaker_wal_inspection.wal_reopen,
        "let admitted = interrupted_active_tail::inspect(",
        "let admitted = inspect_complete_wal_segment(",
    );
    assert!(inspect(&weaker_wal_inspection).is_err());

    let mut unadmitted_successor = source.clone();
    unadmitted_successor.wal_reopen = mutate_once(
        &unadmitted_successor.wal_reopen,
        "candidate.admit_after(previous)?",
        "candidate",
    );
    assert!(inspect(&unadmitted_successor).is_err());

    let mut embedded_fate = source;
    embedded_fate.fate.clear();
    assert!(inspect(&embedded_fate).is_err());
}

#[test]
fn generation_and_fate_contract_rejects_premature_authority_mutants() {
    let source = sources();

    let mut staged_advancement = source.clone();
    staged_advancement.publication.push_str(
        "\nfn staged_bypass(binding_cutover: Cutover, namespace_sync: Sync) { \
         binding_cutover.commit_namespace_durable(&namespace_sync); }\n",
    );
    assert!(inspect(&staged_advancement).is_err());

    let mut rename_only_advancement = source.clone();
    rename_only_advancement.publication = mutate_once(
        &rename_only_advancement.publication,
        ".commit_namespace_durable(&namespace_sync)",
        ".commit_namespace_durable(&replacement)",
    );
    assert!(inspect(&rename_only_advancement).is_err());

    let mut omitted_fate = source.clone();
    omitted_fate.compaction = mutate_once(
        &omitted_fate.compaction,
        ".then(|| encode_terminal(basis, fate))",
        ".then(|| None?)",
    );
    assert!(inspect(&omitted_fate).is_err());

    let mut premature_expiry = source;
    premature_expiry.fate = mutate_once(
        &premature_expiry.fate,
        "lease.is_expired_at(generation) && last_compacted.is_some()",
        "lease.is_expired_at(generation)",
    );
    assert!(inspect(&premature_expiry).is_err());
}

#[derive(Clone)]
struct ReopenSources {
    construction: String,
    bootstrap: String,
    owner: String,
    publication: String,
    compaction: String,
    checkpoint_reopen: String,
    wal_reopen: String,
    registry: String,
    fate: String,
}

fn sources() -> ReopenSources {
    ReopenSources {
        construction: read(CONSTRUCTION),
        bootstrap: read(BOOTSTRAP),
        owner: read(OWNER),
        publication: read(PUBLICATION),
        compaction: read(COMPACTION),
        checkpoint_reopen: read(CHECKPOINT_REOPEN),
        wal_reopen: read(WAL_REOPEN),
        registry: read(REGISTRY),
        fate: read(FATE),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path)
        .unwrap_or_else(|error| panic!("{error}"))
        .replace("\r\n", "\n")
}

fn inspect(source: &ReopenSources) -> Result<(), String> {
    inspect_construction(&source.construction, &source.bootstrap)?;
    inspect_authority(&source.owner)?;
    inspect_generation_cutover(&source.publication)?;
    inspect_streaming(&source.compaction, &source.checkpoint_reopen)?;
    inspect_wal(&source.wal_reopen)?;
    inspect_fate(&source.registry, &source.compaction, &source.fate)?;
    Ok(())
}

fn inspect_construction(construction: &str, bootstrap: &str) -> Result<(), &'static str> {
    let construction = compact(construction);
    if !contains_in_order(
        &construction,
        &[
            "reopen_durability_basis(",
            "durability_reopen.install(durability)",
            ".install(&installed_work,&reopened)",
        ],
    ) {
        return Err("record serving can install before durability reconstruction");
    }
    let bootstrap = compact(bootstrap);
    if bootstrap.matches("reopen_wal_inventory(").count() != 1
        || !contains_in_order(
            &bootstrap,
            &[
                "reopen_binding_compaction(media)",
                "letcutoff=matchcheckpoint",
                "reopen_wal_inventory(media,observation.wal_policy(),cutoff)",
                "inventory.take_members()",
                "rebuild_idempotency(",
                "PhysicalWalRuntimeOwner::from_reopened(",
            ],
        )
    {
        return Err("fresh-process durability no longer joins one checkpoint and WAL basis");
    }
    Ok(())
}

fn inspect_authority(source: &str) -> Result<(), &'static str> {
    let raw = impl_body(source, "impl PhysicalDurabilityRuntimeOwner {")
        .ok_or("raw durability owner implementation absent")?;
    for forbidden in [
        "idempotency_authority(",
        "binding_compaction_authority(",
        "grouping_authority(",
    ] {
        if raw.contains(forbidden) {
            return Err("pre-reopen durability owner exposes serving authority");
        }
    }
    let reopened = impl_body(source, "impl ReopenedPhysicalDurabilityRuntimeOwner {")
        .ok_or("reopened durability owner implementation absent")?;
    for required in [
        "idempotency_authority(",
        "binding_compaction_authority(",
        "grouping_authority(",
    ] {
        if !reopened.contains(required) {
            return Err("post-reopen durability owner lost required authority");
        }
    }
    Ok(())
}

fn inspect_generation_cutover(source: &str) -> Result<(), &'static str> {
    let compacted = compact(source);
    if compacted.matches("commit_namespace_durable(").count() != 1
        || !contains_in_order(
            &compacted,
            &[
                "PhysicalCheckpointWorkAction::SynchronizeNamespace",
                ".map_err(PhysicalCheckpointNamespaceFinalizationFailure::Action)?",
                ".commit_namespace_durable(&namespace_sync)",
            ],
        )
    {
        return Err("durable generation can advance without one namespace-sync completion");
    }
    Ok(())
}

fn inspect_streaming(compaction: &str, reopen: &str) -> Result<(), &'static str> {
    let compacted = compact(compaction);
    if compacted.contains("records:Vec<Box<[u8]>>")
        || !compacted.contains("fnfor_each_record<E>(")
        || !compacted.contains("consume(&encoded)?")
    {
        return Err("binding compaction is no longer incrementally emitted");
    }
    let reopen = compact(reopen);
    if reopen.contains("Vec<Box<[u8]>>")
        || !contains_in_order(
            &reopen,
            &[
                "whileoffset<self.footer_offset",
                "CheckpointBindingRecordFrameLength::decode_prefix",
                "consume(payload)",
                "offset=end",
            ],
        )
    {
        return Err("checkpoint bindings are no longer incrementally reopened");
    }
    Ok(())
}

fn inspect_wal(source: &str) -> Result<(), String> {
    let syntax = ParsedRustSource::parse(source, "WAL reopen owner")?;
    let reopen = syntax.function("reopen_wal_inventory")?;
    reopen.require_exact("call:inspect", 1)?;
    reopen.deny("call:inspect_complete_wal_segment")?;
    reopen.require_exact("method:admit_after", 1)?;
    reopen.require_exact("method:frames", 1)?;
    reopen.require_in_order(&["call:inspect", "method:admit_after", "method:frames"])
}

fn inspect_fate(registry: &str, compaction: &str, fate: &str) -> Result<(), &'static str> {
    if !compact(registry).contains("fate:PersistedPhysicalMutationFate") {
        return Err("terminal idempotency state embeds fate outside its semantic seam");
    }
    if !compact(compaction).contains(".then(||encode_terminal(basis,fate))") {
        return Err("retained terminal fate can disappear from binding compaction");
    }
    let fate = compact(fate);
    for required in [
        "enumPersistedPhysicalMutationFate",
        "ProvenNoEffect(ProvenNoEffectPhysicalMutation)",
        "Completed(PersistedCompletedPhysicalMutation)",
        "Indeterminate(PersistedIndeterminatePhysicalMutation)",
        "fnduplicate_observation(",
        "fnencode(&self",
        "fndecode(",
        "requires_compaction_at(",
        "reclamation_eligible_at(",
        "lease.is_expired_at(generation)&&last_compacted.is_some()",
    ] {
        if !fate.contains(required) {
            return Err("persisted mutation fate seam is incomplete");
        }
    }
    Ok(())
}

fn mutate_once(source: &str, from: &str, to: &str) -> String {
    assert_eq!(
        source.matches(from).count(),
        1,
        "controlled mutant anchor must occur exactly once: {from}"
    );
    source.replacen(from, to, 1)
}

fn impl_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    let start = source.find(signature)?;
    let open = source[start..].find('{')? + start;
    let mut depth = 0_u32;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn contains_in_order(source: &str, required: &[&str]) -> bool {
    let mut offset = 0;
    required.iter().all(|needle| {
        let Some(found) = source[offset..].find(needle) else {
            return false;
        };
        offset += found + needle.len();
        true
    })
}
