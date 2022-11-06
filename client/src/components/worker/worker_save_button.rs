use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::model::worker_page::WorkerState;
use crate::model::worker_page::PutWorkerRequest;
use crate::common::api::backend_url;

#[derive(Properties, PartialEq)]
pub struct SaveButtonProperty {
    pub input_id: String,
    pub workers_state_handle: UseStateHandle<Vec<WorkerState>>,
    pub is_changed: bool,
    pub is_saved: bool,
}

#[function_component(SaveButton)]
pub fn save_button(props: &SaveButtonProperty) -> Html {
    let id_for_update = props.input_id.clone();
    let id_for_save = props.input_id.clone();
    let workers_state_for_update = props.workers_state_handle.clone();
    let workers_state_for_save = props.workers_state_handle.clone();
    let update_onclick = Callback::from(move |_| {
        let id = id_for_update.clone();
        let workers_state = workers_state_for_update.clone();
        let worker_name = {
            let input: web_sys::HtmlInputElement = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(&id)
                .unwrap()
                .dyn_into()
                .unwrap();
            input.value()
        };
        wasm_bindgen_futures::spawn_local(async move {
            let put_url = format!("{}{}{}", backend_url(), "/api/worker/", id);
            let client = Request::put(&put_url)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&PutWorkerRequest {
                        name: worker_name,
                    })
                    .unwrap(),
                );
            let put_response = client.send().await.expect("Failed to update worker");
            if put_response.status() == 200 {
                let mut new_workers = vec![];
                // is_changedをfalseにしてsetする
                for worker_state in workers_state.iter() {
                    if worker_state.id == id {
                        new_workers.push(WorkerState {
                            id: worker_state.id.clone(),
                            name: worker_state.name.clone(),
                            is_changed: false,
                            is_saved: true,
                        });
                    } else {
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
                // responseが200以外の場合はエラーを出す
                let error_message = format!("Failed to update worker: {}", put_response.status());
                let error_message = JsValue::from_str(&error_message);
                web_sys::console::error_1(&error_message);
            }
        });
    });
    let save_onclick = Callback::from(move |_| {
        let id = id_for_save.clone();
        let workers_state = workers_state_for_save.clone();
        let worker_name = {
            let input: web_sys::HtmlInputElement = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(&id)
                .unwrap()
                .dyn_into()
                .unwrap();
            input.value()
        };
        // worker_nameが空の場合はアラートを出す
        if worker_name == "" {
            let error_message = "名前を入力してください";
            web_sys::window()
                .unwrap()
                .alert_with_message(&error_message)
                .unwrap();
            return;
        }
        wasm_bindgen_futures::spawn_local(async move {
            let post_url = format!("{}{}", backend_url(), "/api/new_worker");
            let client = Request::post(&post_url)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&PutWorkerRequest {
                        name: worker_name,
                    })
                    .unwrap(),
                );
            let post_response = client.send().await.expect("Failed to update worker");
            if post_response.status() == 201 {
                let mut new_workers = vec![];
                // serverで発行されたidをsetする
                let new_id = post_response
                    .json::<String>()
                    .await
                    .expect("Failed to get new id");
                for worker_state in workers_state.iter() {
                    if worker_state.id == id {
                        new_workers.push(WorkerState {
                            id: new_id.clone(),
                            name: worker_state.name.clone(),
                            is_changed: false,
                            is_saved: true,
                        });
                    } else {
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
                // responseが200以外の場合はエラーを出す
                let error_message = format!("Failed to update worker: {}", post_response.status());
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&error_message)
                    .unwrap();
                let error_message = JsValue::from_str(&error_message);
                web_sys::console::error_1(&error_message);
            }
        });
    });
    if props.is_changed {
        if props.is_saved {
            html! {
                <button onclick={ update_onclick }>{ "更新" }</button>
            }
        } else {
            html! {
                <button onclick={ save_onclick }>{ "保存" }</button>
            }
        }
    } else {
        html! {
            <button disabled=true>{ "更新" }</button>
        }
    }
}
