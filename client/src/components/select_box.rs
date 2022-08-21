use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectBoxProperty {
    pub id: String,
    pub name: String,
    pub select_list: Vec<String>,
    pub value: String,
}

#[function_component(SelectBox)]
pub fn input_text(props: &SelectBoxProperty) -> Html {
    html! {
      <select class="item-edit-input"
        id={ props.id.clone() } name={ props.name.clone() } >
        {
          props.select_list.clone().into_iter().map(|option| {
              if props.value.clone() == option {
                  html! {
                      <option value={ option.clone() } selected=true>{ option.clone() }</option>
                  }
              } else {
                  html! {
                      <option value={ option.clone() }>{ option.clone() }</option>
                  }
              }
          }).collect::<Html>()
        }
      </select>
    }
}
