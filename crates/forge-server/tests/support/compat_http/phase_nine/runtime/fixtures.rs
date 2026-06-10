use forge_server::{ForgeServerMultipartUpload, ForgeServerUploadManifest, ForgeServerUploadPart};
use serde_json::json;

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

pub(crate) fn malformed_upload() -> ForgeServerMultipartUpload {
    ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": ""
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(4)
            .with_body_bytes(b"nope".to_vec()),
    )
}
