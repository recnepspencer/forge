use crate::access::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::access::grammar::PhysicalLayoutAccessFamily;
use crate::{
    ManifestDiscoveryAuthority, ManifestDiscoveryCounterSnapshot, ManifestDiscoveryReport,
    OfflineManifestCodec, OfflineVerifierCounterSnapshot, PhysicalBootstrapCatalogDenial,
    PhysicalHeaderAuthority, PhysicalReferenceAuthority, PhysicalRootManifest,
    PhysicalRootReference, PhysicalStoreRuntime, PhysicalStoreRuntimeDenial,
    PhysicalStoreRuntimeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootManifestLayoutReport {
    root_reference: PhysicalRootReference,
    segment_count: u32,
    page_slot_count: u32,
    extent_count: u32,
    allocation_class_count: u32,
    free_space_count: u32,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalRootManifestAccess {
    root: PhysicalRootManifest,
    manifest_counters: ManifestDiscoveryCounterSnapshot,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug)]
pub struct RootDiscoveryAccess<'a> {
    facade: &'a mut PhysicalStoreRuntime,
}

impl<'a> RootDiscoveryAccess<'a> {
    pub(crate) fn new(facade: &'a mut PhysicalStoreRuntime) -> Self {
        Self { facade }
    }

    pub fn current_root_manifest(
        &mut self,
    ) -> Result<RootManifestLayoutReport, PhysicalStoreRuntimeDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let _ = self.facade.mark_read();
        Ok(RootManifestLayoutReport {
            root_reference: access.root().root_publication().root_reference(),
            segment_count: access.root().segments().len() as u32,
            page_slot_count: access.root().page_slots().len() as u32,
            extent_count: access.root().extents().len() as u32,
            allocation_class_count: access.root().allocation_classes().len() as u32,
            free_space_count: access.root().free_space().len() as u32,
            counters: access.counters(),
        })
    }
}

impl RootManifestLayoutReport {
    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::RootManifest
    }

    pub const fn root_reference(self) -> PhysicalRootReference {
        self.root_reference
    }

    pub const fn segment_count(self) -> u32 {
        self.segment_count
    }

    pub const fn page_slot_count(self) -> u32 {
        self.page_slot_count
    }

    pub const fn extent_count(self) -> u32 {
        self.extent_count
    }

    pub const fn allocation_class_count(self) -> u32 {
        self.allocation_class_count
    }

    pub const fn free_space_count(self) -> u32 {
        self.free_space_count
    }

    pub const fn counters(self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }
}

impl CanonicalRootManifestAccess {
    pub(crate) const fn root(&self) -> &PhysicalRootManifest {
        &self.root
    }

    pub(in crate::access) const fn counters(&self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }

    pub(in crate::access) fn manifest_report(&self) -> ManifestDiscoveryReport<'_> {
        ManifestDiscoveryReport::new(&self.root, self.manifest_counters)
    }

    pub(in crate::access) fn manifest_lookup_counters(
        &self,
    ) -> PhysicalLayoutAccessCounterSnapshot {
        exact_manifest_counters(
            self.counters.bytes_read(),
            self.counters.page_touches(),
            self.manifest_counters.with_manifest_index_probe(),
            0,
        )
    }

    pub(in crate::access) fn manifest_traversal_counters(
        &self,
        extra_range_steps: u32,
    ) -> PhysicalLayoutAccessCounterSnapshot {
        exact_manifest_counters(
            self.counters.bytes_read(),
            self.counters.page_touches(),
            self.manifest_counters,
            extra_range_steps,
        )
    }
}

pub(crate) fn canonical_root_manifest(
    facade: &PhysicalStoreRuntime,
) -> Result<CanonicalRootManifestAccess, PhysicalStoreRuntimeDenial> {
    let witness = facade
        .storage_ref()
        .admit_bootstrap_open_witness(facade.headers_ref())
        .map_err(map_bootstrap_denial)?;
    decode_canonical_root_manifest(&witness, facade.headers_ref())
}

fn decode_canonical_root_manifest(
    witness: &crate::PhysicalBootstrapCatalogOpenWitness,
    headers: &PhysicalHeaderAuthority,
) -> Result<CanonicalRootManifestAccess, PhysicalStoreRuntimeDenial> {
    let mut decode_denial = None;
    let mut discovery_denial = None;
    for root_manifest in witness.root_manifest_candidates() {
        let decoded = match OfflineManifestCodec::decode(
            witness.byte_order(),
            root_manifest,
            witness.segment_manifest(),
            witness.extent_manifest(),
            witness.free_space_map(),
            OfflineVerifierCounterSnapshot::empty(),
        ) {
            Ok(decoded) => decoded,
            Err(denial) => {
                decode_denial = Some(denial);
                continue;
            }
        };
        let root = PhysicalRootManifest::new(
            decoded.root,
            decoded.segments,
            decoded.page_slots,
            decoded.extents,
            decoded.allocation_classes,
            decoded.free_space,
        );
        let manifest_counters = match ManifestDiscoveryAuthority::for_canonical_physical_format()
            .reopen_from_root(
                &root,
                PhysicalReferenceAuthority::for_canonical_physical_format()
                    .admit_root_publication(root.root_publication()),
            ) {
            Ok(report) => report.counters(),
            Err(denial) => {
                discovery_denial = Some(denial);
                continue;
            }
        };
        return Ok(CanonicalRootManifestAccess {
            counters: PhysicalLayoutAccessCounterSnapshot::range(
                manifest_bytes_len(witness) as u64,
                1,
                0,
                manifest_range_steps(manifest_counters),
            ),
            root,
            manifest_counters,
        });
    }

    if let Some(denial) = discovery_denial {
        return Err(PhysicalStoreRuntimeDenial::new(
            PhysicalStoreRuntimeDenialKind::ManifestDiscoveryDenied,
        )
        .with_manifest_denial(denial));
    }

    let _ = headers;
    Err(map_bootstrap_denial(
        PhysicalBootstrapCatalogDenial::ManifestDecodeDenied(Box::new(
            decode_denial
                .expect("bootstrap open witness always carries at least one root candidate"),
        )),
    ))
}

fn manifest_bytes_len(witness: &crate::PhysicalBootstrapCatalogOpenWitness) -> usize {
    witness
        .root_manifest_candidates()
        .iter()
        .map(Vec::len)
        .sum::<usize>()
        + witness.segment_manifest().len()
        + witness.extent_manifest().len()
        + witness.free_space_map().len()
}

fn manifest_range_steps(counters: ManifestDiscoveryCounterSnapshot) -> u16 {
    let total = counters.root_manifest_entry_count()
        + counters.segment_manifest_entry_count()
        + counters.extent_manifest_entry_count()
        + counters.allocation_class_entry_count()
        + counters.free_space_map_entry_count();
    total.try_into().unwrap_or(u16::MAX)
}

fn exact_manifest_counters(
    bytes_read: u64,
    page_touches: u16,
    counters: ManifestDiscoveryCounterSnapshot,
    extra_range_steps: u32,
) -> PhysicalLayoutAccessCounterSnapshot {
    let range_steps = u32::from(manifest_range_steps(counters)).saturating_add(extra_range_steps);
    let index_probes = counters
        .manifest_index_probe_count()
        .try_into()
        .unwrap_or(u16::MAX);
    PhysicalLayoutAccessCounterSnapshot::range(
        bytes_read,
        page_touches,
        index_probes,
        range_steps.try_into().unwrap_or(u16::MAX),
    )
}

fn map_bootstrap_denial(denial: PhysicalBootstrapCatalogDenial) -> PhysicalStoreRuntimeDenial {
    match denial {
        PhysicalBootstrapCatalogDenial::ManifestDecodeDenied(denial) => {
            let kind = match denial.kind() {
                crate::OfflineVerifierDenialKind::MissingRootManifest => {
                    PhysicalStoreRuntimeDenialKind::MissingPhysicalRoot
                }
                _ => PhysicalStoreRuntimeDenialKind::OfflineVerifierDenied,
            };
            PhysicalStoreRuntimeDenial::new(kind).with_verifier_denial(*denial)
        }
        PhysicalBootstrapCatalogDenial::ManifestDiscoveryDenied(denial) => {
            PhysicalStoreRuntimeDenial::new(PhysicalStoreRuntimeDenialKind::ManifestDiscoveryDenied)
                .with_manifest_denial(*denial)
        }
        PhysicalBootstrapCatalogDenial::BootstrapChecksumDenied(_) => {
            PhysicalStoreRuntimeDenial::new(PhysicalStoreRuntimeDenialKind::OfflineVerifierDenied)
        }
    }
}
