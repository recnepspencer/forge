use super::super::read_repository_document;

const REOPEN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      durability/wal/inventory/reopen.rs";
const INVENTORY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/wal/inventory/live_segment_inventory.rs";

pub(super) fn inspect(reopen: &str, inventory: &str) -> Result<(), &'static str> {
    let reopen = compact(reopen);
    if !reopen.contains(
        "letrequires_inspection=cutoff.lsn().is_none()\
         &&!segment_inventory.retains_canonical_wal_origin();",
    ) {
        return Err("WAL reopen lost its checkpoint-cutoff or canonical-origin classification");
    }

    let origin = compact(
        function_body(inventory, "fn retains_canonical_wal_origin(")
            .ok_or("canonical WAL origin predicate is absent")?,
    );
    for required in [
        "letSome(first)=self.entries.first()else{returnfalse;};",
        "first.identity.segment()==WalSegmentId::new(1)",
        "first.identity.generation()==WalSegmentGeneration::new(1)",
        "first.lsn_range.start()==LogSequenceNumber::new(LogSequenceNumber::GENESIS.get()+1)",
    ] {
        if !origin.contains(required) {
            return Err("canonical WAL origin proof lost segment, generation, or LSN identity");
        }
    }
    Ok(())
}

#[test]
fn wal_reopen_requires_checkpoint_cutoff_or_exact_canonical_origin() {
    inspect(&read(REOPEN), &read(INVENTORY)).unwrap();
}

#[test]
fn wal_reopen_origin_gate_rejects_incomplete_classification_and_origin_proof() {
    let reopen = read(REOPEN);
    let inventory = read(INVENTORY);

    let blanket_sealing = replace_once(
        &reopen,
        "cutoff.lsn().is_none() && !segment_inventory.retains_canonical_wal_origin()",
        "cutoff.lsn().is_none()",
    );
    assert!(inspect(&blanket_sealing, &inventory).is_err());

    let unproved_trust = replace_once(
        &reopen,
        "cutoff.lsn().is_none() && !segment_inventory.retains_canonical_wal_origin()",
        "false",
    );
    assert!(inspect(&unproved_trust, &inventory).is_err());

    for removed in [
        "first.identity.segment()\n            == WalSegmentId::new(1).expect(\"the canonical first WAL segment is nonzero\")\n            && ",
        "first.identity.generation()\n                == WalSegmentGeneration::new(1)\n                    .expect(\"the canonical first WAL generation is nonzero\")\n            && ",
        "&& first.lsn_range.start()\n                == LogSequenceNumber::new(LogSequenceNumber::GENESIS.get() + 1)",
    ] {
        let incomplete = replace_once(&inventory, removed, "");
        assert!(inspect(&reopen, &incomplete).is_err());
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
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

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert_eq!(source.matches(from).count(), 1, "control anchor: {from}");
    source.replacen(from, to, 1)
}
