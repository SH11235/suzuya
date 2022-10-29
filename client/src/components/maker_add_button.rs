use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::model::maker_page::MakerState;

#[derive(Properties, PartialEq)]
pub struct AddButtonProperty {
    pub makers_state_handle: UseStateHandle<Vec<MakerState>>,
}

#[function_component(AddButton)]
pub fn add_button(props: &AddButtonProperty) -> Html {
    let makers_state = props.makers_state_handle.clone();
    let add_onclick = Callback::from(move |_| {
        let makers_state = makers_state.clone();
        let mut new_makers = vec![];
        for maker_state in makers_state.iter() {
            new_makers.push(MakerState {
                id: maker_state.id.clone(),
                code_name: maker_state.code_name.clone(),
                is_changed: maker_state.is_changed,
                is_saved: maker_state.is_saved,
            });
        }
        let length = new_makers.len();
        new_makers.push(MakerState {
            id: format!("new_maker_{}", length),
            code_name: "".to_string(),
            is_changed: true,
            is_saved: false,
        });
        makers_state.set(new_makers);
    });
    html! {
        <button onclick={ add_onclick }>{ "メーカー追加" }</button>
    }
}
