use worth_store_physical_format::{
    ManifestDiscoveryCounterSnapshot, PhysicalBinaryEncodingWitness,
    PhysicalForegroundBoundednessReport, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalHeaderAuthority, PhysicalOperationCounterSnapshot, PhysicalOperationKind,
    PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalHostileScaleFixtureReport {
    operation: PhysicalOperationKind,
    condition: PhysicalHostileScaleCondition,
    source: PhysicalHostileScaleFixtureSource,
    unrelated_segments: u32,
    unrelated_pages: u32,
    unrelated_extents: u32,
    unrelated_manifests: u32,
    baseline: PhysicalOperationCounterSnapshot,
    grown: PhysicalOperationCounterSnapshot,
    free_space: Option<PhysicalForegroundBoundednessReport>,
}

impl PhysicalHostileScaleFixtureReport {
    pub fn locate_reference_unrelated_growth() -> Result<Self, PhysicalHostileScaleFixtureDenial> {
        let baseline = target_locate_counters()?;
        let unrelated = build_unrelated_physical_growth(9, 64, 12, 7)?;
        let grown = target_locate_counters()?;
        Ok(Self {
            operation: PhysicalOperationKind::LocateByReference,
            condition: PhysicalHostileScaleCondition::LocateUnrelatedGrowth,
            source: PhysicalHostileScaleFixtureSource::AuthorityExecution,
            unrelated_segments: unrelated.segments,
            unrelated_pages: unrelated.pages,
            unrelated_extents: unrelated.extents,
            unrelated_manifests: unrelated.manifests,
            baseline,
            grown,
            free_space: None,
        })
    }

    pub fn header_decode_fixed_fields() -> Self {
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let page_cell = generations
            .page_cell(segment(41), page(51))
            .with_page_generation(generation(7));
        let header_authority = PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical()
                .expect("static S.1 fixture encoding witness is valid"),
        );
        let bytes = crate::physical_fixture_encoding::data_page_bytes(page_cell, &[]);
        let counters = PhysicalOperationCounterSnapshot::from_header_decode(
            header_authority
                .decode_page_header(page_cell, &bytes, PhysicalPageKind::DataPage)
                .expect("fixture page header bytes should decode")
                .counters(),
        );
        Self::declared_growth(
            PhysicalOperationKind::HeaderDecode,
            PhysicalHostileScaleCondition::FixedHeaderDecode,
            PhysicalHostileScaleFixtureSource::AuthorityExecution,
            counters,
        )
    }

    pub fn reference_validation_fixed_fields() -> Self {
        let counters = PhysicalOperationCounterSnapshot::from_reference_validation(
            reference_validation_counters(),
        );
        Self::declared_growth(
            PhysicalOperationKind::PhysicalReferenceValidation,
            PhysicalHostileScaleCondition::ReferenceValidationFixedFields,
            PhysicalHostileScaleFixtureSource::AuthorityExecution,
            counters,
        )
    }

    pub fn manifest_index_lookup_growth(counters: PhysicalOperationCounterSnapshot) -> Self {
        Self::declared_growth(
            PhysicalOperationKind::ManifestLookup,
            PhysicalHostileScaleCondition::ManifestIndexUnrelatedGrowth,
            PhysicalHostileScaleFixtureSource::DeclaredCounterReceipt,
            counters,
        )
    }

    pub fn manifest_index_lookup_counter_drift(
        baseline: PhysicalOperationCounterSnapshot,
        grown: PhysicalOperationCounterSnapshot,
    ) -> Self {
        Self::declared_growth_pair(
            PhysicalOperationKind::ManifestLookup,
            PhysicalHostileScaleCondition::ManifestIndexUnrelatedGrowth,
            PhysicalHostileScaleFixtureSource::DeclaredCounterReceipt,
            baseline,
            grown,
        )
    }

    pub fn root_open_root_entries(counters: PhysicalOperationCounterSnapshot) -> Self {
        Self::declared_growth(
            PhysicalOperationKind::RootManifestOpen,
            PhysicalHostileScaleCondition::RootOpenRootEntries,
            PhysicalHostileScaleFixtureSource::DeclaredCounterReceipt,
            counters,
        )
    }

    pub fn manifest_traversal_declared_growth(counters: PhysicalOperationCounterSnapshot) -> Self {
        Self::declared_growth(
            PhysicalOperationKind::ManifestTraversal,
            PhysicalHostileScaleCondition::DeclaredManifestTraversal,
            PhysicalHostileScaleFixtureSource::DeclaredCounterReceipt,
            counters,
        )
    }

    pub fn offline_verifier_declared_walk(counters: PhysicalOperationCounterSnapshot) -> Self {
        Self::declared_growth(
            PhysicalOperationKind::OfflineVerifierWalk,
            PhysicalHostileScaleCondition::OfflineVerifierDeclaredWalk,
            PhysicalHostileScaleFixtureSource::DeclaredCounterReceipt,
            counters,
        )
    }

    pub fn fragmented_free_space_for_append(
        report: PhysicalForegroundBoundednessReport,
        counters: PhysicalOperationCounterSnapshot,
    ) -> Self {
        Self {
            operation: PhysicalOperationKind::AppendRecordPlacement,
            condition: PhysicalHostileScaleCondition::FragmentedFreeSpacePressure,
            source: PhysicalHostileScaleFixtureSource::PolicyEvaluation,
            unrelated_segments: 0,
            unrelated_pages: 0,
            unrelated_extents: 0,
            unrelated_manifests: report.pressure().fragmented_candidates(),
            baseline: counters.clone(),
            grown: counters,
            free_space: Some(report),
        }
    }

    pub const fn operation(&self) -> PhysicalOperationKind {
        self.operation
    }

    pub const fn condition(&self) -> PhysicalHostileScaleCondition {
        self.condition
    }

    pub const fn source(&self) -> PhysicalHostileScaleFixtureSource {
        self.source
    }

    pub fn baseline_counters(&self) -> &PhysicalOperationCounterSnapshot {
        &self.baseline
    }

    pub fn grown_counters(&self) -> &PhysicalOperationCounterSnapshot {
        &self.grown
    }

    pub const fn free_space_report(&self) -> Option<PhysicalForegroundBoundednessReport> {
        self.free_space
    }

    pub const fn proves_unrelated_growth(&self) -> bool {
        self.unrelated_segments > 0
            || self.unrelated_pages > 0
            || self.unrelated_extents > 0
            || self.unrelated_manifests > 0
    }

    fn declared_growth(
        operation: PhysicalOperationKind,
        condition: PhysicalHostileScaleCondition,
        source: PhysicalHostileScaleFixtureSource,
        counters: PhysicalOperationCounterSnapshot,
    ) -> Self {
        Self::declared_growth_pair(operation, condition, source, counters.clone(), counters)
    }

    fn declared_growth_pair(
        operation: PhysicalOperationKind,
        condition: PhysicalHostileScaleCondition,
        source: PhysicalHostileScaleFixtureSource,
        baseline: PhysicalOperationCounterSnapshot,
        grown: PhysicalOperationCounterSnapshot,
    ) -> Self {
        Self {
            operation,
            condition,
            source,
            unrelated_segments: 3,
            unrelated_pages: 5,
            unrelated_extents: 7,
            unrelated_manifests: 2,
            baseline,
            grown,
            free_space: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHostileScaleCondition {
    FixedHeaderDecode,
    ReferenceValidationFixedFields,
    LocateUnrelatedGrowth,
    ManifestIndexUnrelatedGrowth,
    RootOpenRootEntries,
    FragmentedFreeSpacePressure,
    DeclaredManifestTraversal,
    OfflineVerifierDeclaredWalk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHostileScaleFixtureSource {
    AuthorityExecution,
    PolicyEvaluation,
    DeclaredCounterReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHostileScaleFixtureDenial {
    HeaderDecode,
    PayloadAdmission,
    Append,
    ReferenceValidation,
    Locate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnrelatedGrowth {
    segments: u32,
    pages: u32,
    extents: u32,
    manifests: u32,
}

fn target_locate_counters(
) -> Result<PhysicalOperationCounterSnapshot, PhysicalHostileScaleFixtureDenial> {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let empty_page = crate::physical_fixture_encoding::data_page_bytes(page_cell, &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page)?,
            SlotAppendRequest::ordinary(slot_cell, b"target"),
        )
        .map_err(|_| PhysicalHostileScaleFixtureDenial::Append)?;
    let reopened_page =
        crate::physical_fixture_encoding::data_page_bytes(page_cell, append.page_payload());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .map_err(|_| PhysicalHostileScaleFixtureDenial::ReferenceValidation)?;
    let located = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page)?,
            validation,
        )
        .map_err(|_| PhysicalHostileScaleFixtureDenial::Locate)?;
    Ok(PhysicalOperationCounterSnapshot::from_page_record_locate(
        located.counters(),
    ))
}

fn build_unrelated_physical_growth(
    segments: u32,
    pages: u32,
    extents: u32,
    manifests: u32,
) -> Result<UnrelatedGrowth, PhysicalHostileScaleFixtureDenial> {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    for index in 0..pages {
        let page_cell = generations
            .page_cell(segment(100 + index as u64), page(200 + index as u64))
            .with_page_generation(generation(3 + index as u64));
        let slot_cell = generations
            .slot_cell(
                segment(100 + index as u64),
                page(200 + index as u64),
                slot(1),
            )
            .with_slot_generation(generation(30 + index as u64));
        let empty_page = crate::physical_fixture_encoding::data_page_bytes(page_cell, &[]);
        records
            .append_record(
                admitted_page(&records, page_cell, &empty_page)?,
                SlotAppendRequest::ordinary(slot_cell, b"unrelated"),
            )
            .map_err(|_| PhysicalHostileScaleFixtureDenial::Append)?;
    }
    let _manifest_pressure = ManifestDiscoveryCounterSnapshot::for_reopen()
        .with_segment_manifest(segments)
        .with_extent_manifest(extents)
        .with_root_entries(manifests);
    Ok(UnrelatedGrowth {
        segments,
        pages,
        extents,
        manifests,
    })
}

fn reference_validation_counters(
) -> worth_store_physical_format::PhysicalReferenceValidationCounterSnapshot {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let slot_cell = generations
        .slot_cell(segment(21), page(31), slot(1))
        .with_slot_generation(generation(2));
    references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .expect("fixture reference validation should admit matching slot")
        .counters()
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: worth_store_physical_format::PageGenerationCell,
    bytes: &'a [u8],
) -> Result<worth_store_physical_format::RecordPagePayload<'a>, PhysicalHostileScaleFixtureDenial> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .map_err(|_| PhysicalHostileScaleFixtureDenial::HeaderDecode)?;
    records
        .admit_record_page_payload(bytes, header.witness())
        .map_err(|_| PhysicalHostileScaleFixtureDenial::PayloadAdmission)
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical()
                .expect("static S.1 fixture encoding witness is valid"),
        ),
    )
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("fixture segment id is nonzero")
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("fixture page id is nonzero")
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("fixture slot id is nonzero")
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("fixture generation is nonzero")
}
