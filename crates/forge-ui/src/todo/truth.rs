use forge_query::facade::{
    ForgeQueryAspect, ForgeQueryAspectValue, ForgeQueryEntity, ForgeQueryMemoryWorkspace,
    ForgeQueryMutationReceipt, ForgeQueryWorkspaceError,
};
use serde_json::{json, Value};

use super::task::{BoardColumn, BoardUser, Task};

const TASKS: &str = "Task";
const COLUMNS: &str = "Column";
const USERS: &str = "User";
pub struct TodoWorkspace {
    tasks: ForgeQueryMemoryWorkspace,
    columns: ForgeQueryMemoryWorkspace,
    users: ForgeQueryMemoryWorkspace,
}

#[derive(Clone, Debug)]
pub enum TodoCommand {
    CreateTask(Task),
    AddColumn(String),
    AddUser(String),
    UpdateTaskField {
        task_id: String,
        aspect: String,
        field: String,
        value: Value,
    },
    DeleteTask(String),
}

#[derive(Clone, Debug)]
pub struct TodoView {
    pub tasks: Vec<Task>,
    pub columns: Vec<BoardColumn>,
    pub users: Vec<BoardUser>,
    pub snapshot_token: String,
}

impl TodoWorkspace {
    pub fn seeded() -> Self {
        let mut workspace = Self::new().expect("todo workspace should build");
        for name in ["Todo", "Doing", "Done"] {
            workspace
                .submit(TodoCommand::AddColumn(name.to_string()))
                .expect("default column should insert");
        }
        workspace
    }

    fn new() -> Result<Self, ForgeQueryWorkspaceError> {
        let tasks = ForgeQueryMemoryWorkspace::collection(
            TASKS,
            [
                ForgeQueryAspect::new("identity.id", "identity.id"),
                ForgeQueryAspect::new("title.value", "title.value"),
                ForgeQueryAspect::new("column.id", "column.id"),
                ForgeQueryAspect::new("assignee.id", "assignee.id"),
                ForgeQueryAspect::new("priority.level", "priority.level"),
                ForgeQueryAspect::new("due.date", "due.date"),
                ForgeQueryAspect::new("tag.name", "tag.name"),
            ],
        )?;
        let columns = ForgeQueryMemoryWorkspace::collection(
            COLUMNS,
            [
                ForgeQueryAspect::new("identity.id", "identity.id"),
                ForgeQueryAspect::new("name.value", "name.value"),
                ForgeQueryAspect::new("order.index", "order.index"),
            ],
        )?;
        let users = ForgeQueryMemoryWorkspace::collection(
            USERS,
            [
                ForgeQueryAspect::new("identity.id", "identity.id"),
                ForgeQueryAspect::new("name.value", "name.value"),
            ],
        )?;
        Ok(Self {
            tasks,
            columns,
            users,
        })
    }

    pub fn submit(
        &mut self,
        command: TodoCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        match command {
            TodoCommand::CreateTask(task) => self.tasks.insert_aspects(task_aspects(&task)),
            TodoCommand::AddColumn(name) => {
                let order = self.live_columns().len() + 1;
                self.columns.insert_aspects(vec![
                    aspect_value("identity.id", ""),
                    aspect_value("name.value", name),
                    ForgeQueryAspectValue::new("order.index", json!(order))
                        .expect("order aspect should serialize"),
                ])
            }
            TodoCommand::AddUser(name) => self.users.insert_aspects(vec![
                aspect_value("identity.id", ""),
                aspect_value("name.value", name),
            ]),
            TodoCommand::UpdateTaskField {
                task_id,
                aspect,
                field,
                value,
            } => {
                let entity_identity = self
                    .task_entity_identity(&task_id)
                    .ok_or_else(|| ForgeQueryWorkspaceError::new("task not found"))?;
                self.tasks
                    .update_aspect(&entity_identity, &format!("{aspect}.{field}"), value)
            }
            TodoCommand::DeleteTask(task_id) => {
                let entity_identity = self
                    .task_entity_identity(&task_id)
                    .ok_or_else(|| ForgeQueryWorkspaceError::new("task not found"))?;
                self.tasks.delete(&entity_identity)
            }
        }
    }

    pub fn view(&self) -> TodoView {
        TodoView {
            tasks: self.live_tasks(),
            columns: self.live_columns(),
            users: self.live_users(),
            snapshot_token: self.snapshot_token(),
        }
    }

    pub fn snapshot_token(&self) -> String {
        format!(
            "{}|{}|{}",
            self.tasks.snapshot_token(),
            self.columns.snapshot_token(),
            self.users.snapshot_token()
        )
    }

    pub fn live_tasks(&self) -> Vec<Task> {
        let mut tasks = self
            .tasks
            .entities()
            .into_iter()
            .filter_map(task_from_entity)
            .collect::<Vec<_>>();
        tasks.sort_by(|left, right| left.title.cmp(&right.title));
        tasks
    }

    pub fn live_columns(&self) -> Vec<BoardColumn> {
        let mut columns = self
            .columns
            .entities()
            .into_iter()
            .filter_map(column_from_entity)
            .collect::<Vec<_>>();
        columns.sort_by_key(|column| column.order);
        columns
    }

    pub fn live_users(&self) -> Vec<BoardUser> {
        let mut users = self
            .users
            .entities()
            .into_iter()
            .filter_map(user_from_entity)
            .collect::<Vec<_>>();
        users.sort_by(|left, right| left.name.cmp(&right.name));
        users
    }

    pub fn live_task(&self, id: &str) -> Option<Task> {
        self.live_tasks().into_iter().find(|task| task.id == id)
    }

    pub fn drain_board_patches(&mut self) {}

    fn task_entity_identity(&self, task_id: &str) -> Option<String> {
        self.tasks
            .entities()
            .into_iter()
            .find(|entity| {
                string_path(&entity.payload, &["identity", "id"]).as_deref() == Some(task_id)
            })
            .map(|entity| entity.identity)
    }
}

fn task_from_entity(entity: ForgeQueryEntity) -> Option<Task> {
    let payload = entity.payload;
    Some(Task {
        id: string_path(&payload, &["identity", "id"]).unwrap_or(entity.identity),
        title: string_path(&payload, &["title", "value"])?,
        column_id: string_path(&payload, &["column", "id"])?,
        assignee_id: non_empty_string_path(&payload, &["assignee", "id"]),
        priority: string_path(&payload, &["priority", "level"]).unwrap_or_else(|| "P2".to_string()),
        due_date: non_empty_string_path(&payload, &["due", "date"]),
        tag: string_path(&payload, &["tag", "name"]).unwrap_or_default(),
    })
}

fn column_from_entity(entity: ForgeQueryEntity) -> Option<BoardColumn> {
    let payload = entity.payload;
    Some(BoardColumn {
        id: string_path(&payload, &["identity", "id"]).unwrap_or(entity.identity),
        name: string_path(&payload, &["name", "value"])?,
        order: payload
            .pointer("/order/index")
            .and_then(Value::as_u64)
            .unwrap_or(usize::MAX as u64) as usize,
    })
}

fn user_from_entity(entity: ForgeQueryEntity) -> Option<BoardUser> {
    let payload = entity.payload;
    Some(BoardUser {
        id: string_path(&payload, &["identity", "id"]).unwrap_or(entity.identity),
        name: string_path(&payload, &["name", "value"])?,
    })
}

fn task_aspects(task: &Task) -> Vec<ForgeQueryAspectValue> {
    vec![
        aspect_value("identity.id", task.id.clone()),
        aspect_value("title.value", task.title.clone()),
        aspect_value("column.id", task.column_id.clone()),
        aspect_value("assignee.id", task.assignee_id.clone().unwrap_or_default()),
        aspect_value("priority.level", task.priority.clone()),
        aspect_value("due.date", task.due_date.clone().unwrap_or_default()),
        aspect_value("tag.name", task.tag.clone()),
    ]
}

fn aspect_value(aspect_path: &str, value: impl Into<String>) -> ForgeQueryAspectValue {
    ForgeQueryAspectValue::new(aspect_path, json!(value.into()))
        .expect("todo aspect value should serialize")
}

fn string_path(payload: &Value, path: &[&str]) -> Option<String> {
    let mut current = payload;
    for segment in path {
        current = current.get(*segment)?;
    }
    current.as_str().map(ToString::to_string)
}

fn non_empty_string_path(payload: &Value, path: &[&str]) -> Option<String> {
    string_path(payload, path).filter(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_workspace_uses_explicit_memory_workspace_scaffold() {
        let mut workspace = TodoWorkspace::seeded();

        assert_eq!(workspace.live_columns().len(), 3);
        assert!(workspace.live_tasks().is_empty());

        workspace
            .submit(TodoCommand::CreateTask(Task {
                id: "task-1".to_string(),
                title: "Batch 9 audit".to_string(),
                column_id: workspace.live_columns()[0].id.clone(),
                assignee_id: None,
                priority: "P1".to_string(),
                due_date: None,
                tag: "runtime-api".to_string(),
            }))
            .expect("scaffold write should mutate explicit memory workspaces");

        let tasks = workspace.live_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Batch 9 audit");
    }
}
