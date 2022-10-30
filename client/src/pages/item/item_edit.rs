use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::components::item::item_detail::ItemDetail;
use crate::model::common::NameOptionIdPair;
use crate::model::item_page::{GetItemInfoByTitleId, ItemState, TitleInfo, TitleState};
use crate::settings::api::backend_url;
use crate::settings::select::{announce_status_list, catalog_status_list, project_type_list};
use reqwasm::http::Request;
use urlencoding::decode;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, Html,
    Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct EditItemPageProperty {
    pub title_id: String,
}

#[function_component(ItemEdit)]
pub fn edit_item(props: &EditItemPageProperty) -> Html {
    let title_id = props.title_id.clone();
    let title_state = use_state(|| TitleState {
        ..Default::default()
    });
    let items_state = use_state(|| vec![]);
    let makers_state = use_state(|| vec![]);
    let workers_state = use_state(|| vec![]);
    let get_url = format!(
        "{}{}",
        backend_url() + "/api/item/",
        decode(&props.title_id.clone()).expect("UTF-8")
    );
    {
        let title_id = title_id.clone();
        let title_state = title_state.clone();
        let items_state = items_state.clone();
        let makers_state = makers_state.clone();
        let workers_state = workers_state.clone();
        use_effect_with_deps(
            move |_| {
                let title_state = title_state.clone();
                let items_state = items_state.clone();
                let makers_state = makers_state.clone();
                let workers_state = workers_state.clone();
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
                    title_state.set(TitleState {
                        id: title_id,
                        release_date: fetched_items.release_date,
                        reservation_start_date: fetched_items.reservation_start_date,
                        reservation_deadline: fetched_items.reservation_deadline,
                        order_date_to_maker: fetched_items.order_date_to_maker,
                        project_type: fetched_items.project_type,
                        catalog_status: fetched_items.catalog_status,
                        announcement_status: fetched_items.announcement_status,
                        remarks: fetched_items.remarks,
                        title: fetched_items.title,
                    });
                    let items = fetched_items
                        .items
                        .iter()
                        .map(|item| ItemState {
                            id: item.id.clone(),
                            name: item.name.clone(),
                            product_code: item.product_code.clone(),
                            sku: item.sku,
                            illust_status: item.illust_status.clone(),
                            pic_illust_id: item.pic_illust_id.clone(),
                            design_status: item.design_status.clone(),
                            pic_design_id: item.pic_design_id.clone(),
                            maker_id: item.maker_id.clone(),
                            retail_price: item.retail_price,
                            double_check_person_id: item.double_check_person_id.clone(),
                            is_saved: true,
                        })
                        .collect::<Vec<ItemState>>();
                    items_state.set(items);

                    let mut makers = vec![];
                    fetched_items.makers.iter().for_each(|maker| {
                        makers.push(NameOptionIdPair {
                            name: maker.code_name.clone(),
                            id: Some(maker.id.clone()),
                        });
                    });
                    makers.sort_by(|a, b| a.name.cmp(&b.name));
                    makers_state.set(makers);

                    let mut workers = vec![];
                    fetched_items.workers.iter().for_each(|worker| {
                        workers.push(NameOptionIdPair {
                            name: worker.name.clone(),
                            id: Some(worker.id.clone()),
                        });
                    });
                    workers.sort_by(|a, b| a.name.cmp(&b.name));
                    workers_state.set(workers);
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
                reservation_start_date: title_state.reservation_start_date.clone(),
                reservation_deadline: title_state.reservation_deadline.clone(),
                order_date_to_maker: title_state.order_date_to_maker.clone(),
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
                    title_state.set(TitleState {
                        release_date: Some(val),
                        ..original_title_state
                    });
                }
                TitleInfo::ReservationStartDate => {
                    title_state.set(TitleState {
                        reservation_start_date: Some(val),
                        ..original_title_state
                    });
                }
                TitleInfo::ReservationDeadline => {
                    title_state.set(TitleState {
                        reservation_deadline: Some(val),
                        ..original_title_state
                    });
                }
                TitleInfo::OrderDateToMaker => {
                    title_state.set(TitleState {
                        order_date_to_maker: Some(val),
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
                    title_state.set(TitleState {
                        remarks: Some(val),
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
                    double_check_person_id: item_state.double_check_person_id.clone(),
                    is_saved: item_state.is_saved,
                });
            }
            let new_item = ItemState {
                ..Default::default()
            };
            new_items.push(new_item);
            items_state.set(new_items);
        })
    };

    html! {
      <div class="edit-item-page">
        <h1>{ "Edit Items" }</h1>
        {
            if items_state.len() == 0 {
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
                let reservation_start_date = parse_date(&title_state.reservation_start_date);
                let reservation_deadline = parse_date(&title_state.reservation_deadline);
                let order_date_to_maker = parse_date(&title_state.order_date_to_maker);
                let items = items_state.clone();
                let mut index: usize = 0;
                html! {
                    <>
                        { "発売日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        { "案内日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_start_date" name="reservation_start_date" value={reservation_start_date} />
                        <br/>
                        { "締切日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="reservation_deadline" name="reservation_deadline" value={reservation_deadline} />
                        <br/>
                        { "発注日：" }<TextBox onchange={title_onchange.clone()} input_type="date" placeholder="yyyy-mm-dd" id="order_date_to_maker" name="order_date_to_maker" value={order_date_to_maker} />
                        <br/>
                        { "タイトル：" }<TextBox onchange={title_onchange.clone()} input_type="text" placeholder="yyyy-mm-dd" id="title" name="title" value={title_state.title.clone()} />
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
                        { "カタログステータス：" }<SelectBox onchange={title_onchange.clone()} id="catalog_status" name="catalog_status" value={title_state.catalog_status.clone()} select_list={catalog_status_list()} />
                        <br/>
                        { "告知：" }<SelectBox onchange={title_onchange.clone()} id="announcement_status" name="announcement_status" value={title_state.announcement_status.clone()} select_list={announce_status_list()} />
                        <br/>
                        { "備考：" }<TextBox onchange={title_onchange.clone()} input_type="text" placeholder="備考" id="remarks" name="remarks" value={title_state.remarks.clone().unwrap_or("".to_string())} />
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
