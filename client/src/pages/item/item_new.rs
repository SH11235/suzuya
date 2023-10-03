use crate::common::api::backend_url;
use crate::common::date_util::{date_string_to_iso_string, parse_date};
use crate::common::select::{announce_status_list, catalog_status_list, project_type_list};
use crate::components::common::header_link::HeaderLink;
use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::components::item::item_detail::ItemDetail;
use crate::model::common::NameOptionIdPair;
use crate::model::item_page::{
    ItemNewResponse, ItemRegisterParams, ItemState, RequestPutTitleInfo, TitleInfo, TitleState,
};
use reqwasm::http::Request;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, Html,
    TargetCast,
};

#[function_component(ItemNew)]
pub fn edit_item() -> Html {
    let title_state = use_state(|| TitleState {
        id: Uuid::new_v4().to_string(),
        ..Default::default()
    });
    let initial_items: Vec<ItemState> = vec![];
    let items_state = use_state(|| initial_items);
    let makers_state = use_state(|| vec![]);
    let workers_state = use_state(|| vec![]);
    let fetching_state = use_state(|| true);
    let get_url = format!("{}", backend_url() + "/api/item_new");
    {
        let makers_state = makers_state.clone();
        let workers_state = workers_state.clone();
        let fetching_state = fetching_state.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let fetched_items: ItemNewResponse = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");

                    let mut makers = vec![];
                    fetched_items.makers.iter().for_each(|maker| {
                        makers.push(NameOptionIdPair {
                            name: maker.code_name.clone(),
                            id: Some(maker.id.clone()),
                        });
                    });
                    makers_state.set(makers);

                    let mut workers = vec![];
                    fetched_items.workers.iter().for_each(|worker| {
                        workers.push(NameOptionIdPair {
                            name: worker.name.clone(),
                            id: Some(worker.id.clone()),
                        });
                    });
                    workers_state.set(workers);
                    fetching_state.set(false);
                });
                || ()
            },
            (),
        );
    }

    let title_onchange = {
        let title_state = title_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val: String = input.value();
            let name: String = input.name();
            let name = name.as_str();
            // nameをenumに変換する
            let name = match name {
                "title" => TitleInfo::Title,
                "release_date" => TitleInfo::ReleaseDate,
                "delivery_date" => TitleInfo::DeliveryDate,
                "list_submission_date" => TitleInfo::ListSubmissionDate,
                "reservation_start_date" => TitleInfo::ReservationStartDate,
                "reservation_deadline" => TitleInfo::ReservationDeadline,
                "order_date_to_maker" => TitleInfo::OrderDateToMaker,
                "project_type" => TitleInfo::ProjectType,
                "catalog_status" => TitleInfo::CatalogStatus,
                "announcement_status" => TitleInfo::AnnouncementStatus,
                "remarks" => TitleInfo::Remarks,
                _ => {
                    panic!("nameがTitleInfoのどれともmatchしない");
                }
            };

            let title_state = title_state.clone();
            let original_title_state = TitleState {
                id: title_state.id.clone(),
                title: title_state.title.clone(),
                release_date: title_state.release_date.clone(),
                delivery_date: title_state.delivery_date.clone(),
                list_submission_date: title_state.list_submission_date.clone(),
                reservation_start_date: title_state.reservation_start_date.clone(),
                reservation_deadline: title_state.reservation_deadline.clone(),
                order_date_to_maker: title_state.order_date_to_maker.clone(),
                updated_at: title_state.updated_at.clone(),
                project_type: title_state.project_type.clone(),
                catalog_status: title_state.catalog_status.clone(),
                announcement_status: title_state.announcement_status.clone(),
                remarks: title_state.remarks.clone(),
            };

            match name {
                TitleInfo::Title => {
                    title_state.set(TitleState {
                        title: val,
                        ..original_title_state
                    });
                }
                TitleInfo::ReleaseDate => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        release_date: val,
                        ..original_title_state
                    });
                }
                TitleInfo::DeliveryDate => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        delivery_date: val,
                        ..original_title_state
                    });
                }
                TitleInfo::ListSubmissionDate => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        list_submission_date: val,
                        ..original_title_state
                    });
                }
                TitleInfo::ReservationStartDate => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        reservation_start_date: val,
                        ..original_title_state
                    });
                }
                TitleInfo::ReservationDeadline => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        reservation_deadline: val,
                        ..original_title_state
                    });
                }
                TitleInfo::OrderDateToMaker => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        order_date_to_maker: val,
                        ..original_title_state
                    });
                }
                TitleInfo::ProjectType => {
                    title_state.set(TitleState {
                        project_type: val,
                        ..original_title_state
                    });
                }
                TitleInfo::CatalogStatus => {
                    title_state.set(TitleState {
                        catalog_status: val,
                        ..original_title_state
                    });
                }
                TitleInfo::AnnouncementStatus => {
                    title_state.set(TitleState {
                        announcement_status: val,
                        ..original_title_state
                    });
                }
                TitleInfo::Remarks => {
                    let val = if val == "" { None } else { Some(val) };
                    title_state.set(TitleState {
                        remarks: val,
                        ..original_title_state
                    });
                }
            }
        })
    };

    let item_add_click = {
        let items_state = items_state.clone();
        Callback::from(move |_| {
            let mut new_items = vec![];
            let id = Uuid::new_v4().to_string();
            for item_state in items_state.iter() {
                new_items.push(ItemState {
                    id: item_state.id.clone(),
                    name: item_state.name.clone(),
                    product_code: item_state.product_code.clone(),
                    sku: item_state.sku,
                    illust_status: item_state.illust_status.clone(),
                    pic_illust_id: item_state.pic_illust_id.clone(),
                    design_status: item_state.design_status.clone(),
                    pic_design_id: item_state.pic_design_id.clone(),
                    maker_id: item_state.maker_id.clone(),
                    retail_price: item_state.retail_price,
                    resubmission: item_state.resubmission,
                    line: item_state.line.clone(),
                    is_saved: item_state.is_saved,
                });
            }
            let new_item = ItemState {
                id,
                line: "未対応".to_string(),
                ..Default::default()
            };
            new_items.push(new_item);
            items_state.set(new_items);
        })
    };

    let save_onclick = {
        let items_state = items_state.clone();
        let title_state = title_state.clone();
        Callback::from(move |_| {
            let mut saved_items = vec![];
            web_sys::console::log_1(&"save_onclick".into());
            items_state.iter().for_each(|item_state| {
                saved_items.push(ItemRegisterParams {
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
                    resubmission: item_state.resubmission,
                    line: item_state.line.clone(),
                });
            });
            let title_state = title_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let post_url = format!("{}", backend_url() + "/api/item_new",);
                let client = Request::post(&post_url)
                    .header("Content-Type", "application/json")
                    .body(
                        serde_json::to_string(&RequestPutTitleInfo {
                            release_date: date_string_to_iso_string(
                                title_state.release_date.clone(),
                            ),
                            delivery_date: date_string_to_iso_string(
                                title_state.delivery_date.clone(),
                            ),
                            list_submission_date: date_string_to_iso_string(
                                title_state.list_submission_date.clone(),
                            ),
                            reservation_start_date: date_string_to_iso_string(
                                title_state.reservation_start_date.clone(),
                            ),
                            reservation_deadline: date_string_to_iso_string(
                                title_state.reservation_deadline.clone(),
                            ),
                            order_date_to_maker: date_string_to_iso_string(
                                title_state.order_date_to_maker.clone(),
                            ),
                            title_id: title_state.id.clone(),
                            title_name: title_state.title.clone(),
                            project_type: title_state.project_type.clone(),
                            items: saved_items,
                            catalog_status: title_state.catalog_status.clone(),
                            announcement_status: title_state.announcement_status.clone(),
                            remarks: title_state.remarks.clone(),
                        })
                        .unwrap(),
                    );
                let post_response = client.send().await.expect("Failed to update maker");
                if post_response.status() == 201 {
                    let release_date = title_state.release_date.clone();
                    match release_date {
                        Some(date) => {
                            // 2022-05-03... → 202205
                            let date = date.replace("-", "")[0..6].to_string();
                            web_sys::window()
                                .unwrap()
                                .location()
                                .set_href(&format!("/item_list?yyyymm={}", date))
                                .unwrap()
                        }
                        None => web_sys::window()
                            .unwrap()
                            .location()
                            .set_href("/item_list")
                            .unwrap(),
                    };
                } else {
                    // responseが200以外の場合はエラーを出す
                    let error_message =
                        format!("Failed to create title: {}", post_response.status());
                    web_sys::window()
                        .unwrap()
                        .alert_with_message(&error_message)
                        .unwrap();
                    let error_message = JsValue::from_str(&error_message);
                    web_sys::console::error_1(&error_message);
                }
            });
        })
    };

    html! {
      <div class="new-item-page">
        <h1>{ "New Items" }</h1>
        <HeaderLink current_page={"item_new".to_string()} />
        {
            if *fetching_state {
                html! {
                    <p>{ "Loading..." }</p>
                }
            } else {
                let mut makers = vec![];
                makers_state.iter().for_each(|maker| {
                    makers.push(NameOptionIdPair {
                        name: maker.name.clone(),
                        id: maker.id.clone(),
                    });
                });
                let mut workers = vec![];
                workers_state.iter().for_each(|worker| {
                    workers.push(NameOptionIdPair {
                        name: worker.name.clone(),
                        id: worker.id.clone(),
                    });
                });
                let release_date = parse_date(&title_state.release_date);
                let delivery_date = parse_date(&title_state.delivery_date);
                let list_submission_date = parse_date(&title_state.list_submission_date);
                let reservation_start_date = parse_date(&title_state.reservation_start_date);
                let reservation_deadline = parse_date(&title_state.reservation_deadline);
                let order_date_to_maker = parse_date(&title_state.order_date_to_maker);
                let items = items_state.clone();
                let mut index: usize = 0;
                html! {
                    <>
                        { "発売日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        { "納品日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="delivery_date" name="delivery_date" value={delivery_date} />
                        <br/>
                        { "リスト提出日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="list_submission_date" name="list_submission_date" value={list_submission_date} />
                        <br/>
                        { "案内日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_start_date" name="reservation_start_date" value={reservation_start_date} />
                        <br/>
                        { "締切日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_deadline" name="reservation_deadline" value={reservation_deadline} />
                        <br/>
                        { "発注日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="order_date_to_maker" name="order_date_to_maker" value={order_date_to_maker} />
                        <br/>
                        { "タイトル：" }<TextBox onchange={title_onchange.clone()} input_type="text" placeholder="title" id="title" name="title" value={title_state.title.clone()} />
                        <br/>
                        { "案件：" }<SelectBox onchange={title_onchange.clone()} id="project_type" name="project_type" value={title_state.project_type.clone()} select_list={project_type_list()} />
                        <br/>
                        {
                            html! {
                                <div class="item-detail-wrapper">
                                {
                                    items.iter().map(|item| {
                                        index += 1;
                                        html! {
                                            <ItemDetail
                                                items_state_handle={items_state.clone()}
                                                item_info={item.clone()}
                                                index={index}
                                                workers={workers.clone()}
                                                makers={makers.clone()}
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
                        { "カタログ：" }<SelectBox onchange={title_onchange.clone()} id="catalog_status" name="catalog_status" value={title_state.catalog_status.clone()} select_list={catalog_status_list()} />
                        <br/>
                        { "告知：" }<SelectBox onchange={title_onchange.clone()} id="announcement_status" name="announcement_status" value={title_state.announcement_status.clone()} select_list={announce_status_list()} />
                        <br/>
                        { "備考：" }<TextBox onchange={title_onchange.clone()} input_type="text" placeholder="備考" id="remarks" name="remarks" value={title_state.remarks.clone().unwrap_or("".to_string())} />
                        <br/>
                        {
                            html! {
                                <button onclick={save_onclick} class="save-button" id="btn_submit">
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
