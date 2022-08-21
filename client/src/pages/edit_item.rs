use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::model::edit_item::{GetItem, PostItem};
use crate::settings::api::backend_url;
use crate::settings::select::{announce_status_list, catalog_status_list, project_type_list};
use reqwasm::http::Request;
use urlencoding::decode;
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct EditItemPageProperty {
    pub title: String,
}

#[function_component(EditItem)]
pub fn edit_item(props: &EditItemPageProperty) -> Html {
    let items = use_state(|| vec![]);
    let get_url = format!(
        "{}{}",
        backend_url() + "/api/item/",
        decode(&props.title.clone()).expect("UTF-8")
    );
    let onclick = {
        let items = items.clone();
        Callback::from(move |_| {
            if items.len() > 0 {
                let item: &GetItem = &items[0];
                let post_item = PostItem {
                    release_date: item.release_date.clone(),
                    reservation_start_date: item.reservation_start_date.clone(),
                    reservation_deadline: item.reservation_deadline.clone(),
                    order_date: item.order_date.clone(),
                    title: item.items[0].title.clone(),
                    project_type: "item.project_type.clone()".to_string(), // TODO
                    catalog_status: "item.catalog_status.clone()".to_string(), // TODO
                    announcement_status: "item.announcement_status.clone()".to_string(), // TODO
                    remarks: Some("item.remarks.clone()".to_string()),     // TODO
                    items: item.items.clone(),
                };
                web_sys::console::log_1(&JsValue::from_serde(&items[0]).unwrap());
                web_sys::console::log_1(&JsValue::from_serde(&post_item).unwrap());
            }
        })
    };
    {
        let items = items.clone();
        use_effect_with_deps(
            move |_| {
                let items = items.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let fetched_items: GetItem = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    // debug
                    web_sys::console::log_1(&JsValue::from_serde(&fetched_items).unwrap());
                    items.set(vec![fetched_items]);
                });
                || ()
            },
            (),
        );
    }

    html! {
      <div>
        <h1>{ "Edit Items" }</h1>
        {
            if items.len() == 0 {
                html! {
                    <p>{ "Loading..." }</p>
                }
            } else {
                let item = &items[0].items[0];
                let release_date = parse_date(&item.release_date);
                let reservation_start_date = parse_date(&item.reservation_start_date);
                let reservation_deadline = parse_date(&item.reservation_deadline);
                let order_date = parse_date(&item.order_date);
                html! {
                    <>
                        <button {onclick}>{ "test button" }</button>
                        <br/>
                        { "発売日：" }<TextBox input_type="text" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        { "案内日：" }<TextBox input_type="text" placeholder="yyyy-mm-dd" id="reservation_start_date" name="reservation_start_date" value={reservation_start_date} />
                        <br/>
                        { "締切日：" }<TextBox input_type="text" placeholder="yyyy-mm-dd" id="reservation_deadline" name="reservation_deadline" value={reservation_deadline} />
                        <br/>
                        { "発注日：" }<TextBox input_type="text" placeholder="yyyy-mm-dd" id="order_date" name="order_date" value={order_date} />
                        <br/>
                        { "タイトル：" }<TextBox input_type="text" placeholder="yyyy-mm-dd" id="title" name="title" value={item.title.clone()} />
                        <br/>
                        { "案件：" }<SelectBox id="project_type" name="project_type" value={item.project_type.clone()} select_list={project_type_list()} />
                        <br/>
                        { "最終更新日：" } <span>{ parse_date_time(&item.last_updated) }</span>
                        <br/>
                        { "ここにアイテム詳細が並ぶ" }
                        <br/>
                        { "カタログステータス：" }<SelectBox id="catalog_status" name="catalog_status" value={item.catalog_status.clone()} select_list={catalog_status_list()} />
                        <br/>
                        { "告知：" }<SelectBox id="announcement_status" name="announcement_status" value={item.announcement_status.clone()} select_list={announce_status_list()} />
                        <br/>
                    </>
                }
            }
        }
      </div>
    }
}

fn parse_date(date: &Option<String>) -> String {
    match date {
        Some(date) => (&date[0..10]).to_string(),
        None => "".to_string(),
    }
}

fn parse_date_time(date: &String) -> String {
    let date_str = &date[0..10];
    let time = &date[11..19];
    format!("{} {}", date_str, time)
}
