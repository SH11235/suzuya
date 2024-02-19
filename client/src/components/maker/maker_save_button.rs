use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::common::api::backend_url;
use crate::model::maker_page::MakerState;
use crate::model::maker_page::PutMakerRequest;

#[derive(Properties, PartialEq)]
pub struct SaveButtonProperty {
    pub input_id: String,
    pub makers_state_handle: UseStateHandle<Vec<MakerState>>,
    pub is_changed: bool,
    pub is_saved: bool,
}

#[function_component(SaveButton)]
pub fn save_button(props: &SaveButtonProperty) -> Html {
    let id_for_update = props.input_id.clone();
    let id_for_save = props.input_id.clone();
    let makers_state_for_update = props.makers_state_handle.clone();
    let makers_state_for_save = props.makers_state_handle.clone();
    let update_onclick = Callback::from(move |_| {
        let id = id_for_update.clone();
        let makers_state = makers_state_for_update.clone();
        let maker_code = {
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
            let put_url = format!("{}{}{}", backend_url(), "/api/maker/", id);
            let client = Request::put(&put_url)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&PutMakerRequest {
                        code_name: maker_code,
                    })
                    .unwrap(),
                );
            let put_response = client.send().await.expect("Failed to update maker");
            if put_response.status() == 200 {
                let mut new_makers = vec![];
                // is_changedをfalseにしてsetする
                for maker_state in makers_state.iter() {
                    if maker_state.id == id {
                        new_makers.push(MakerState {
                            id: maker_state.id.clone(),
                            code_name: maker_state.code_name.clone(),
                            is_changed: false,
                            is_saved: true,
                        });
                    } else {
                        new_makers.push(MakerState {
                            id: maker_state.id.clone(),
                            code_name: maker_state.code_name.clone(),
                            is_changed: maker_state.is_changed,
                            is_saved: maker_state.is_saved,
                        });
                    }
                }
                makers_state.set(new_makers);
            } else {
                // responseが200以外の場合はエラーを出す
                let error_message = format!("Failed to update maker: {}", put_response.status());
                log::error!("{}", error_message);
            }
        });
    });
    let save_onclick = Callback::from(move |_| {
        let id = id_for_save.clone();
        let makers_state = makers_state_for_save.clone();
        let maker_code = {
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
        // maker_codeが空の場合はアラートを出す
        if maker_code.is_empty() {
            let error_message = "メーカーコードを入力してください";
            web_sys::window()
                .unwrap()
                .alert_with_message(error_message)
                .unwrap();
            return;
        }
        wasm_bindgen_futures::spawn_local(async move {
            let post_url = format!("{}{}", backend_url(), "/api/new_maker");
            let client = Request::post(&post_url)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&PutMakerRequest {
                        code_name: maker_code,
                    })
                    .unwrap(),
                );
            let post_response = client.send().await.expect("Failed to update maker");
            if post_response.status() == 201 {
                let mut new_makers = vec![];
                // serverで発行されたidをsetする
                let new_id = post_response
                    .json::<String>()
                    .await
                    .expect("Failed to get new id");
                for maker_state in makers_state.iter() {
                    if maker_state.id == id {
                        new_makers.push(MakerState {
                            id: new_id.clone(),
                            code_name: maker_state.code_name.clone(),
                            is_changed: false,
                            is_saved: true,
                        });
                    } else {
                        new_makers.push(MakerState {
                            id: maker_state.id.clone(),
                            code_name: maker_state.code_name.clone(),
                            is_changed: maker_state.is_changed,
                            is_saved: maker_state.is_saved,
                        });
                    }
                }
                makers_state.set(new_makers);
            } else {
                // responseが200以外の場合はエラーを出す
                let error_message = format!("Failed to update maker: {}", post_response.status());
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&error_message)
                    .unwrap();
                log::error!("{}", error_message);
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
