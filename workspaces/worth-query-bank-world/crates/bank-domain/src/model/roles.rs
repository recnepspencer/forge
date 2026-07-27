#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CustomerRole {
    PersonalOwner,
    BusinessOwner,
    Initiator,
    Approver,
    Viewer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmployeeRole {
    Teller,
    Auditor,
}
