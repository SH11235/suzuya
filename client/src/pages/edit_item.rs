use crate::components::input::Input;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct EditItemPageProperty {
    pub title: String,
}

#[function_component(EditItem)]
pub fn edit_item(props: &EditItemPageProperty) -> Html {
    html! {
      <div>
        <h1>{ "Edit Items" }</h1>
        { props.title.clone() } // temporary
        <br/>
        { "発売日：" }<Input input_type="text" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value="" />
      </div>
    }
}
