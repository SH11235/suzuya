use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::components::item_detail::ItemDetail;
use crate::model::edit_item::{GetItem, ItemModel};
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
    {
        let get_item = get_item.clone();
        use_effect_with_deps(
            move |_| {
                let get_item = get_item.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let mut fetched_items: GetItem = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    fetched_items
                        .items
                        .sort_by(|a, b| a.product_code.cmp(&b.product_code));
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
            let mut items = get_item.items.clone();
            let users = get_item.users.clone();
            let makers = get_item.makers.clone();
            match name {
                "release_date" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            release_date: Some(val.clone()),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: Some(val),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "reservation_start_date" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            reservation_start_date: Some(val.clone()),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: Some(val),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "reservation_deadline" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            reservation_deadline: Some(val.clone()),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: Some(val),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "order_date" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            order_date: Some(val.clone()),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: Some(val),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "title" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            title: val.clone(),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "project_type" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            project_type: val.clone(),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "catalog_status" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            catalog_status: val.clone(),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "announcement_status" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            announcement_status: val.clone(),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "remarks" => {
                    items = items
                        .iter()
                        .map(|item| ItemModel {
                            remarks: Some(val.clone()),
                            ..item.clone()
                        })
                        .collect();
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                _ => {
                    web_sys::console::log_1(&JsValue::from_str("No defined state."));
                    web_sys::console::log_1(&JsValue::from_str("name"));
                    web_sys::console::log_1(&JsValue::from_str(name));
                    web_sys::console::log_1(&JsValue::from_str("&val"));
                    web_sys::console::log_1(&JsValue::from_str(&val));
                }
            }
        })
    };

    let item_add_click = {
        let get_item = get_item.clone();
        Callback::from(move |_| {
            let mut items = get_item.items.clone();
            let users = get_item.users.clone();
            let makers = get_item.makers.clone();
            let new_item = ItemModel {
                ..Default::default()
            };
            items.push(new_item);
            get_item.set(GetItem {
                items,
                users,
                makers,
                release_date: get_item.release_date.clone(),
                reservation_start_date: get_item.reservation_start_date.clone(),
                reservation_deadline: get_item.reservation_deadline.clone(),
                order_date: get_item.order_date.clone(),
                last_updated: get_item.last_updated.clone(),
            });
        })
    };

    html! {
      <div class="edit-item-page">
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
                let items = get_item.items.clone();
                let mut index: usize = 0;
                html! {
                    <>
                        { "発売日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        { "案内日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_start_date" name="reservation_start_date" value={reservation_start_date} />
                        <br/>
                        { "締切日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_deadline" name="reservation_deadline" value={reservation_deadline} />
                        <br/>
                        { "発注日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="order_date" name="order_date" value={order_date} />
                        <br/>
                        { "タイトル：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="title" name="title" value={item.title.clone()} />
                        <br/>
                        { "案件：" }<SelectBox onchange={onchange.clone()} id="project_type" name="project_type" value={item.project_type.clone()} select_list={project_type_list()} />
                        <br/>
                        { "最終更新日：" } <span>{ parse_date_time(&item.last_updated) }</span>
                        {
                            html! {
                                <div class="item-detail-wrapper">
                                {
                                    items.into_iter().map(|item| {
                                        index += 1;
                                        html! {
                                            <ItemDetail
                                                id={item.id.clone()}
                                                get_item={get_item.clone()}
                                                index={index}
                                                item_name={item.name.clone()}
                                                product_code={item.product_code.clone()}
                                                sku={item.sku.clone()}
                                                illust_status={item.illust_status.clone()}
                                                pic_illust_id={item.pic_illust_id.clone()}
                                                design_status={item.design_status.clone()}
                                                pic_design_id={item.pic_design_id.clone()}
                                                maker_id={item.maker_id.clone()}
                                                retail_price={item.retail_price.clone()}
                                                double_check_person_id={item.double_check_person_id.clone()}
                                            />
                                        }
                                    }).collect::<Html>()
                                }
                                    <button onclick={item_add_click} class="item-add-button" id="item-add-button">
                                        { "+" }
                                    </button>
                                </div>
                            }
                        }
                        { "カタログステータス：" }<SelectBox onchange={onchange.clone()} id="catalog_status" name="catalog_status" value={item.catalog_status.clone()} select_list={catalog_status_list()} />
                        <br/>
                        { "告知：" }<SelectBox onchange={onchange.clone()} id="announcement_status" name="announcement_status" value={item.announcement_status.clone()} select_list={announce_status_list()} />
                        <br/>
                        { "備考：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="備考" id="remarks" name="remarks" value={item.remarks.clone().unwrap_or("".to_string())} />
                        <br/>
                        {
                            html! {
                                <button class="save-button" id="btn_submit">
                                    { "保存" }
                                </button>
                            }
                        }
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
