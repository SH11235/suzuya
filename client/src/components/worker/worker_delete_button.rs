use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::model::worker_page::WorkerState;
use crate::common::api::backend_url;

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProperty {
    pub input_id: String,
    pub workers_state_handle: UseStateHandle<Vec<WorkerState>>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &DeleteButtonProperty) -> Html {
    let id_for_db = props.input_id.clone();
    let id_for_view = props.input_id.clone();
    let workers_state_for_db = props.workers_state_handle.clone();
    let workers_state_for_view = props.workers_state_handle.clone();
    let target_worker = props.workers_state_handle.iter().find(|worker| worker.id == props.input_id).unwrap();
    let delete_from_db = Callback::from(move |_| {
        // confirmを出す
        let confirm = web_sys::window()
            .unwrap()
            .confirm_with_message("本当に消しますか？")
            .unwrap();
        if confirm {
            let id = id_for_db.clone();
            let workers_state = workers_state_for_db.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let delete_url = format!("{}{}{}", backend_url(), "/api/delete_worker/", id);
                let client = Request::delete(&delete_url);
                let delete_response = client.send().await.expect("Failed to delete worker");
                // responseが200以外の場合はエラーを出す
                if delete_response.status() == 200 {
                    let mut new_workers = vec![];
                    for worker_state in workers_state.iter() {
                        if worker_state.id != id {
                            new_workers.push(WorkerState {
                                id: worker_state.id.clone(),
                                name: worker_state.name.clone(),
                                is_changed: worker_state.is_changed,
                                is_saved: worker_state.is_saved,
                            });
                        }
                    }
                    workers_state.set(new_workers);
                } else {
                    let error_message =
                        format!("Failed to delete worker: {}", delete_response.status());
                    let error_message = JsValue::from_str(&error_message);
                    web_sys::console::log_1(&error_message);
                }
            });
        }
    });

    let delete_from_view = Callback::from(move |_| {
        let id = id_for_view.clone();
        let workers_state = workers_state_for_view.clone();
        let mut new_workers = vec![];
        for worker_state in workers_state.iter() {
            if worker_state.id != id {
                new_workers.push(WorkerState {
                    id: worker_state.id.clone(),
                    name: worker_state.name.clone(),
                    is_changed: worker_state.is_changed,
                    is_saved: worker_state.is_saved,
                });
            }
        }
        workers_state.set(new_workers);
    });

    if target_worker.is_saved {
        html! {
            <button onclick={ delete_from_db }>{ "削除" }</button>
        }
    } else {
        html! {
            <button onclick={ delete_from_view }>{ "削除" }</button>
        }
    }
}
