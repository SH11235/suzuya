use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::model::common::MakerModel;
use crate::model::maker_page::MakerState;
use crate::model::maker_page::PutMakerRequest;
use crate::settings::api::backend_url;

#[derive(Properties, PartialEq)]
pub struct SaveButtonProperty {
    pub input_id: String,
    pub makers_state_handle: UseStateHandle<Vec<MakerState>>,
    pub is_changed: bool,
}

#[function_component(SaveButton)]
pub fn save_button(props: &SaveButtonProperty) -> Html {
    let id = props.input_id.clone();
    let makers_state = props.makers_state_handle.clone();
    let save_onclick = Callback::from(move |_| {
        let id = id.clone();
        let makers_state = makers_state.clone();
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
                        });
                    } else {
                        new_makers.push(MakerState {
                            id: maker_state.id.clone(),
                            code_name: maker_state.code_name.clone(),
                            is_changed: maker_state.is_changed,
                        });
                    }
                }
                makers_state.set(new_makers);
            } else { // responseが200以外の場合はエラーを出す
                let error_message = format!("Failed to update maker: {}", put_response.status());
                let error_message = JsValue::from_str(&error_message);
                web_sys::console::error_1(&error_message);
            }
        });
    });
    if props.is_changed {
        html! {
            <button onclick={ save_onclick }>{ "更新" }</button>
        }
    } else {
        html! {
            <button disabled=true>{ "更新" }</button>
        }
    }
}
