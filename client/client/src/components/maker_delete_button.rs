use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::model::common::MakerModel;
use crate::model::maker_page::MakerState;
use crate::model::maker_page::PutMakerRequest;
use crate::settings::common::backend_url;

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProperty {
    pub input_id: String,
    pub makers_state_handle: UseStateHandle<Vec<MakerState>>,
}

#[function_component(SaveButton)]
pub fn delete_button(props: &DeleteButtonProperty) -> Html {
    let id = props.input_id.clone();
    let makers_state = props.makers_state_handle.clone();
    let delete_onclick = Callback::from(move |_| {
        let id = id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let delete_url = format!("{}{}{}", backend_url(), "/api/delete_maker/", id);
            // console.log(delete_url);
            web_sys::console::log_1(&JsValue::from_str(&delete_url));
            let client = Request::delete(&delete_url);
            let delete_response = client.send().await.expect("Failed to delete maker");
            // responseが200以外の場合はエラーを出す
            if delete_response.status() != 200 {
                // console.log
                let error_message = format!("Failed to delete maker: {}", delete_response.status());
                let error_message = JsValue::from_str(&error_message);
                web_sys::console::log_1(&error_message);
            } else {
                let error_message = format!("Failed to delete maker: {}", delete_response.status());
                let error_message = JsValue::from_str(&error_message);
                web_sys::console::log_1(&error_message);
            }
        });
    });
    html! {
        <button onclick={ delete_onclick }>{ "削除" }</button>
    }
}
