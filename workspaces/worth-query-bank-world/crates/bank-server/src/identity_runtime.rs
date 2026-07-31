use std::collections::BTreeSet;

use bank_domain::estate::BankEstateWorld;
use bank_domain::model::BankPrincipalId;
use bank_domain::proposals::BankSnapshot;
use bank_domain::schema::{BankPrincipalBinding, BankSchema, ExternalPrincipalMapping, Principal};
use worth_query_host::facade::admission::authenticated_principal::{
    admit_authentication_adapter, WorthQueryAuthenticationAdapter,
    WorthQueryAuthenticationAdapterAdmission, WorthQueryAuthenticationAudience,
    WorthQueryAuthenticationMethod, WorthQueryRequestScope,
};
use worth_query_host::facade::domain::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstalledPrincipalBinding,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationInvariantProjectionAuthority, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode,
};
use worth_query_host::facade::runtime::{
    WorthQueryApplicationQueryResourceProfile, WorthQueryExecutionRuntimeInstaller,
};

use crate::domain_package::bank_domain_package;
use crate::error::{
    BankAuthenticationBoundaryBuildError, BankIdentityRuntimeBuildError,
    BankPrincipalAdmissionError, BankWorldSeedDenial,
};
use crate::graph_bootstrap::bind_bank_world_with_estate;
use crate::principal_seed::{BankPrincipalSeed, PreparedBankPrincipalSeed};
use crate::{
    BankApplicationQueryDenial, BankAuthenticatedPrincipal, BankAuthenticationBoundary,
    BankBusinessOwnerSeed, BankEmployeeAssignmentSeed, BankPreviewSession, BankWorldSeed,
};

pub struct BankAuthenticationConfiguration {
    audience: WorthQueryAuthenticationAudience,
    method: WorthQueryAuthenticationMethod,
}

impl BankAuthenticationConfiguration {
    pub const fn new(
        audience: WorthQueryAuthenticationAudience,
        method: WorthQueryAuthenticationMethod,
    ) -> Self {
        Self { audience, method }
    }
}

pub struct BankIdentityRuntime {
    runtime: WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    binding: WorthQueryInstalledPrincipalBinding<
        BankSchema,
        BankPrincipalBinding,
        ExternalPrincipalMapping,
        Principal,
        BankPrincipalId,
    >,
    invariant_projection: WorthQueryApplicationInvariantProjectionAuthority<BankSchema>,
}

impl BankIdentityRuntime {
    pub fn install(
        seeds: impl IntoIterator<Item = BankPrincipalSeed>,
    ) -> Result<Self, BankIdentityRuntimeBuildError> {
        let seeds = prepare_seeds(seeds)?;
        Self::install_prepared(seeds, None)
    }

    pub fn install_world(seed: BankWorldSeed) -> Result<Self, BankIdentityRuntimeBuildError> {
        let (snapshot, principals, owners, employees, estate) = seed.into_parts();
        let principals = prepare_seeds(principals)?;
        validate_world_principals(&snapshot, &principals)?;
        Self::install_prepared(
            principals,
            Some(BankGraphSeed {
                snapshot,
                owners,
                employees,
                estate,
            }),
        )
    }

    fn install_prepared(
        seeds: Vec<PreparedBankPrincipalSeed>,
        world: Option<BankGraphSeed>,
    ) -> Result<Self, BankIdentityRuntimeBuildError> {
        let package =
            bank_domain_package().map_err(BankIdentityRuntimeBuildError::SchemaDeclaration)?;
        let validated = package
            .validate()
            .map_err(BankIdentityRuntimeBuildError::PackageValidation)?;
        let admitted = WorthQueryInstallationAdmissionProfile::new(
            "worth-query-primary-graph-host-v1",
            "bank-primary-graph-v1",
        )
        .admit(validated)
        .map_err(BankIdentityRuntimeBuildError::PackageAdmission)?;
        let application_query_resources =
            WorthQueryApplicationQueryResourceProfile::bounded(32_768, 4_096, 32_768)
                .expect("bank application-query resource profile is statically non-zero");
        let installation = WorthQueryExecutionRuntimeInstaller::new()
            .application_query_resources(application_query_resources)
            .install(WorthQueryInstallationGeneration::initial(), [admitted])
            .map_err(BankIdentityRuntimeBuildError::RuntimeInstallation)?;
        let (runtime, authority) = installation.into_parts();
        let installed_schema = runtime
            .installed_packages()
            .bind_application_schema(
                BankSchema::declaration()
                    .map_err(BankIdentityRuntimeBuildError::SchemaDeclaration)?,
            )
            .map_err(BankIdentityRuntimeBuildError::InstalledSchema)?;
        let mut graph = authority
            .prepare_primary_graph(&runtime, &installed_schema)
            .map_err(BankIdentityRuntimeBuildError::PrimaryGraph)?;
        for seed in seeds {
            graph
                .bind_principal(
                    &installed_schema
                        .principal_binding(BankPrincipalBinding::reference())
                        .map_err(BankIdentityRuntimeBuildError::InstalledBinding)?,
                    seed.key,
                    seed.principal_id,
                    seed.external_identity,
                    seed.status,
                )
                .map_err(BankIdentityRuntimeBuildError::PrimaryGraph)?;
        }
        if let Some(world) = &world {
            bind_bank_world_with_estate(
                &mut graph,
                &world.snapshot,
                &world.owners,
                &world.employees,
                world.estate.as_ref(),
            )
            .map_err(BankIdentityRuntimeBuildError::PrimaryGraph)?;
        }
        let invariant_projection = graph.retain_invariant_projection_authority();
        let runtime = graph
            .publish_application_runtime(runtime, authority, installed_schema)
            .map_err(BankIdentityRuntimeBuildError::PrimaryGraph)?;
        let binding = runtime
            .installed_schema()
            .principal_binding(BankPrincipalBinding::reference())
            .map_err(BankIdentityRuntimeBuildError::InstalledBinding)?;

        Ok(Self {
            runtime,
            binding,
            invariant_projection,
        })
    }

    pub fn admit_authentication_adapter<Adapter>(
        &self,
        configuration: BankAuthenticationConfiguration,
        adapter: Adapter,
    ) -> Result<BankAuthenticationBoundary<Adapter>, BankAuthenticationBoundaryBuildError>
    where
        Adapter: WorthQueryAuthenticationAdapter,
    {
        admit_authentication_adapter(
            self.runtime.installed_schema(),
            WorthQueryAuthenticationAdapterAdmission::new(
                configuration.audience,
                configuration.method,
            ),
            adapter,
        )
        .map(BankAuthenticationBoundary::new)
        .map_err(BankAuthenticationBoundaryBuildError)
    }

    pub async fn authenticate_with<Adapter>(
        &self,
        authentication: &BankAuthenticationBoundary<Adapter>,
        credential: Adapter::Credential,
        scope: &WorthQueryRequestScope,
    ) -> Result<BankAuthenticatedPrincipal, BankPrincipalAdmissionError>
    where
        Adapter: WorthQueryAuthenticationAdapter,
    {
        let external = authentication
            .authenticate(credential, scope)
            .await
            .map_err(BankPrincipalAdmissionError::Authentication)?;
        let query = self
            .runtime
            .resolve_authenticated_principal(
                &self.binding,
                external,
                scope,
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankPrincipalAdmissionError::Resolution)?;
        let principal_id = *query.principal_identity();
        Ok(BankAuthenticatedPrincipal::new(principal_id, query))
    }

    pub fn validate(
        &self,
        principal: &BankAuthenticatedPrincipal,
        scope: &WorthQueryRequestScope,
    ) -> Result<(), BankPrincipalAdmissionError> {
        self.runtime
            .validate_authenticated_principal(principal.query(), scope)
            .map_err(BankPrincipalAdmissionError::Resolution)
    }

    pub fn mapped_principal_count(&self) -> usize {
        self.runtime.publication().principal_binding_count()
    }

    pub fn open_preview(
        &self,
        request: &WorthQueryRequestScope,
    ) -> Result<BankPreviewSession, BankApplicationQueryDenial> {
        self.runtime
            .open_application_preview_session(request)
            .map_err(BankApplicationQueryDenial::PreviewSession)
    }

    pub(crate) const fn application_runtime(
        &self,
    ) -> &WorthQueryPrimaryGraphApplicationRuntime<BankSchema> {
        &self.runtime
    }

    pub(crate) const fn invariant_projection(
        &self,
    ) -> &WorthQueryApplicationInvariantProjectionAuthority<BankSchema> {
        &self.invariant_projection
    }
}

struct BankGraphSeed {
    snapshot: BankSnapshot,
    owners: Vec<BankBusinessOwnerSeed>,
    employees: Vec<BankEmployeeAssignmentSeed>,
    estate: Option<BankEstateWorld>,
}

fn prepare_seeds(
    seeds: impl IntoIterator<Item = BankPrincipalSeed>,
) -> Result<Vec<PreparedBankPrincipalSeed>, BankIdentityRuntimeBuildError> {
    seeds
        .into_iter()
        .map(|seed| {
            seed.prepare()
                .map_err(BankIdentityRuntimeBuildError::PrincipalKey)
        })
        .collect()
}

fn validate_world_principals(
    snapshot: &BankSnapshot,
    principals: &[PreparedBankPrincipalSeed],
) -> Result<(), BankIdentityRuntimeBuildError> {
    let expected = snapshot.principals().collect::<BTreeSet<_>>();
    let supplied = principals
        .iter()
        .map(|principal| principal.principal_id)
        .collect::<BTreeSet<_>>();
    if expected == supplied && supplied.len() == principals.len() {
        Ok(())
    } else {
        Err(BankIdentityRuntimeBuildError::WorldSeed(
            BankWorldSeedDenial::PrincipalSetMismatch,
        ))
    }
}
