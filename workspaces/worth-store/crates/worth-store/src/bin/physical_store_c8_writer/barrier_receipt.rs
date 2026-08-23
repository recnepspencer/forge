const MAGIC: &[u8] = b"WORTH-C8-CHECKPOINT-BARRIER-V1\n";

pub(crate) fn write(path: &std::path::Path, stage: &str) -> Result<(), String> {
    let posture = match stage {
        "candidate-footer" => "footer-append-settled-before-candidate-synchronization",
        "candidate-synchronization" => "candidate-synchronization-settled-after-durability",
        "candidate-publication" => "candidate-publication-settled-before-namespace-sync",
        "namespace-synchronization" => "namespace-synchronization-settled",
        "candidate-creation" => "candidate-creation-settled",
        "candidate-append" => "candidate-append-settled",
        "candidate-binding-header" => "candidate-binding-header-settled",
        "candidate-binding-record" => "candidate-binding-record-settled",
        other => return Err(format!("unknown C8 barrier stage `{other}`")),
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(stage.as_bytes());
    bytes.push(b'\n');
    bytes.extend_from_slice(posture.as_bytes());
    bytes.push(b'\n');
    std::fs::write(path, bytes).map_err(|error| format!("write C8 barrier receipt: {error}"))
}
