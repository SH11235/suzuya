use crate::components::common::select_box::SelectBox;
use crate::components::common::select_user_maker::SelectUserMaker;
use crate::components::common::text_box::TextBox;
use crate::model::edit_item::{GetItem, NameIdPair};
use crate::settings::select::{design_status_list, illust_status_list};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, Callback, Html, Properties, TargetCast, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct ItemDetailProperty {
    pub id: i32,
    pub get_item: UseStateHandle<GetItem>,
    pub index: usize,
    pub item_name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<i32>,
    pub design_status: String,
    pub pic_design_id: Option<i32>,
    pub maker_id: Option<i32>,
    pub retail_price: Option<i32>,
    pub double_check_person_id: Option<i32>,
}

#[function_component(ItemDetail)]
pub fn item_detail(props: &ItemDetailProperty) -> Html {
    let sku = props.sku.clone().unwrap_or(0);
    let sku = if sku == 0 {
        "".to_string()
    } else {
        sku.to_string()
    };

    let users = props.get_item.users.clone();
    let user_name_id_vec: Vec<NameIdPair> = users
        .into_iter()
        .map(|user| NameIdPair {
            name: user.name.clone(),
            id: user.id.clone(),
        })
        .collect();

    let makers = props.get_item.makers.clone();
    let maker_name_id_vec: Vec<NameIdPair> = makers
        .into_iter()
        .map(|maker| NameIdPair {
            name: maker.code_name.clone(),
            id: maker.id.clone(),
        })
        .collect();

        let retail_price = props.retail_price.clone().unwrap_or(0);
        let retail_price = if retail_price == 0 {
            "".to_string()
        } else {
            retail_price.to_string()
        };

    let onchange = {
        let get_item = props.get_item.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val: String = input.value();
            let name_index: String = input.name();
            let name_index = name_index.as_str().split("-").collect::<Vec<&str>>();
            let name = name_index[0];
            let index: usize = name_index[1].parse().unwrap();

            let mut items = get_item.items.clone();
            let users = get_item.users.clone();
            let makers = get_item.makers.clone();

            match name {
                "name" => {
                    items[index - 1].name = val;
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "product_code" => {
                    items[index - 1].product_code = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "sku" => {
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].sku = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "illust_status" => {
                    items[index - 1].illust_status = val;
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "pic_illust" => {
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].pic_illust_id = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "design_status" => {
                    items[index - 1].design_status = val;
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "pic_design" => {
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].pic_design_id = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "maker_code" => {
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].maker_id = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "retail_price" => {
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].retail_price = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "double_check_person" => {
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].double_check_person_id = Some(val);
                    get_item.set(GetItem {
                        items,
                        users,
                        makers,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date_to_maker: get_item.order_date_to_maker.clone(),
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

    html! {
        <div class="item-wrapper js-item">
            <div style="display: none;" class="input-warpper">{"id"}{ props.index }
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="id" id={ format!("{}-{}", "id", props.index) }
                    name={format!("{}-{}", "id", props.index) } value={ props.id.clone().to_string() } />
            </div>
            <div class="input-warpper">{"アイテム"}{ props.index }
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="アイテム名" id={ format!("{}-{}", "name", props.index) }
                    name={format!("{}-{}", "name", props.index) } value={ props.item_name.clone() } />
            </div>
            <div class="input-warpper">{"品番"}
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="品番" id={ format!("{}-{}", "product_code", props.index) }
                    name={format!("{}-{}", "product_code", props.index) } value={ props.product_code.clone().unwrap_or("".to_string()) } />
            </div>
            <div class="input-warpper">{"SKU"}
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="SKU" id={ format!("{}-{}", "sku", props.index) }
                    name={format!("{}-{}", "sku", props.index) } value={ sku } />
            </div>
            <div class="input-warpper">{"イラストステータス："}
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "illust_status", props.index)} name={ format!("{}-{}", "illust_status", props.index)}
                    value={props.illust_status.clone()} select_list={illust_status_list()}/>
            </div>
            <div class="input-warpper">{"イラスト担当者"}
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "pic_illust", props.index)} name={ format!("{}-{}", "pic_illust", props.index)}
                    value={props.pic_illust_id.clone()} name_value_list={user_name_id_vec.clone()}/>
            </div>
            <div class="input-warpper">{"デザインステータス"}
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "design_status", props.index)} name={ format!("{}-{}", "design_status", props.index)}
                    value={props.design_status.clone()} select_list={design_status_list()}/>
            </div>
            <div class="input-warpper">{"デザイン担当者"}
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "pic_design", props.index)} name={ format!("{}-{}", "pic_design", props.index)}
                    value={props.pic_design_id.clone()} name_value_list={user_name_id_vec.clone()}/>
            </div>
            <div class="input-warpper">{"メーカー"}
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "maker_code", props.index)} name={ format!("{}-{}", "maker_code", props.index)}
                    value={props.maker_id.clone()} name_value_list={maker_name_id_vec.clone()}/>
            </div>
            <div class="input-warpper">{"上代"}
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="上代" id={ format!("{}-{}", "retail_price", props.index) }
                    name={format!("{}-{}", "retail_price", props.index) } value={ retail_price } />
            </div>
            <div class="input-warpper">{"ダブルチェック"}
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "double_check_person", props.index)} name={ format!("{}-{}", "double_check_person", props.index)}
                    value={props.double_check_person_id.clone()} name_value_list={user_name_id_vec.clone()}/>
            </div>
        </div>

    }
}
