use crate::components::common::text_box::TextBox;
use crate::components::maker_delete_button::DeleteButton;
use crate::components::maker_save_button::SaveButton;
use crate::components::maker_add_button::AddButton;
use crate::model::maker_page::MakerState;
use crate::{model::common::MakerModel, settings::api::backend_url};
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, TargetCast,
};

#[function_component(MakerList)]
pub fn home() -> Html {
    let makers_state = use_state(|| vec![]);
    let get_url = format!("{}{}", backend_url(), "/api/maker_list");
    let mut index: usize = 0;
    {
        let makers_state = makers_state.clone();
        use_effect_with_deps(
            move |_| {
                let makers_state = makers_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let mut fetched_items: Vec<MakerModel> = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    fetched_items.sort_by(|a, b| a.code_name.cmp(&b.code_name));
                    let makers = fetched_items
                        .iter()
                        .map(|maker| MakerState {
                            id: maker.id.clone(),
                            code_name: maker.code_name.clone(),
                            is_changed: false,
                            is_saved: true,
                        })
                        .collect::<Vec<MakerState>>();
                    makers_state.set(makers);
                });
                || ()
            },
            (),
        );
    }

    // idが一致する要素のis_changedをtrueにしてsetする
    let text_box_onchange = {
        let makers_state = makers_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let id = input.id();
            let val: String = input.value();
            let mut new_makers = vec![];
            for maker_state in makers_state.iter() {
                if maker_state.id == id {
                    new_makers.push(MakerState {
                        id: maker_state.id.clone(),
                        code_name: val.clone(),
                        is_changed: true,
                        is_saved: maker_state.is_saved,
                    });
                } else {
                    new_makers.push(MakerState {
                        id: maker_state.id.clone(),
                        code_name: maker_state.code_name.clone(),
                        is_changed: maker_state.is_changed,
                        is_saved: maker_state.is_saved,
                    });
                }
            }
            makers_state.set(new_makers);
        })
    };
    html! {
        <div class="maker-list-page">
            <h1>{ "メーカーリスト" }</h1>
            <table>
                <tbody>
                    <tr>
                        <th>{ "連番" }</th>
                        <th>{ "コード" }</th>
                    </tr>
            {
                for makers_state.iter().map(|maker| {
                    index += 1;
                    html! {
                        <tr>
                            <td>{ index }</td>
                            <td>
                                <TextBox onchange={text_box_onchange.clone()} input_type="text" placeholder="id" id={ maker.id.clone() }
                                    name={format!("{}-{}", "index", index) } value={ maker.code_name.clone() } />
                                <SaveButton input_id={ maker.id.clone() } makers_state_handle={ makers_state.clone() } is_changed={ maker.is_changed } is_saved={ maker.is_saved } />
                                <DeleteButton input_id={ maker.id.clone() } makers_state_handle={ makers_state.clone() } />
                            </td>
                        </tr>
                    }
                })
            }
                </tbody>
            </table>
            <AddButton makers_state_handle={ makers_state.clone() } />
        </div>
    }
}
