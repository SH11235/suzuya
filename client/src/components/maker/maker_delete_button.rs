use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::model::maker_page::MakerState;
use crate::common::api::backend_url;

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProperty {
    pub input_id: String,
    pub makers_state_handle: UseStateHandle<Vec<MakerState>>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &DeleteButtonProperty) -> Html {
    let id_for_db = props.input_id.clone();
    let id_for_view = props.input_id.clone();
    let makers_state_for_db = props.makers_state_handle.clone();
    let makers_state_for_view = props.makers_state_handle.clone();
    let target_maker = props.makers_state_handle.iter().find(|maker| maker.id == props.input_id).unwrap();
    let delete_from_db = Callback::from(move |_| {
        // confirmを出す
        let confirm = web_sys::window()
            .unwrap()
            .confirm_with_message("本当に消しますか？")
            .unwrap();
        if confirm {
            let id = id_for_db.clone();
            let makers_state = makers_state_for_db.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let delete_url = format!("{}{}{}", backend_url(), "/api/delete_maker/", id);
                let client = Request::delete(&delete_url);
                let delete_response = client.send().await.expect("Failed to delete maker");
                // responseが200以外の場合はエラーを出す
                if delete_response.status() == 200 {
                    let mut new_makers = vec![];
                    for maker_state in makers_state.iter() {
                        if maker_state.id != id {
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
                    let error_message =
                        format!("Failed to delete maker: {}", delete_response.status());
                    let error_message = JsValue::from_str(&error_message);
                    web_sys::console::log_1(&error_message);
                }
            });
        }
    });

    let delete_from_view = Callback::from(move |_| {
        let id = id_for_view.clone();
        let makers_state = makers_state_for_view.clone();
        let mut new_makers = vec![];
        for maker_state in makers_state.iter() {
            if maker_state.id != id {
                new_makers.push(MakerState {
                    id: maker_state.id.clone(),
                    code_name: maker_state.code_name.clone(),
                    is_changed: maker_state.is_changed,
                    is_saved: maker_state.is_saved,
                });
            }
        }
        makers_state.set(new_makers);
    });

    if target_maker.is_saved {
        html! {
            <button onclick={ delete_from_db }>{ "削除" }</button>
        }
    } else {
        html! {
            <button onclick={ delete_from_view }>{ "削除" }</button>
        }
    }
}
