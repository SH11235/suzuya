use yew::{events::Event, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectBoxProperty {
    pub onchange: Callback<Event>,
    pub id: String,
    pub name: String,
    pub select_list: Vec<String>,
    pub value: String,
}

#[function_component(SelectBox)]
pub fn select_box(props: &SelectBoxProperty) -> Html {
    let mut first_item = true;
    html! {
        <select class="item-edit-input"
            onchange = { props.onchange.clone() }
            id={ props.id.clone() } name={ props.name.clone() } >
        {
            props.select_list.clone().into_iter().map(|option| {
                if first_item && props.value.clone() == "".to_string() {
                    first_item = false;
                    html! {
                        <option value={ option.clone() } selected=true>{ option.clone() }</option>
                    }
                } else if props.value.clone() == option {
                    first_item = false;
                    html! {
                        <option value={ option.clone() } selected=true>{ option.clone() }</option>
                    }
                } else {
                    first_item = false;
                    html! {
                        <option value={ option.clone() }>{ option.clone() }</option>
                    }
                }
            }).collect::<Html>()
        }
        </select>
    }
}
