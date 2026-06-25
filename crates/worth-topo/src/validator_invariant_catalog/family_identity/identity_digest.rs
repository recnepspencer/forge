pub(in crate::validator_invariant_catalog) fn legality_family_identity_digest(
    parts: &[&str],
) -> String {
    let mut digest = "worth-topo-legality-family-identity-v1".to_string();
    for part in parts {
        digest.push('|');
        digest.push_str(&(part.len()).to_string());
        digest.push(':');
        digest.push_str(part);
    }
    digest
}
