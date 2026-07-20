//! Physical reference raw construction is sealed:
//! ```compile_fail
//! use worth_store_physical_format::{
//!     PhysicalGeneration, PhysicalPageId, PhysicalRecordSlot, PhysicalReference,
//!     PhysicalSegmentId,
//! };
//!
//! let _forged = PhysicalReference::for_page_slot(
//!     PhysicalSegmentId::from_raw(1).unwrap(),
//!     PhysicalPageId::from_raw(1).unwrap(),
//!     PhysicalRecordSlot::from_raw(1).unwrap(),
//!     PhysicalGeneration::from_raw(1).unwrap(),
//! );
//! ```
//! Admission witnesses are sealed proof values:
//!
//! ```compile_fail
//! use worth_store_physical_format::PhysicalReferenceAdmissionWitness;
//!
//! let _forged = PhysicalReferenceAdmissionWitness { reference: todo!() };
//! ```
//! Generation owners are sealed evidence, not raw diagnostic bags:
//!
//! ```compile_fail
//! use worth_store_physical_format::{
//!     PhysicalCellReuseDomain, PhysicalGeneration, PhysicalGenerationOwner,
//! };
//!
//! let _forged = PhysicalGenerationOwner {
//!     domain: PhysicalCellReuseDomain::SlotAllocation,
//!     generation: PhysicalGeneration::from_raw(1).unwrap(),
//! };
//! ```
//! Page generation cannot substitute for slot generation:
//!
//! ```compile_fail
//! use worth_store_physical_format::{
//!     PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
//!     PhysicalReferenceAuthority, PhysicalSegmentId,
//! };
//!
//! let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
//! let references = PhysicalReferenceAuthority::for_canonical_physical_format();
//! let page_cell = generations
//!     .page_cell(
//!         PhysicalSegmentId::from_raw(1).unwrap(),
//!         PhysicalPageId::from_raw(1).unwrap(),
//!     )
//!     .with_page_generation(PhysicalGeneration::from_raw(1).unwrap());
//!
//! let _ = references.admit_page_slot(page_cell);
//! ```
//! Semantic artifact digests are not physical placement identity:
//!
//! ```compile_fail
//! use worth_store_contracts::StableDigest;
//! use worth_store_physical_format::PhysicalReferenceAuthority;
//!
//! let digest = StableDigest::new("sha256:not-physical-identity").unwrap();
//! let _ = PhysicalReferenceAuthority::for_canonical_physical_format().admit_root_publication(digest);
//! ```
//! Header decode witnesses are sealed proof values:
//!
//! ```compile_fail
//! use worth_store_physical_format::PhysicalHeaderDecodeWitness;
//!
//! let _forged = PhysicalHeaderDecodeWitness {
//!     header: todo!(),
//!     owner: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Payload views cannot be minted directly from raw bytes:
//!
//! ```compile_fail
//! use worth_store_physical_format::{PhysicalPayloadView, PhysicalHeaderDecodeWitness};
//!
//! let raw = b"not admitted payload";
//! let witness: PhysicalHeaderDecodeWitness = todo!();
//! let _forged = PhysicalPayloadView::new(raw, witness);
//! ```
//! Framed record views cannot be minted without record-page admission:
//!
//! ```compile_fail
//! use worth_store_physical_format::{FramedRecordPayload, FramedRecordView};
//!
//! let raw = b"not admitted record";
//! let payload = FramedRecordPayload::new(raw);
//! let _forged = FramedRecordView::new(todo!(), payload, todo!());
//! ```
//! Shortcut boundary denials are emitted by the facade boundary, not minted by
//! public callers:
//!
//! ```compile_fail
//! use worth_store_physical_format::PhysicalShortcutBoundaryDenial;
//!
//! let _forged = PhysicalShortcutBoundaryDenial::live_runtime_cache();
//! ```
//! Shortcut boundary evidence cannot be attached to facade denials outside the
//! physical-format crate:
//!
//! ```compile_fail
//! use worth_store_physical_format::{
//!     PhysicalShortcutBoundaryDenial, InMemoryPhysicalFormatModelDenial,
//!     InMemoryPhysicalFormatModelDenialKind,
//! };
//!
//! let denial = InMemoryPhysicalFormatModelDenial::new(
//!     InMemoryPhysicalFormatModelDenialKind::ShortcutBoundaryRejected,
//! );
//! let shortcut: PhysicalShortcutBoundaryDenial = todo!();
//! let _forged = denial.with_shortcut_denial(shortcut);
//! ```
