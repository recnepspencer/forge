use std::path::Path;

pub(crate) fn assert_barrier_receipt(path: &Path, stage: &str) {
    let bytes = std::fs::read(path).expect("checkpoint barrier receipt");
    let text = String::from_utf8(bytes).expect("checkpoint barrier receipt is UTF-8");
    let expected = match stage {
        "candidate-footer" => "footer-append-settled-before-candidate-synchronization",
        "candidate-synchronization" => "candidate-synchronization-settled-after-durability",
        "candidate-publication" => "candidate-publication-settled-before-namespace-sync",
        "namespace-synchronization" => "namespace-synchronization-settled",
        "candidate-creation" => "candidate-creation-settled",
        "candidate-append" => "candidate-append-settled",
        "candidate-binding-header" => "candidate-binding-header-settled",
        "candidate-binding-record" => "candidate-binding-record-settled",
        other => panic!("unrecognized checkpoint barrier stage {other}"),
    };
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("WORTH-C8-CHECKPOINT-BARRIER-V1"));
    assert_eq!(lines.next(), Some(stage));
    assert_eq!(lines.next(), Some(expected));
    assert!(lines.next().is_none());
}
