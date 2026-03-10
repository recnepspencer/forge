use super::fixture::{build_fixture, FintechDomainFixture};
use super::scales::FintechScale;

pub(super) fn intraday_pricing_and_risk(scale: FintechScale) -> FintechDomainFixture {
    build_fixture(scale)
}
