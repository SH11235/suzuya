use crate::model::edit_item::NameIdPair;
use yew::{events::Event, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectBoxProperty {
    pub onchange: Callback<Event>,
    pub id: String,
    pub name: String,
    pub name_value_list: Vec<NameIdPair>,
    pub value: Option<i32>,
}

#[function_component(SelectUserMaker)]
pub fn select_user_maker(props: &SelectBoxProperty) -> Html {
    html! {
      <select class="item-edit-input"
        onchange = { props.onchange.clone() }
        id={ props.id.clone() } name={ props.name.clone() } >
        {
          props.name_value_list.clone().into_iter().map(|name_id_pair| {
              if props.value.clone() == Some(name_id_pair.id) {
                  html! {
                      <option value={ name_id_pair.id.clone().to_string() } selected=true>{ name_id_pair.name.clone() }</option>
                  }
              } else {
                  html! {
                      <option value={ name_id_pair.id.clone().to_string() }>{ name_id_pair.name.clone() }</option>
                  }
              }
          }).collect::<Html>()
        }
      </select>
    }
}
