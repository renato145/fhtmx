use crate::{
    error::AppError,
    html_elements::{sort_list_btn, todo_list_description},
    state::State,
};
use actix_web::{HttpResponse, web};
use fhtmx::prelude::*;
use fhtmx_actix::prelude::*;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TodoContent {
    pub content: String,
}

#[tracing::instrument(skip(state))]
pub async fn add_todo(
    web::Form(params): web::Form<TodoContent>,
    state: web::Data<State>,
) -> Result<HttpResponse, AppError> {
    let (new_item, n) = state.add_item(params.content)?;
    let mut html_body = vec![
        new_item.into_element(),
        todo_list_description(n).hx_swap_oob("true"),
    ];
    if n == 1 {
        html_body.push(sort_list_btn(1).hx_swap_oob("true"));
    }
    Ok(html_body.render_response())
}

#[tracing::instrument(skip(state))]
pub async fn rm_todo(
    id: web::Path<String>,
    state: web::Data<State>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&id).map_err(AppError::InvalidUuid)?;
    let mut todo_list = state.todo_list.lock().unwrap();
    let idx = todo_list
        .iter()
        .position(|o| o.id == id)
        .ok_or(AppError::IdNotFound)?;
    todo_list.remove(idx);
    let mut html_body = vec![todo_list_description(todo_list.len()).hx_swap_oob("true")];
    if todo_list.is_empty() {
        html_body.push(sort_list_btn(0).hx_swap_oob("true"));
    }
    Ok(html_body.render_response())
}

#[tracing::instrument(skip(state))]
pub async fn todo_form(
    id: web::Path<String>,
    state: web::Data<State>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&id).map_err(AppError::InvalidUuid)?;
    let response = state
        .get_item(id)
        .ok_or(AppError::IdNotFound)?
        .html_form()
        .render_response();
    Ok(response)
}

#[tracing::instrument(skip(state))]
pub async fn get_todo(
    id: web::Path<String>,
    state: web::Data<State>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&id).map_err(AppError::InvalidUuid)?;
    let response = state
        .get_item(id)
        .ok_or(AppError::IdNotFound)?
        .into_element()
        .render_response();
    Ok(response)
}

#[tracing::instrument(skip(state))]
pub async fn update_todo(
    id: web::Path<String>,
    web::Form(params): web::Form<TodoContent>,
    state: web::Data<State>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&id).map_err(AppError::InvalidUuid)?;
    let mut todo_list = state.todo_list.lock().unwrap();
    let item = todo_list
        .iter_mut()
        .find(|o| o.id == id)
        .ok_or(AppError::IdNotFound)?;
    item.value = params.content;
    Ok(item.clone().into_element().render_response())
}

#[tracing::instrument(skip(state))]
pub async fn sort_todo(state: web::Data<State>) -> HttpResponse {
    let mut todo_list = state.todo_list.lock().unwrap();
    todo_list.sort_by_key(|o| o.value.clone());
    todo_list.clone().into_node().render_response()
}

#[tracing::instrument(skip(state))]
pub async fn clear_todo(state: web::Data<State>) -> HttpResponse {
    state.todo_list.lock().unwrap().clear();
    fragment([
        todo_list_description(0).hx_swap_oob("true"),
        sort_list_btn(0).hx_swap_oob("true"),
    ])
    .render_response()
}
