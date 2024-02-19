use crate::common::select::StatusName;
use yew::{events::Event, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectBoxProperty {
    pub onchange: Callback<Event>,
    pub id: String,
    pub name: String,
    pub select_list: Vec<StatusName>,
    pub value: String,
}

#[function_component(SelectBox)]
pub fn select_box(props: &SelectBoxProperty) -> Html {
    html! {
        <select class="select-list"
            onchange = { props.onchange.clone() }
            id={ props.id.clone() } name={ props.name.clone() } >
        {
            props.select_list.clone().into_iter().map(|option| {
                let selected = props.value.clone() == option.name;
                html! {
                    <option class={ option.color.clone().to_string() } value={ option.name.clone() } selected={ selected }>{ option.name.clone() }</option>
                }
            }).collect::<Html>()
        }
        </select>
    }
}
