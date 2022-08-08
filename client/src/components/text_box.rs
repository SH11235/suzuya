use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct EditItemProperty {
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
          type={ props.input_type.clone() } placeholder={ props.placeholder.clone() }
          name={ props.name.clone() } id={ props.id.clone() } value={ props.value.clone() } />
      </>
    }
}
