#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardColumn {
    pub id: String,
    pub name: String,
    pub order: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardUser {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub column_id: String,
    pub assignee_id: Option<String>,
    pub priority: String,
    pub due_date: Option<String>,
    pub tag: String,
}
