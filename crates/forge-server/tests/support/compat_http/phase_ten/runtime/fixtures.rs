use forge_server::{ForgeServerMultipartUpload, ForgeServerUploadManifest, ForgeServerUploadPart};
use serde_json::{json, Map, Value};

pub(crate) fn canonical_upload(identity: &str) -> ForgeServerMultipartUpload {
    ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": {
                    "identity.id": identity,
                    "title.value": format!("Title for {identity}")
                }
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(11)
            .with_body_bytes(b"hello-world".to_vec()),
    )
}

pub(crate) fn reordered_canonical_upload(identity: &str) -> ForgeServerMultipartUpload {
    ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "collection": "Task",
                "aspects": {
                    "title.value": format!("Title for {identity}"),
                    "identity.id": identity
                },
                "family": "insert"
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(11)
            .with_body_bytes(b"hello-world".to_vec()),
    )
}

pub(crate) fn ambiguous_metadata_upload() -> ForgeServerMultipartUpload {
    let mut aspects = Map::new();
    aspects.insert("title.value".to_string(), Value::String("safe".to_string()));
    aspects.insert(
        " Title.Value ".to_string(),
        Value::String("unsafe".to_string()),
    );

    ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": Value::Object(aspects)
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(11)
            .with_body_bytes(b"hello-world".to_vec()),
    )
}

pub(crate) fn non_ascii_metadata_upload() -> ForgeServerMultipartUpload {
    let mut aspects = Map::new();
    aspects.insert(
        "títle.value".to_string(),
        Value::String("unsafe".to_string()),
    );

    ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": Value::Object(aspects)
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(11)
            .with_body_bytes(b"hello-world".to_vec()),
    )
}
