use super::super::*;

#[test]
fn staged_name_grammar_rejects_ambiguous_spellings() {
    let attempt = NamespaceInitializationAttempt::from_nonzero_bytes([0xab; 16]).unwrap();
    let name = StagedNamespaceName::for_identity(attempt);
    assert_eq!(
        StagedNamespaceName::parse(name.as_str()),
        Some(name.clone())
    );
    assert!(StagedNamespaceName::parse("identity-ABAB.staged").is_none());
    assert!(
        StagedNamespaceName::parse("identity-00000000000000000000000000000000.staged").is_none()
    );
    assert!(StagedNamespaceName::parse("../identity-ab.staged").is_none());
}
