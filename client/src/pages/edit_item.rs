use crate::components::input::Input;
use yew::{function_component, html};

#[function_component(EditItem)]
pub fn edit_item() -> Html {
    html! {
      <div>
        <h1>{ "Edit Items" }</h1>
        <div>
            { "Inputテスト：" } <Input/>
        <br/>
        </div>
      </div>
    }
}
