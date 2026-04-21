use forge_query::facade::DeclarativeWritebackValue;
use forge_ui_components::{dropdown::DropdownState, IconStore};
use forge_ui_theme::{dark_theme, ForgeTheme};
use serde_json::Value;

use crate::todo::task::{BoardColumn, BoardUser, Task};
use crate::todo::{TodoCommand, TodoWorkspace};
pub(crate) struct TodoShowcaseApp {
    pub(crate) truth: TodoWorkspace,
    pub(crate) selected_task: String,
    pub(crate) new_task_draft: Task,
    pub(crate) dragged_task: Option<String>,
    pub(crate) current_filter: String,
    pub(crate) show_add_task_modal: bool,
    pub(crate) show_edit_task_modal: bool,
    pub(crate) show_add_column_modal: bool,
    pub(crate) show_add_user_modal: bool,
    pub(crate) new_user_name: String,
    pub(crate) new_column_name: String,
    pub(crate) theme: ForgeTheme,
    pub(crate) icons: IconStore,

    // Dropdown states
    pub(crate) assignee_dropdown: DropdownState,
    pub(crate) priority_dropdown: DropdownState,
    pub(crate) column_dropdown: DropdownState,
}

impl TodoShowcaseApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = dark_theme();
        theme.apply_to_egui(&cc.egui_ctx);
        let icons = IconStore::load(&cc.egui_ctx);
        let truth = TodoWorkspace::seeded();
        let initial_view = truth.view();
        let selected_task = initial_view
            .tasks
            .first()
            .map(|task| task.id.clone())
            .unwrap_or_default();
        let columns = initial_view.columns.clone();
        let _initial_runtime_summary = (
            initial_view.users.len(),
            initial_view.snapshot_token.clone(),
        );
        Self {
            truth,
            selected_task,
            new_task_draft: Task {
                id: String::new(),
                title: String::new(),
                column_id: columns.first().map(|c| c.id.clone()).unwrap_or_default(),
                assignee_id: None,
                priority: "P2".to_string(),
                due_date: None,
                tag: String::new(),
            },
            dragged_task: None,
            current_filter: "Inbox".to_string(),
            show_add_task_modal: false,
            show_edit_task_modal: false,
            show_add_column_modal: false,
            show_add_user_modal: false,
            new_user_name: String::new(),
            new_column_name: String::new(),
            theme,
            icons,
            assignee_dropdown: DropdownState::default(),
            priority_dropdown: DropdownState::default(),
            column_dropdown: DropdownState::default(),
        }
    }

    pub(crate) fn submit_command(&mut self, command: TodoCommand) {
        let _ = self.truth.submit(command);
        self.truth.drain_board_patches();
    }

    pub(crate) fn tasks(&self) -> Vec<Task> {
        self.truth.live_tasks()
    }

    pub(crate) fn columns(&self) -> Vec<BoardColumn> {
        self.truth.live_columns()
    }

    pub(crate) fn users(&self) -> Vec<BoardUser> {
        self.truth.live_users()
    }

    pub(crate) fn filtered_tasks(&self) -> Vec<Task> {
        let all = self.tasks();
        all.into_iter()
            .filter(|_task| {
                let matches_filter = match self.current_filter.as_str() {
                    "Inbox" => true,
                    _ => true,
                };
                matches_filter
            })
            .collect()
    }

    pub(crate) fn selected(&self) -> Option<Task> {
        self.truth.live_task(&self.selected_task)
    }

    pub(crate) fn create_column(&mut self) {
        if self.new_column_name.trim().is_empty() {
            return;
        }
        self.submit_command(TodoCommand::AddColumn(self.new_column_name.clone()));
        self.new_column_name.clear();
        self.show_add_column_modal = false;
    }

    pub(crate) fn create_user(&mut self) {
        if self.new_user_name.trim().is_empty() {
            return;
        }
        self.submit_command(TodoCommand::AddUser(self.new_user_name.clone()));
        self.new_user_name.clear();
        self.show_add_user_modal = false;
    }

    pub(crate) fn write_field(
        &mut self,
        task_id: String,
        aspect: &str,
        field: &str,
        value: DeclarativeWritebackValue,
    ) {
        let value = match value {
            DeclarativeWritebackValue::String(value) => Value::String(value),
            DeclarativeWritebackValue::Integer(value) => Value::Number(value.into()),
            DeclarativeWritebackValue::Boolean(value) => Value::Bool(value),
            DeclarativeWritebackValue::StructuredJson(value) => {
                serde_json::from_str(&value).unwrap_or(Value::String(value))
            }
        };
        self.submit_command(TodoCommand::UpdateTaskField {
            task_id,
            aspect: aspect.to_string(),
            field: field.to_string(),
            value,
        });
    }

    pub(crate) fn create_task(&mut self) {
        let task = self.new_task_draft.clone();
        if task.title.trim().is_empty() {
            return;
        }

        self.submit_command(TodoCommand::CreateTask(task));

        // Reset draft
        let columns = self.columns();
        self.new_task_draft = Task {
            id: String::new(),
            title: String::new(),
            column_id: columns.first().map(|c| c.id.clone()).unwrap_or_default(),
            assignee_id: None,
            priority: "P2".to_string(),
            due_date: None,
            tag: String::new(),
        };

        self.show_add_task_modal = false;
    }

    pub(crate) fn delete_task(&mut self) {
        if self.selected_task.is_empty() {
            return;
        }
        self.submit_command(TodoCommand::DeleteTask(self.selected_task.clone()));
        self.selected_task.clear();
        self.show_edit_task_modal = false;
    }
}
