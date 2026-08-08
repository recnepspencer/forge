//! Minimal application schema that installs aftermath through the production door.

mod declaration;
mod operations;

use worth_query_declaration::facade::application_schema::{
    ApplicationOperationRef, ApplicationSchema,
};
use worth_query_installation::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledAftermathContract,
    WorthQueryInstalledPackageIndex, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
};

use declaration::{
    AftermathFixtureSchema, ApproveEmergencyAccess, AuditRetention, Charge, FixtureInput,
    FreezeAccount, FreezeBalance, FreezeNote, LegalHold, NotifyDeath, ReleaseEstate, Transfer,
    TransferLarge, TransferSmall, WireTransferFinal,
};

fn op<Operation>(
    name: &'static str,
) -> ApplicationOperationRef<AftermathFixtureSchema, Operation, FixtureInput> {
    ApplicationOperationRef::from_schema_identifier(name)
}

fn installed_schema(
) -> worth_query_installation::facade::WorthQueryInstalledApplicationSchema<AftermathFixtureSchema>
{
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "aftermath-fixture",
        1,
        0,
    ))
    .application_schema(AftermathFixtureSchema::declaration().unwrap())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    let index = WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap();
    index
        .bind_application_schema(AftermathFixtureSchema::declaration().unwrap())
        .unwrap()
}

fn aftermath_of<Operation>(name: &'static str) -> WorthQueryInstalledAftermathContract {
    installed_schema()
        .installed_operation(op::<Operation>(name))
        .expect("operation installs")
        .contracts()
        .aftermath()
        .expect("aftermath declared")
        .clone()
}

pub(crate) fn release_estate() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<ReleaseEstate>("ReleaseEstate")
}
pub(crate) fn transfer() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<Transfer>("transfer")
}
pub(crate) fn transfer_small() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<TransferSmall>("transfer-small")
}
pub(crate) fn transfer_large() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<TransferLarge>("transfer-large")
}
pub(crate) fn freeze_account() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<FreezeAccount>("freeze-account")
}
pub(crate) fn notify_death() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<NotifyDeath>("notify-death")
}
pub(crate) fn legal_hold() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<LegalHold>("legal-hold")
}
pub(crate) fn audit_retention() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<AuditRetention>("audit-retention")
}
pub(crate) fn approve_emergency_access() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<ApproveEmergencyAccess>("ApproveEmergencyAccess")
}
pub(crate) fn wire_transfer_final() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<WireTransferFinal>("wire-transfer-final")
}
pub(crate) fn freeze_note() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<FreezeNote>("freeze-note")
}
pub(crate) fn freeze_balance() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<FreezeBalance>("freeze-balance")
}
pub(crate) fn charge() -> WorthQueryInstalledAftermathContract {
    aftermath_of::<Charge>("charge")
}
