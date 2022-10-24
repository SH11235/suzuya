use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::components::item_detail::ItemDetail;
use crate::model::edit_item::{GetItemInfoByTitleId, ItemModel};
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
    pub title_id: String,
}

#[function_component(EditItem)]
pub fn edit_item(props: &EditItemPageProperty) -> Html {
    let get_info_by_title_id = use_state(|| GetItemInfoByTitleId {
        ..Default::default()
    });
    let get_url = format!(
        "{}{}",
        backend_url() + "/api/item/",
        decode(&props.title_id.clone()).expect("UTF-8")
    );
    {
        let get_info_by_title_id = get_info_by_title_id.clone();
        use_effect_with_deps(
            move |_| {
                let get_info_by_title_id = get_info_by_title_id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let mut fetched_items: GetItemInfoByTitleId = client
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
                    get_info_by_title_id.set(fetched_items);
                });
                || ()
            },
            (),
        );
    }

    let onchange = {
        let get_info_by_title_id = get_info_by_title_id.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val: String = input.value();
            let name: String = input.name();
            let name = name.as_str();
            let items = get_info_by_title_id.items.clone();
            let title = get_info_by_title_id.title.clone();
            let workers = get_info_by_title_id.workers.clone();
            let makers = get_info_by_title_id.makers.clone();
            match name {
                "release_date" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: Some(val),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "reservation_start_date" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: Some(val),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "reservation_deadline" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: Some(val),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "order_date_to_maker" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: Some(val),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "title" => {
                    let title = val.clone();
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "project_type" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: val.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "catalog_status" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: val.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "announcement_status" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: val.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
                    });
                }
                "remarks" => {
                    get_info_by_title_id.set(GetItemInfoByTitleId {
                        items,
                        title,
                        workers,
                        makers,
                        release_date: get_info_by_title_id.release_date.clone(),
                        reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                        reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                        order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                        project_type: get_info_by_title_id.project_type.clone(),
                        catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: Some(val.clone()),
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
        let get_info_by_title_id = get_info_by_title_id.clone();
        Callback::from(move |_| {
            let mut items = get_info_by_title_id.items.clone();
            let title = get_info_by_title_id.title.clone();
            let workers = get_info_by_title_id.workers.clone();
            let makers = get_info_by_title_id.makers.clone();
            let new_item = ItemModel {
                ..Default::default()
            };
            items.push(new_item);
            get_info_by_title_id.set(GetItemInfoByTitleId {
                items,
                title,
                workers,
                makers,
                release_date: get_info_by_title_id.release_date.clone(),
                reservation_start_date: get_info_by_title_id.reservation_start_date.clone(),
                reservation_deadline: get_info_by_title_id.reservation_deadline.clone(),
                order_date_to_maker: get_info_by_title_id.order_date_to_maker.clone(),
                project_type: get_info_by_title_id.project_type.clone(),
                catalog_status: get_info_by_title_id.catalog_status.clone(),
                        announcement_status: get_info_by_title_id.announcement_status.clone(),
                        remarks: get_info_by_title_id.remarks.clone(),
            });
        })
    };

    html! {
      <div class="edit-item-page">
        <h1>{ "Edit Items" }</h1>
        {
            if get_info_by_title_id.items.len() == 0 {
                html! {
                    <p>{ "Loading..." }</p>
                }
            } else {
                let release_date = parse_date(&get_info_by_title_id.release_date);
                let reservation_start_date = parse_date(&get_info_by_title_id.reservation_start_date);
                let reservation_deadline = parse_date(&get_info_by_title_id.reservation_deadline);
                let order_date_to_maker = parse_date(&get_info_by_title_id.order_date_to_maker);
                let items = get_info_by_title_id.items.clone();
                let mut index: usize = 0;
                html! {
                    <>
                        { "発売日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        { "案内日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_start_date" name="reservation_start_date" value={reservation_start_date} />
                        <br/>
                        { "締切日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_deadline" name="reservation_deadline" value={reservation_deadline} />
                        <br/>
                        { "発注日：" }<TextBox onchange={onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="order_date_to_maker" name="order_date_to_maker" value={order_date_to_maker} />
                        <br/>
                        { "タイトル：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="title" name="title" value={get_info_by_title_id.title.clone()} />
                        <br/>
                        { "案件：" }<SelectBox onchange={onchange.clone()} id="project_type" name="project_type" value={get_info_by_title_id.project_type.clone()} select_list={project_type_list()} />
                        <br/>
                        {
                            html! {
                                <div class="item-detail-wrapper">
                                {
                                    items.into_iter().map(|item| {
                                        index += 1;
                                        html! {
                                            <ItemDetail
                                                id={item.id.clone()}
                                                get_info_by_title_id={get_info_by_title_id.clone()}
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
                        { "カタログステータス：" }<SelectBox onchange={onchange.clone()} id="catalog_status" name="catalog_status" value={get_info_by_title_id.catalog_status.clone()} select_list={catalog_status_list()} />
                        <br/>
                        { "告知：" }<SelectBox onchange={onchange.clone()} id="announcement_status" name="announcement_status" value={get_info_by_title_id.announcement_status.clone()} select_list={announce_status_list()} />
                        <br/>
                        { "備考：" }<TextBox onchange={onchange.clone()} input_type="text" placeholder="備考" id="remarks" name="remarks" value={get_info_by_title_id.remarks.clone().unwrap_or("".to_string())} />
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
    web_sys::console::log_1(&JsValue::from_str(&date));
    let date = date.replace("/", "-");
    let date_str = &date[0..10];
    format!("{}", date_str)
}
