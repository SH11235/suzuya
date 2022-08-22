use crate::components::common::text_box::TextBox;
use crate::components::common::select_box::SelectBox;
use crate::settings::select::illust_status_list;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ItemDetailProperty {
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

    html! {
        <>
            <div>{"アイテム"}{ props.index }
                <TextBox input_type="text" placeholder="アイテム名" id={ format!("{}{}", "name", props.index) }
                    name={format!("{}{}", "name", props.index) } value={ props.item_name.clone() } />
            </div>
            <div>{"品番"}
                <TextBox input_type="text" placeholder="品番" id={ format!("{}{}", "product_code", props.index) }
                    name={format!("{}{}", "product_code", props.index) } value={ props.product_code.clone().unwrap_or("".to_string()) } />
            </div>
            <div>{"SKU"}
                <TextBox input_type="text" placeholder="SKU" id={ format!("{}{}", "sku", props.index) }
                    name={format!("{}{}", "sku", props.index) } value={ sku } />
            </div>
            <div>{"イラストステータス："}
                <SelectBox id={ format!("{}{}", "illust_status", props.index)} name={ format!("{}{}", "illust_status", props.index)}
                    value={props.illust_status.clone()} select_list={illust_status_list()}/>
            </div>
        </>
    }
}
