//! Raw strings cannot satisfy blob chunk security scope:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkSecurityScope;
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let tenant = "tenant-a";
//! requires_scope(tenant);
//! ```
//! JWT claims cannot satisfy blob chunk security scope:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkSecurityScope;
//! use worth_store_security::StoreJwtSubjectClaim;
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let claim = StoreJwtSubjectClaim::raw("subject");
//! requires_scope(claim);
//! ```
//! KMS ids cannot satisfy blob chunk security scope:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkSecurityScope;
//! use worth_store_security::StoreKmsKeyIdentifier;
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let kms = StoreKmsKeyIdentifier::raw("kms-key");
//! requires_scope(kms);
//! ```
//! IAM role names cannot satisfy blob chunk security scope:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkSecurityScope;
//! use worth_store_security::StoreIamRoleClaim;
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let role = StoreIamRoleClaim::raw("role");
//! requires_scope(role);
//! ```
//! Operator identities cannot satisfy blob chunk security scope:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkSecurityScope;
//! use worth_store_security::StoreOperatorIdentityClaim;
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let operator = StoreOperatorIdentityClaim::raw("operator");
//! requires_scope(operator);
//! ```
//! Blob chunk security metadata cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkSecurityMetadataWitness;
//!
//! let _forged = BlobChunkSecurityMetadataWitness {
//!     identity: todo!(),
//!     key_scope: todo!(),
//!     key_version_posture: todo!(),
//!     tenant_scope: todo!(),
//!     authenticity_requirement: todo!(),
//!     custody_posture: todo!(),
//!     receipt: todo!(),
//!     counters: todo!(),
//! };
//! ```
