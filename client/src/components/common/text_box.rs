use yew::{events::Event, function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct EditItemProperty {
    pub onchange: Callback<Event>,
    pub placeholder: String,
    pub input_type: String,
    pub id: String,
    pub name: String,
    pub value: String,
}

#[function_component(TextBox)]
pub fn input_text(props: &EditItemProperty) -> Html {
    html! {
      <>
        <input class="item-edit-input"
          onchange = { props.onchange.clone() }
          type={ props.input_type.clone() } placeholder={ props.placeholder.clone() }
          name={ props.name.clone() } id={ props.id.clone() } value={ props.value.clone() } />
      </>
    }
}
