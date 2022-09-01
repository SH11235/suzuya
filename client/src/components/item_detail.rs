use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::model::edit_item::GetItem;
use crate::settings::select::illust_status_list;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, Callback, Html, Properties, TargetCast, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct ItemDetailProperty {
    pub get_item: UseStateHandle<GetItem>,
    pub index: usize,
    pub item_name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
}

#[function_component(ItemDetail)]
pub fn item_detail(props: &ItemDetailProperty) -> Html {
    let sku = props.sku.clone().unwrap_or(0);
    let sku = if sku == 0 {
        "".to_string()
    } else {
        sku.to_string()
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

            match name {
                "name" => {
                    let mut items = get_item.items.clone();
                    items[index - 1].name = val;
                    get_item.set(GetItem {
                        items: items,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "product_code" => {
                    let mut items = get_item.items.clone();
                    items[index - 1].product_code = Some(val);
                    get_item.set(GetItem {
                        items: items,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "sku" => {
                    let mut items = get_item.items.clone();
                    let val: i32 = val.parse().unwrap();
                    items[index - 1].sku = Some(val);
                    get_item.set(GetItem {
                        items: items,
                        release_date: get_item.release_date.clone(),
                        reservation_start_date: get_item.reservation_start_date.clone(),
                        reservation_deadline: get_item.reservation_deadline.clone(),
                        order_date: get_item.order_date.clone(),
                        last_updated: get_item.last_updated.clone(),
                    });
                }
                "illust_status" => {
                    let mut items = get_item.items.clone();
                    items[index - 1].illust_status = val;
                    get_item.set(GetItem {
                        items: items,
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

    html! {
        <>
            <div>{"アイテム"}{ props.index }
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="アイテム名" id={ format!("{}-{}", "name", props.index) }
                    name={format!("{}-{}", "name", props.index) } value={ props.item_name.clone() } />
            </div>
            <div>{"品番"}
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="品番" id={ format!("{}-{}", "product_code", props.index) }
                    name={format!("{}-{}", "product_code", props.index) } value={ props.product_code.clone().unwrap_or("".to_string()) } />
            </div>
            <div>{"SKU"}
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="SKU" id={ format!("{}-{}", "sku", props.index) }
                    name={format!("{}-{}", "sku", props.index) } value={ sku } />
            </div>
            <div>{"イラストステータス："}
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "illust_status", props.index)} name={ format!("{}-{}", "illust_status", props.index)}
                    value={props.illust_status.clone()} select_list={illust_status_list()}/>
            </div>
        </>
    // <div>担当者
    //     <select class="item-edit-input" name="pic_illust{{ loop.index }}" id="pic_illust{{ loop.index }}">
    //         {% for user in users %}
    //         {% if user.id == item.pic_illust_id %}
    //         <option value="{{ user.id }}" selected>{{ user.name }}</option>
    //         {% else %}
    //         <option value="{{ user.id }}">{{ user.name }}</option>
    //         {% endif %}
    //         {% endfor %}
    //     </select>
    // </div>
    // <div>デザインステータス
    //     <select class="item-edit-input" name="design_status{{ loop.index }}"
    //         id="design_status{{ loop.index }}">
    //         {% for status in design_status_list %}
    //         {% if status.name == item.design_status %}
    //         <option value="{{ status.name }}" selected>{{ status.name }}</option>
    //         {% else %}
    //         <option value="{{ status.name }}">{{ status.name }}</option>
    //         {% endif %}
    //         {% endfor %}
    //     </select>
    // </div>
    // <div>担当者
    //     <select class="item-edit-input" name="pic_design{{ loop.index }}" id="pic_design{{ loop.index }}">
    //         {% for user in users %}
    //         {% if user.id == item.pic_design_id %}
    //         <option value="{{ user.id }}" selected>{{ user.name }}</option>
    //         {% else %}
    //         <option value="{{ user.id }}">{{ user.name }}</option>
    //         {% endif %}
    //         {% endfor %}
    //     </select>
    // </div>
    // <div>メーカー
    //     <select class="item-edit-input" name="maker_code{{ loop.index }}" id="maker_code{{ loop.index }}">
    //         {% for maker in makers %}
    //         {% if maker.id == item.maker_id %}
    //         <option value="{{ maker.id }}" selected>{{ maker.code_name }}</option>
    //         {% else %}
    //         <option value="{{ maker.id }}">{{ maker.code_name }}</option>
    //         {% endif %}
    //         {% endfor %}
    //     </select>
    // </div>
    // <div>上代
    //     <input class="item-edit-input" type="text" placeholder="上代" name="retail_price{{ loop.index }}"
    //         id="retail_price{{ loop.index }}" value="{{ item.retail_price }}" autofocus />
    // </div>
    // <div>ダブルチェック
    //     <select class="item-edit-input" name="double_check_person{{ loop.index }}"
    //         id="double_check_person{{ loop.index }}">
    //         {% for user in users %}
    //         {% if user.id == item.double_check_person_id %}
    //         <option value="{{ user.id }}" selected>{{ user.name }}</option>
    //         {% else %}
    //         <option value="{{ user.id }}">{{ user.name }}</option>
    //         {% endif %}
    //         {% endfor %}
    //     </select>
    // </div>

    }
}
