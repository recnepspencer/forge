#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryArtifactCategory {
    Summary,
    Report,
    Artifact,
    Receipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBoundaryCategoryDefinition {
    category: FoundationalBoundaryArtifactCategory,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl FoundationalBoundaryCategoryDefinition {
    pub const fn new(
        category: FoundationalBoundaryArtifactCategory,
        name: &'static str,
        intended_use: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            category,
            name,
            intended_use,
            must_not_mean,
        }
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

const SUMMARY_DEFINITION: FoundationalBoundaryCategoryDefinition =
    FoundationalBoundaryCategoryDefinition::new(
        FoundationalBoundaryArtifactCategory::Summary,
        "summary",
        "bounded overview-shaped boundary output",
        "an exhaustive report, a structured artifact payload, or a completed receipt",
    );
const REPORT_DEFINITION: FoundationalBoundaryCategoryDefinition =
    FoundationalBoundaryCategoryDefinition::new(
        FoundationalBoundaryArtifactCategory::Report,
        "report",
        "explanatory boundary output with evidence-bearing rows or sections",
        "a bounded summary, a generic payload envelope, or a completed receipt",
    );
const ARTIFACT_DEFINITION: FoundationalBoundaryCategoryDefinition =
    FoundationalBoundaryCategoryDefinition::new(
        FoundationalBoundaryArtifactCategory::Artifact,
        "artifact",
        "structured boundary payload with category-owned body shape",
        "an overview summary, an explanatory report, or an attested completion receipt",
    );
const RECEIPT_DEFINITION: FoundationalBoundaryCategoryDefinition =
    FoundationalBoundaryCategoryDefinition::new(
        FoundationalBoundaryArtifactCategory::Receipt,
        "receipt",
        "completed-boundary attestation surface",
        "a planned action, an explanatory report, or a generic payload artifact",
    );

pub const fn boundary_summary_definition() -> &'static FoundationalBoundaryCategoryDefinition {
    &SUMMARY_DEFINITION
}

pub const fn boundary_report_definition() -> &'static FoundationalBoundaryCategoryDefinition {
    &REPORT_DEFINITION
}

pub const fn boundary_artifact_surface_definition(
) -> &'static FoundationalBoundaryCategoryDefinition {
    &ARTIFACT_DEFINITION
}

pub const fn boundary_receipt_definition() -> &'static FoundationalBoundaryCategoryDefinition {
    &RECEIPT_DEFINITION
}

pub const fn boundary_artifact_category_definitions() -> [FoundationalBoundaryCategoryDefinition; 4]
{
    [
        SUMMARY_DEFINITION,
        REPORT_DEFINITION,
        ARTIFACT_DEFINITION,
        RECEIPT_DEFINITION,
    ]
}

pub trait FoundationalBoundaryCategoryMarker: sealed::Sealed {
    const CATEGORY: FoundationalBoundaryArtifactCategory;
    fn definition() -> &'static FoundationalBoundaryCategoryDefinition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SummaryCategory(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReportCategory(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactCategory(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptCategory(());

impl FoundationalBoundaryCategoryMarker for SummaryCategory {
    const CATEGORY: FoundationalBoundaryArtifactCategory =
        FoundationalBoundaryArtifactCategory::Summary;

    fn definition() -> &'static FoundationalBoundaryCategoryDefinition {
        boundary_summary_definition()
    }
}

impl FoundationalBoundaryCategoryMarker for ReportCategory {
    const CATEGORY: FoundationalBoundaryArtifactCategory =
        FoundationalBoundaryArtifactCategory::Report;

    fn definition() -> &'static FoundationalBoundaryCategoryDefinition {
        boundary_report_definition()
    }
}

impl FoundationalBoundaryCategoryMarker for ArtifactCategory {
    const CATEGORY: FoundationalBoundaryArtifactCategory =
        FoundationalBoundaryArtifactCategory::Artifact;

    fn definition() -> &'static FoundationalBoundaryCategoryDefinition {
        boundary_artifact_surface_definition()
    }
}

impl FoundationalBoundaryCategoryMarker for ReceiptCategory {
    const CATEGORY: FoundationalBoundaryArtifactCategory =
        FoundationalBoundaryArtifactCategory::Receipt;

    fn definition() -> &'static FoundationalBoundaryCategoryDefinition {
        boundary_receipt_definition()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryCategoryConstructionDenial {
    SummaryRequiresOverviewText,
    ReportRequiresAtLeastOneRow,
    ReceiptRequiresCompletedBoundaryDescription,
}

pub trait FoundationalBoundaryCategorySurface: sealed::Sealed {
    type Category: FoundationalBoundaryCategoryMarker;

    fn category(&self) -> FoundationalBoundaryArtifactCategory {
        <Self::Category as FoundationalBoundaryCategoryMarker>::CATEGORY
    }

    fn definition(&self) -> &'static FoundationalBoundaryCategoryDefinition {
        <Self::Category as FoundationalBoundaryCategoryMarker>::definition()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundarySummarySurface {
    overview: String,
    supporting_point_count: usize,
}

impl FoundationalBoundarySummarySurface {
    pub fn new(
        overview: impl Into<String>,
        supporting_point_count: usize,
    ) -> Result<Self, FoundationalBoundaryCategoryConstructionDenial> {
        let overview = overview.into();
        if overview.trim().is_empty() {
            return Err(
                FoundationalBoundaryCategoryConstructionDenial::SummaryRequiresOverviewText,
            );
        }

        Ok(Self {
            overview,
            supporting_point_count,
        })
    }

    pub fn overview(&self) -> &str {
        &self.overview
    }

    pub const fn supporting_point_count(&self) -> usize {
        self.supporting_point_count
    }
}

impl FoundationalBoundaryCategorySurface for FoundationalBoundarySummarySurface {
    type Category = SummaryCategory;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryReportSurface<Row> {
    rows: Vec<Row>,
    explanatory_section_count: usize,
}

impl<Row> FoundationalBoundaryReportSurface<Row> {
    pub fn new(
        rows: Vec<Row>,
        explanatory_section_count: usize,
    ) -> Result<Self, FoundationalBoundaryCategoryConstructionDenial> {
        if rows.is_empty() {
            return Err(
                FoundationalBoundaryCategoryConstructionDenial::ReportRequiresAtLeastOneRow,
            );
        }

        Ok(Self {
            rows,
            explanatory_section_count,
        })
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub const fn explanatory_section_count(&self) -> usize {
        self.explanatory_section_count
    }
}

impl<Row> FoundationalBoundaryCategorySurface for FoundationalBoundaryReportSurface<Row> {
    type Category = ReportCategory;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryArtifactSurface<T> {
    payload: T,
    attachment_slot_count: usize,
}

impl<T> FoundationalBoundaryArtifactSurface<T> {
    pub const fn new(payload: T, attachment_slot_count: usize) -> Self {
        Self {
            payload,
            attachment_slot_count,
        }
    }

    pub const fn payload(&self) -> &T {
        &self.payload
    }

    pub const fn attachment_slot_count(&self) -> usize {
        self.attachment_slot_count
    }
}

impl<T> FoundationalBoundaryCategorySurface for FoundationalBoundaryArtifactSurface<T> {
    type Category = ArtifactCategory;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryReceiptSurface {
    completed_boundary: String,
    attested_effect_count: usize,
}

impl FoundationalBoundaryReceiptSurface {
    pub fn new(
        completed_boundary: impl Into<String>,
        attested_effect_count: usize,
    ) -> Result<Self, FoundationalBoundaryCategoryConstructionDenial> {
        let completed_boundary = completed_boundary.into();
        if completed_boundary.trim().is_empty() {
            return Err(
                FoundationalBoundaryCategoryConstructionDenial::ReceiptRequiresCompletedBoundaryDescription,
            );
        }

        Ok(Self {
            completed_boundary,
            attested_effect_count,
        })
    }

    pub fn completed_boundary(&self) -> &str {
        &self.completed_boundary
    }

    pub const fn attested_effect_count(&self) -> usize {
        self.attested_effect_count
    }
}

impl FoundationalBoundaryCategorySurface for FoundationalBoundaryReceiptSurface {
    type Category = ReceiptCategory;
}

pub fn boundary_artifact_category_of<S>(surface: &S) -> FoundationalBoundaryArtifactCategory
where
    S: FoundationalBoundaryCategorySurface,
{
    surface.category()
}

pub fn boundary_summary_category_of<S>(surface: &S) -> FoundationalBoundaryArtifactCategory
where
    S: FoundationalBoundaryCategorySurface<Category = SummaryCategory>,
{
    surface.category()
}

pub fn boundary_report_category_of<S>(surface: &S) -> FoundationalBoundaryArtifactCategory
where
    S: FoundationalBoundaryCategorySurface<Category = ReportCategory>,
{
    surface.category()
}

pub fn boundary_receipt_category_of<S>(surface: &S) -> FoundationalBoundaryArtifactCategory
where
    S: FoundationalBoundaryCategorySurface<Category = ReceiptCategory>,
{
    surface.category()
}

mod sealed {
    use super::{
        ArtifactCategory, FoundationalBoundaryArtifactSurface, FoundationalBoundaryReceiptSurface,
        FoundationalBoundaryReportSurface, FoundationalBoundarySummarySurface, ReceiptCategory,
        ReportCategory, SummaryCategory,
    };

    pub trait Sealed {}

    impl Sealed for SummaryCategory {}
    impl Sealed for ReportCategory {}
    impl Sealed for ArtifactCategory {}
    impl Sealed for ReceiptCategory {}
    impl Sealed for FoundationalBoundarySummarySurface {}
    impl<Row> Sealed for FoundationalBoundaryReportSurface<Row> {}
    impl<T> Sealed for FoundationalBoundaryArtifactSurface<T> {}
    impl Sealed for FoundationalBoundaryReceiptSurface {}
}
