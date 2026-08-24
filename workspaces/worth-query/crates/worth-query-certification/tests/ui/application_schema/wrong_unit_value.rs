use std::marker::PhantomData;

use worth_foundational::facade::{AspectValue, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, ApplicationUnitMarker,
    DeclaredApplicationUnit, EqualityPredicate, ReadOnly, TypedApplicationValue,
    TypedUnitApplicationValue,
};

struct Schema;
struct Entity;
struct Aspect;
struct Balance;
struct UsdCurrency;
struct USD;
struct EUR;
struct Money<C>(PhantomData<C>);

impl ApplicationEntityMarkerIdentity for Entity {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "Entity";
}
impl ApplicationAspectMarkerIdentity for Aspect {
    type Schema = Schema;
    type Entity = Entity;
    const IDENTIFIER: &'static str = "Aspect";
    const ASPECT_IDENTITY: worth_query_decl::facade::application_schema::AspectIdentity =
        worth_query_decl::facade::application_schema::AspectIdentity(0x91612007);
    const CONTRACT_REVISION: worth_query_decl::facade::application_schema::AspectContractRevision =
        worth_query_decl::facade::application_schema::AspectContractRevision(1);
}
impl ApplicationFieldMarkerIdentity for Balance {
    type Schema = Schema;
    type Entity = Entity;
    type Aspect = Aspect;
    const IDENTIFIER: &'static str = "Balance";
}

impl ApplicationUnitMarker<USD> for UsdCurrency {
    const NAME: &'static str = "UsdCurrency";
}

impl<C> TypedApplicationValue for Money<C> {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(0)
    }
}

impl<C: 'static> TypedUnitApplicationValue for Money<C> {
    type Unit = C;
}

fn main() {
    let _: ApplicationFieldRef<
        Schema,
        Entity,
        Aspect,
        Balance,
        Money<EUR>,
        ReadOnly,
        EqualityPredicate,
        DeclaredApplicationUnit<UsdCurrency, <Money<EUR> as TypedUnitApplicationValue>::Unit>,
    > = ApplicationFieldRef::from_schema_types();
}
