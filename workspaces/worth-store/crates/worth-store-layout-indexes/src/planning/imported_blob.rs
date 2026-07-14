#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportedBlobReadAdmissionCaseId(&'static str);

impl ImportedBlobReadAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

const ADMITTED: ImportedBlobReadAdmissionCaseId =
    ImportedBlobReadAdmissionCaseId("blob.layout.read_admission.admitted");
const MATERIALIZATION_DENIED: ImportedBlobReadAdmissionCaseId =
    ImportedBlobReadAdmissionCaseId("blob.layout.read_admission.denied.materialization");
const CONCRETE_KEY_DENIED: ImportedBlobReadAdmissionCaseId =
    ImportedBlobReadAdmissionCaseId("blob.layout.read_admission.denied.concrete_key");
const REQUEST_DENIED: ImportedBlobReadAdmissionCaseId =
    ImportedBlobReadAdmissionCaseId("blob.layout.read_admission.denied.request");

pub fn imported_blob_read_admission_cases() -> impl Iterator<Item = ImportedBlobReadAdmissionCaseId>
{
    [
        ADMITTED,
        MATERIALIZATION_DENIED,
        CONCRETE_KEY_DENIED,
        REQUEST_DENIED,
    ]
    .into_iter()
}

pub(super) fn admit_imported_blob_read_request(
    family: crate::AdmittedPhysicalArtifactFamily,
    key_domain: crate::AdmittedPhysicalKeyDomain,
    catalog: &crate::BootstrapCatalogReadAdmission,
    witness: &worth_store_blob_chunks::ImportedBlobWitness,
) -> ImportedBlobReadAdmissionOutcome {
    let materialization = match crate::access_planning()
        .admit_imported_blob_materialization(family, catalog, witness)
        .into_result()
    {
        Ok(materialization) => materialization,
        Err(denial) => return ImportedBlobReadAdmissionOutcome::materialization_denied(denial),
    };
    let concrete_key = match crate::keyspace::admit_blob_key(
        key_domain,
        crate::BlobIdentityKeyBasis::new(
            witness.object_id().digest().clone(),
            crate::BlobGenerationBasis::from_sequence(witness.generation().sequence()),
        ),
    ) {
        Ok(key) => key,
        Err(denial) => return ImportedBlobReadAdmissionOutcome::concrete_key_denied(denial),
    };
    match super::AccessPlanSelector.admit_read_request(
        family,
        concrete_key,
        materialization,
        crate::access::shape::access_shapes().point_lookup_declaration(),
    ) {
        Ok(request) => ImportedBlobReadAdmissionOutcome::admitted(request),
        Err(denial) => ImportedBlobReadAdmissionOutcome::request_denied(denial),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ImportedBlobReadAdmissionCase {
    Admitted(Box<super::AdmittedPhysicalReadRequest>),
    MaterializationDenied(crate::MaterializationDenial),
    ConcreteKeyDenied(crate::ArtifactFamilyDenial),
    RequestDenied(super::PhysicalAccessRequestAdmissionDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportedBlobReadAdmissionOutcome {
    case: ImportedBlobReadAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportedBlobReadAdmissionView<'a> {
    Admitted(&'a super::AdmittedPhysicalReadRequest),
    MaterializationDenied(&'a crate::MaterializationDenial),
    ConcreteKeyDenied(&'a crate::ArtifactFamilyDenial),
    RequestDenied(&'a super::PhysicalAccessRequestAdmissionDenied),
}

impl ImportedBlobReadAdmissionOutcome {
    pub(super) fn admitted(request: super::AdmittedPhysicalReadRequest) -> Self {
        Self {
            case: ImportedBlobReadAdmissionCase::Admitted(Box::new(request)),
        }
    }

    pub(super) fn materialization_denied(denial: crate::MaterializationDenial) -> Self {
        Self {
            case: ImportedBlobReadAdmissionCase::MaterializationDenied(denial),
        }
    }

    pub(super) fn concrete_key_denied(denial: crate::ArtifactFamilyDenial) -> Self {
        Self {
            case: ImportedBlobReadAdmissionCase::ConcreteKeyDenied(denial),
        }
    }

    pub(super) fn request_denied(denial: super::PhysicalAccessRequestAdmissionDenied) -> Self {
        Self {
            case: ImportedBlobReadAdmissionCase::RequestDenied(denial),
        }
    }

    pub fn view(&self) -> ImportedBlobReadAdmissionView<'_> {
        match &self.case {
            ImportedBlobReadAdmissionCase::Admitted(value) => {
                ImportedBlobReadAdmissionView::Admitted(value)
            }
            ImportedBlobReadAdmissionCase::MaterializationDenied(value) => {
                ImportedBlobReadAdmissionView::MaterializationDenied(value)
            }
            ImportedBlobReadAdmissionCase::ConcreteKeyDenied(value) => {
                ImportedBlobReadAdmissionView::ConcreteKeyDenied(value)
            }
            ImportedBlobReadAdmissionCase::RequestDenied(value) => {
                ImportedBlobReadAdmissionView::RequestDenied(value)
            }
        }
    }

    pub fn case_id(&self) -> ImportedBlobReadAdmissionCaseId {
        match self.case {
            ImportedBlobReadAdmissionCase::Admitted(_) => ADMITTED,
            ImportedBlobReadAdmissionCase::MaterializationDenied(_) => MATERIALIZATION_DENIED,
            ImportedBlobReadAdmissionCase::ConcreteKeyDenied(_) => CONCRETE_KEY_DENIED,
            ImportedBlobReadAdmissionCase::RequestDenied(_) => REQUEST_DENIED,
        }
    }

    pub fn into_admitted(self) -> Result<super::AdmittedPhysicalReadRequest, Self> {
        match self.case {
            ImportedBlobReadAdmissionCase::Admitted(value) => Ok(*value),
            case => Err(Self { case }),
        }
    }
}
