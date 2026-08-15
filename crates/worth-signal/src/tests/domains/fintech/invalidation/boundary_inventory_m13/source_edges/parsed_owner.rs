use quote::ToTokens;
use sha2::{Digest, Sha256};
use syn::{ImplItem, Item};

use super::owner_bodies::{OwnerBody, OwnerKind};
use super::source_files::SourceFile;

pub(super) fn assert_owner_body_changed(owner: &OwnerBody, source: &str) {
    assert_ne!(owner_body_digest(owner, source), owner.expected_digest);
}

pub(super) fn assert_source_file_changed(source_file: &SourceFile, source: &str) {
    assert_ne!(
        source_file_digest(source_file, source),
        source_file.expected_digest
    );
}

pub(super) fn source_file_digest(source_file: &SourceFile, source: &str) -> String {
    let file = syn::parse_file(source)
        .unwrap_or_else(|error| panic!("{} no longer parses: {error}", source_file.source_path));
    let normalized = file.to_token_stream().to_string();
    let digest = Sha256::digest(normalized.as_bytes());
    format!("{digest:x}")
}

pub(super) fn owner_body_digest(owner: &OwnerBody, source: &str) -> String {
    let file = syn::parse_file(source).unwrap_or_else(|error| {
        panic!(
            "{} ({}) no longer parses: {error}",
            owner.responsibility, owner.source_path
        )
    });
    let mut bodies = Vec::new();
    for item in file.items {
        match (&owner.kind, item) {
            (OwnerKind::Function(expected), Item::Fn(item)) if item.sig.ident == *expected => {
                bodies.push(item.block.to_token_stream().to_string());
            }
            (OwnerKind::Method(expected), Item::Impl(item)) => {
                for member in item.items {
                    if let ImplItem::Fn(method) = member {
                        if method.sig.ident == *expected {
                            bodies.push(method.block.to_token_stream().to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    assert_eq!(
        bodies.len(),
        1,
        "{} ({}) must resolve to exactly one parsed owner named {}",
        owner.responsibility,
        owner.source_path,
        owner.kind.name()
    );
    let digest = Sha256::digest(bodies[0].as_bytes());
    format!("{digest:x}")
}
