use crate::model::common::NameOptionIdPair;
use yew::{events::Event, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectBoxProperty {
    pub onchange: Callback<Event>,
    pub id: String,
    pub name: String,
    pub name_value_list: Vec<NameOptionIdPair>,
    pub value: Option<String>,
}

#[function_component(SelectUserMaker)]
pub fn select_user_maker(props: &SelectBoxProperty) -> Html {
    let mut first_item = true;
    html! {
        <select class="item-edit-input"
            onchange = { props.onchange.clone() }
            id={ props.id.clone() } name={ props.name.clone() } >
        {
            props.name_value_list.clone().into_iter().map(|name_id_pair| {
                if first_item && props.value.clone() == None {
                    first_item = false;
                    html! {
                        <option value={ "" } selected=true>{ name_id_pair.name.clone() }</option>
                    }
                } else if props.value.clone() == name_id_pair.id.clone() {
                    first_item = false;
                    html! {
                        <option value={ name_id_pair.id.clone() } selected=true>{ name_id_pair.name.clone() }</option>
                    }
                } else {
                    first_item = false;
                    html! {
                        <option value={ name_id_pair.id.clone() }>{ name_id_pair.name.clone() }</option>
                    }
                }
            }).collect::<Html>()
        }
        </select>
    }
}
