use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::components::item_detail::ItemDetail;
use crate::model::edit_item::{GetItem, ItemModel, PostItem};
use crate::settings::api::backend_url;
use crate::settings::select::{announce_status_list, catalog_status_list, project_type_list};
use reqwasm::http::Request;
use urlencoding::decode;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, Html,
    Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct EditItemPageProperty {
    pub title: String,
}

#[function_component(EditItem)]
pub fn edit_item(props: &EditItemPageProperty) -> Html {
    let get_item = use_state(|| GetItem {
        ..Default::default()
    });
    let get_url = format!(
        "{}{}",
        backend_url() + "/api/item/",
        decode(&props.title.clone()).expect("UTF-8")
    );
    let onclick = {
        let get_item = get_item.clone();
        Callback::from(move |_| {
            if get_item.items.len() > 0 {
                let get_item = get_item.clone();
                let post_item = PostItem {
                    release_date: get_item.release_date.clone(),
                    reservation_start_date: get_item.reservation_start_date.clone(),
                    reservation_deadline: get_item.reservation_deadline.clone(),
                    order_date: get_item.order_date.clone(),
                    title: get_item.items[0].title.clone(),
                    project_type: "get_item.project_type.clone()".to_string(), // TODO
                    catalog_status: "get_item.catalog_status.clone()".to_string(), // TODO
                    announcement_status: "get_item.announcement_status.clone()".to_string(), // TODO
                    remarks: Some("get_item.rem&&arks.clone()".to_string()),   // TODO
                    items: get_item.items.clone(),
                };
                web_sys::console::log_1(&JsValue::from_serde(&get_item.items[0]).unwrap());
                web_sys::console::log_1(&JsValue::from_serde(&post_item).unwrap());
            }
        })
    };
    {
        let get_item = get_item.clone();
        use_effect_with_deps(
            move |_| {
                let get_item = get_item.clone();
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
                    get_item.set(fetched_items);
                });
                || ()
            },
            (),
        );
    }

    let onchange = {
        let get_item = get_item.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val: String = input.value();
            let name: String = input.name();
            let name = name.as_str();
            match name {
                "release_date" => {
                    let mut items = get_item.items.clone();
                    items = items.iter()
                        .map(|item| ItemModel {
                            release_date: Some(val.clone()),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items: items,
                        release_date: Some(val),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    })
                }
                _ => {
                    todo!();
                }
            }

            // get_item.items
            // web_sys::console::log_1(&JsValue::from_str(&val));
            // web_sys::console::log_1(&JsValue::from_str(name));
        })
    };
    // function handleInputChange(e) {
    //     const target = e.target;
    //     const value = target.type === "checkbox" ? target.checked : target.value;
    //     const name = target.name;
    //     setValues({ ...values, [name]: value });
    // }

    html! {
      <div>
        <h1>{ "Edit Items" }</h1>
        {
            if get_item.items.len() == 0 {
                html! {
                    <p>{ "Loading..." }</p>
                }
            } else {
                let item = &get_item.items[0];
                let release_date = parse_date(&item.release_date);
                let reservation_start_date = parse_date(&item.reservation_start_date);
                let reservation_deadline = parse_date(&item.reservation_deadline);
                let order_date = parse_date(&item.order_date);
                html! {
                    <>
                        <button {onclick}>{ "test button" }</button>
                        <br/>
                        { "発売日：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        { "案内日：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="reservation_start_date" name="reservation_start_date" value={reservation_start_date} />
                        <br/>
                        { "締切日：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="reservation_deadline" name="reservation_deadline" value={reservation_deadline} />
                        <br/>
                        { "発注日：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="order_date" name="order_date" value={order_date} />
                        <br/>
                        { "タイトル：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="title" name="title" value={item.title.clone()} />
                        <br/>
                        { "案件：" }<SelectBox id="project_type" name="project_type" value={item.project_type.clone()} select_list={project_type_list()} />
                        <br/>
                        { "最終更新日：" } <span>{ parse_date_time(&item.last_updated) }</span>
                        <br/>
                        <ItemDetail get_item={get_item.clone()} index={1} item_name={item.name.clone()}
                            product_code={item.product_code.clone()} sku={item.sku.clone()} illust_status={item.illust_status.clone()} />
                        <br/>
                        { "カタログステータス：" }<SelectBox id="catalog_status" name="catalog_status" value={item.catalog_status.clone()} select_list={catalog_status_list()} />
                        <br/>
                        { "告知：" }<SelectBox id="announcement_status" name="announcement_status" value={item.announcement_status.clone()} select_list={announce_status_list()} />
                        <br/>
                        { "備考：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="備考" id="remarks" name="title" value={item.remarks.clone().unwrap_or("".to_string())} />
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
