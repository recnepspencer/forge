use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "locators",
        "structural value, aspect, field, and boundary-artifact locator vocabulary",
        "stringly producer-private path interpretation",
    )
}
