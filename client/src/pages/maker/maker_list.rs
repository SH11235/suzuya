use crate::{model::common::MakerModel, settings::api::backend_url};
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with_deps, use_state};

#[function_component(MakerList)]
pub fn home() -> Html {
    let get_maker_list = use_state(|| vec![]);
    let get_url = format!("{}{}", backend_url(), "/api/maker_list");
    let mut index: usize = 0;
    {
        let get_maker_list = get_maker_list.clone();
        use_effect_with_deps(
            move |_| {
                let get_maker_list = get_maker_list.clone();
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
                    get_maker_list.set(fetched_items);
                });
                || ()
            },
            (),
        );
    }
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
                for get_maker_list.iter().map(|maker| {
                    index += 1;
                    html! {
                        <tr>
                            <td>{ index }</td>
                            <td>{ &maker.code_name }</td>
                        </tr>
                    }
                })
            }
                </tbody>
            </table>
            <a href="/">
                <input type="button" value="メーカー追加" />
            </a>
        </div>
    }
}
