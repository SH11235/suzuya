use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::common::api::backend_url;
use crate::model::item_page::ItemState;

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProperty {
    pub input_id: String,
    pub items_state_handle: UseStateHandle<Vec<ItemState>>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &DeleteButtonProperty) -> Html {
    let id_for_db = props.input_id.clone();
    let id_for_view = props.input_id.clone();
    let items_state_for_db = props.items_state_handle.clone();
    let items_state_for_view = props.items_state_handle.clone();
    let target_item = props
        .items_state_handle
        .iter()
        .find(|item| item.id == props.input_id)
        .unwrap();
    let delete_from_db = Callback::from(move |_| {
        // confirmを出す
        let confirm = web_sys::window()
            .unwrap()
            .confirm_with_message("本当に消しますか？")
            .unwrap();
        if confirm {
            let id = id_for_db.clone();
            let items_state = items_state_for_db.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let delete_url = format!("{}{}{}", backend_url(), "/api/delete_item/", id);
                let client = Request::delete(&delete_url);
                let delete_response = client.send().await.expect("Failed to delete item");
                // responseが200以外の場合はエラーを出す
                if delete_response.status() == 200 {
                    let mut new_items = vec![];
                    for item_state in items_state.iter() {
                        if item_state.id != id {
                            new_items.push(ItemState {
                                id: item_state.id.clone(),
                                name: item_state.name.clone(),
                                product_code: item_state.product_code.clone(),
                                sku: item_state.sku.clone(),
                                illust_status: item_state.illust_status.clone(),
                                pic_illust_id: item_state.pic_illust_id.clone(),
                                design_status: item_state.design_status.clone(),
                                pic_design_id: item_state.pic_design_id.clone(),
                                maker_id: item_state.maker_id.clone(),
                                retail_price: item_state.retail_price.clone(),
                                resubmission: item_state.resubmission.clone(),
                                line: item_state.line.clone(),
                                rough_coordinator_id: item_state.rough_coordinator_id.clone(),
                                rough_check_person_id: item_state.rough_check_person_id.clone(),
                                line_art_coordinator_id: item_state.line_art_coordinator_id.clone(),
                                line_art_check_person_id: item_state
                                    .line_art_check_person_id
                                    .clone(),
                                coloring_coordinator_id: item_state.coloring_coordinator_id.clone(),
                                coloring_check_person_id: item_state
                                    .coloring_check_person_id
                                    .clone(),
                                design_coordinator_id: item_state.design_coordinator_id.clone(),
                                design_check_person_id: item_state.design_check_person_id.clone(),
                                submission_data_coordinator_id: item_state
                                    .submission_data_coordinator_id
                                    .clone(),
                                submission_data_check_person_id: item_state
                                    .submission_data_check_person_id
                                    .clone(),
                                announcement_materials_coordinator_id: item_state
                                    .announcement_materials_coordinator_id
                                    .clone(),
                                announcement_materials_check_person_id: item_state
                                    .announcement_materials_check_person_id
                                    .clone(),
                                jan_coordinator_id: item_state.jan_coordinator_id.clone(),
                                jan_check_person_id: item_state.jan_check_person_id.clone(),
                                is_saved: item_state.is_saved,
                            });
                        }
                    }
                    items_state.set(new_items);
                } else {
                    let error_message =
                        format!("Failed to delete item: {}", delete_response.status());
                    let error_message = JsValue::from_str(&error_message);
                    web_sys::console::log_1(&error_message);
                }
            });
        }
    });

    let delete_from_view = Callback::from(move |_| {
        let id = id_for_view.clone();
        let items_state = items_state_for_view.clone();
        let mut new_items = vec![];
        for item_state in items_state.iter() {
            if item_state.id != id {
                new_items.push(ItemState {
                    id: item_state.id.clone(),
                    name: item_state.name.clone(),
                    product_code: item_state.product_code.clone(),
                    sku: item_state.sku.clone(),
                    illust_status: item_state.illust_status.clone(),
                    pic_illust_id: item_state.pic_illust_id.clone(),
                    design_status: item_state.design_status.clone(),
                    pic_design_id: item_state.pic_design_id.clone(),
                    maker_id: item_state.maker_id.clone(),
                    retail_price: item_state.retail_price.clone(),
                    resubmission: item_state.resubmission.clone(),
                    line: item_state.line.clone(),
                    rough_coordinator_id: item_state.rough_coordinator_id.clone(),
                    rough_check_person_id: item_state.rough_check_person_id.clone(),
                    line_art_coordinator_id: item_state.line_art_coordinator_id.clone(),
                    line_art_check_person_id: item_state.line_art_check_person_id.clone(),
                    coloring_coordinator_id: item_state.coloring_coordinator_id.clone(),
                    coloring_check_person_id: item_state.coloring_check_person_id.clone(),
                    design_coordinator_id: item_state.design_coordinator_id.clone(),
                    design_check_person_id: item_state.design_check_person_id.clone(),
                    submission_data_coordinator_id: item_state
                        .submission_data_coordinator_id
                        .clone(),
                    submission_data_check_person_id: item_state
                        .submission_data_check_person_id
                        .clone(),
                    announcement_materials_coordinator_id: item_state
                        .announcement_materials_coordinator_id
                        .clone(),
                    announcement_materials_check_person_id: item_state
                        .announcement_materials_check_person_id
                        .clone(),
                    jan_coordinator_id: item_state.jan_coordinator_id.clone(),
                    jan_check_person_id: item_state.jan_check_person_id.clone(),
                    is_saved: item_state.is_saved,
                });
            }
        }
        items_state.set(new_items);
    });

    if target_item.is_saved {
        html! {
            <button onclick={ delete_from_db }>{ "削除" }</button>
        }
    } else {
        html! {
            <button onclick={ delete_from_view }>{ "削除" }</button>
        }
    }
}
