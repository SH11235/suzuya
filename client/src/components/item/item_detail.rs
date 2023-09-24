use crate::common::select::{
    design_status_list, illust_status_list, line_list, resubmission_list, RESUBMISSION_OK,
};
use crate::components::common::select_box::SelectBox;
use crate::components::common::select_worker_maker::SelectUserMaker;
use crate::components::common::text_box::TextBox;
use crate::components::item::item_delete_button::DeleteButton;
use crate::model::common::NameOptionIdPair;
use crate::model::item_page::{ItemInfo, ItemState};
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, Callback, Properties, TargetCast, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct ItemDetailProperty {
    pub items_state_handle: UseStateHandle<Vec<ItemState>>,
    pub item_info: ItemState,
    pub index: usize,
    pub workers: Vec<NameOptionIdPair>,
    pub makers: Vec<NameOptionIdPair>,
}

#[function_component(ItemDetail)]
pub fn item_detail(props: &ItemDetailProperty) -> Html {
    let items_state = props.items_state_handle.clone();

    let workers = props.workers.clone();
    let mut worker_list = vec![NameOptionIdPair {
        name: "未定".to_string(),
        id: Some("null".to_string()),
    }];
    worker_list.extend(workers);

    let makers = props.makers.clone();
    let mut maker_list = vec![NameOptionIdPair {
        name: "未定".to_string(),
        id: Some("null".to_string()),
    }];
    maker_list.extend(makers);

    let sku = props.item_info.sku.clone().unwrap_or(0);
    let sku = if sku == 0 {
        "".to_string()
    } else {
        sku.to_string()
    };
    let retail_price = props.item_info.retail_price.clone().unwrap_or(0);
    let retail_price = if retail_price == 0 {
        "".to_string()
    } else {
        retail_price.to_string()
    };
    let resubmission = if props.item_info.resubmission {
        RESUBMISSION_OK
    } else {
        ""
    };

    let onchange = {
        let items_state = props.items_state_handle.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val: String = input.value();
            let name_index: String = input.name();
            let name_index = name_index.as_str().split("-").collect::<Vec<&str>>();
            let name = name_index[0];
            let name = match name {
                "name" => ItemInfo::Name,
                "product_code" => ItemInfo::ProductCode,
                "sku" => ItemInfo::Sku,
                "illust_status" => ItemInfo::IllustStatus,
                "pic_illust" => ItemInfo::PicIllustId,
                "design_status" => ItemInfo::DesignStatus,
                "pic_design" => ItemInfo::PicDesignId,
                "maker_code" => ItemInfo::MakerId,
                "retail_price" => ItemInfo::RetailPrice,
                "resubmission" => ItemInfo::Resubmission,
                "line" => ItemInfo::Line,
                _ => {
                    panic!("Unexpected name: {}", name);
                }
            };
            let index: usize = name_index[1].parse().unwrap();

            let mut original_items = vec![];
            items_state.iter().for_each(|item| {
                original_items.push(ItemState {
                    id: item.id.clone(),
                    name: item.name.clone(),
                    product_code: item.product_code.clone(),
                    sku: item.sku.clone(),
                    illust_status: item.illust_status.clone(),
                    pic_illust_id: item.pic_illust_id.clone(),
                    design_status: item.design_status.clone(),
                    pic_design_id: item.pic_design_id.clone(),
                    maker_id: item.maker_id.clone(),
                    retail_price: item.retail_price.clone(),
                    resubmission: item.resubmission.clone(),
                    line: item.line.clone(),
                    is_saved: item.is_saved.clone(),
                })
            });

            match name {
                ItemInfo::Name => {
                    original_items[index - 1].name = val;
                }
                ItemInfo::ProductCode => {
                    original_items[index - 1].product_code = Some(val);
                }
                ItemInfo::Sku => {
                    original_items[index - 1].sku = Some(val.parse().unwrap());
                }
                ItemInfo::IllustStatus => {
                    original_items[index - 1].illust_status = val;
                }
                ItemInfo::PicIllustId => {
                    original_items[index - 1].pic_illust_id = Some(val);
                }
                ItemInfo::DesignStatus => {
                    original_items[index - 1].design_status = val;
                }
                ItemInfo::PicDesignId => {
                    original_items[index - 1].pic_design_id = Some(val);
                }
                ItemInfo::MakerId => {
                    original_items[index - 1].maker_id = Some(val);
                }
                ItemInfo::RetailPrice => {
                    original_items[index - 1].retail_price = Some(val.parse().unwrap());
                }
                ItemInfo::Resubmission => {
                    original_items[index - 1].resubmission =
                        if val == RESUBMISSION_OK { true } else { false };
                }
                ItemInfo::Line => {
                    original_items[index - 1].line = val.parse().unwrap();
                }
            }
            items_state.set(original_items);
        })
    };

    html! {
        <div class="item-wrapper js-item">
            <DeleteButton input_id={ props.item_info.id.clone() } items_state_handle={ items_state.clone() } />
            <div style="display: none;" class="input-warpper">{"id"}{ props.index }
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="id" id={ format!("{}-{}", "id", props.index) }
                    name={format!("{}-{}", "id", props.index) } value={ props.item_info.id.clone().to_string() } />
            </div>
            <div class="input-warpper">{"アイテム"}{ props.index }<br/>
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="アイテム名" id={ format!("{}-{}", "name", props.index) }
                    name={format!("{}-{}", "name", props.index) } value={ props.item_info.name.clone() } />
            </div>
            <div class="input-warpper">{"管理番号"}<br/>
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="管理番号" id={ format!("{}-{}", "product_code", props.index) }
                    name={format!("{}-{}", "product_code", props.index) } value={ props.item_info.product_code.clone().unwrap_or("".to_string()) } />
            </div>
            <div class="input-warpper">{"SKU"}<br/>
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="SKU" id={ format!("{}-{}", "sku", props.index) }
                    name={format!("{}-{}", "sku", props.index) } value={ sku } />
            </div>
            <div class="input-warpper">{"イラストステータス："}<br/>
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "illust_status", props.index)} name={ format!("{}-{}", "illust_status", props.index)}
                    value={props.item_info.illust_status.clone()} select_list={illust_status_list()}/>
            </div>
            <div class="input-warpper">{"イラスト担当者"}<br/>
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "pic_illust", props.index)} name={ format!("{}-{}", "pic_illust", props.index)}
                    value={props.item_info.pic_illust_id.clone()} name_value_list={worker_list.clone()}/>
            </div>
            <div class="input-warpper">{"デザインステータス"}<br/>
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "design_status", props.index)} name={ format!("{}-{}", "design_status", props.index)}
                    value={props.item_info.design_status.clone()} select_list={design_status_list()}/>
            </div>
            <div class="input-warpper">{"デザイン担当者"}<br/>
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "pic_design", props.index)} name={ format!("{}-{}", "pic_design", props.index)}
                    value={props.item_info.pic_design_id.clone()} name_value_list={worker_list.clone()}/>
            </div>
            <div class="input-warpper">{"メーカー"}<br/>
                <SelectUserMaker onchange={onchange.clone()} id={ format!("{}-{}", "maker_code", props.index)} name={ format!("{}-{}", "maker_code", props.index)}
                    value={props.item_info.maker_id.clone()} name_value_list={maker_list.clone()}/>
            </div>
            <div class="input-warpper">{"上代"}<br/>
                <TextBox onchange={onchange.clone()} input_type="text" placeholder="上代" id={ format!("{}-{}", "retail_price", props.index) }
                    name={format!("{}-{}", "retail_price", props.index) } value={ retail_price } />
            </div>
            <div class="input-warpper">{"再入稿"}<br/>
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "resubmission", props.index)} name={ format!("{}-{}", "resubmission", props.index)}
                        value={resubmission.clone()} select_list={resubmission_list()}/>
            </div>
            <div class="input-warpper">{"ライン"}<br/>
                <SelectBox onchange={onchange.clone()} id={ format!("{}-{}", "line", props.index)} name={ format!("{}-{}", "line", props.index)}
                        value={props.item_info.line.clone()} select_list={line_list()}/>
            </div>
        </div>

    }
}
