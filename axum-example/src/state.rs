use crate::{domain::TodoListItem, error::AppError};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Default)]
pub struct AppState {
    pub todo_list: Mutex<Vec<TodoListItem>>,
}

impl AppState {
    pub fn read_todo_list(&self) -> Vec<TodoListItem> {
        self.todo_list.lock().unwrap().clone()
    }

    pub fn get_item(&self, id: Uuid) -> Option<TodoListItem> {
        self.todo_list
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.id == id)
            .cloned()
    }

    pub fn add_item(&self, new: String) -> Result<(TodoListItem, usize), AppError> {
        if new.chars().any(|o| char::is_ascii_punctuation(&o)) {
            return Err(AppError::InvalidContent);
        }
        let new_item = TodoListItem::new(new);
        let mut todo_list = self.todo_list.lock().unwrap();
        todo_list.push(new_item.clone());
        Ok((new_item, todo_list.len()))
    }
}
