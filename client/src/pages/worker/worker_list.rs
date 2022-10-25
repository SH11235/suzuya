use crate::{model::common::WorkerModel, settings::api::backend_url};
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with_deps, use_state};

#[function_component(WorkerList)]
pub fn home() -> Html {
    let get_worker_list = use_state(|| vec![]);
    let get_url = format!("{}{}", backend_url(), "/api/worker_list");
    let mut index: usize = 0;
    {
        let get_worker_list = get_worker_list.clone();
        use_effect_with_deps(
            move |_| {
                let get_worker_list = get_worker_list.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let mut fetched_items: Vec<WorkerModel> = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    fetched_items.sort_by(|a, b| a.name.cmp(&b.name));
                    get_worker_list.set(fetched_items);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <div class="worker-list-page">
            <h1>{ "担当者リスト" }</h1>
            <table>
                <tbody>
                    <tr>
                        <th>{ "連番" }</th>
                        <th>{ "名前" }</th>
                    </tr>

            {
                for get_worker_list.iter().map(|worker| {
                    index += 1;
                    html! {
                        <tr>
                            <td>{ index }</td>
                            <td><a href="/">{ &worker.name }</a></td>
                        </tr>
                    }
                })
            }
                </tbody>
            </table>
            <a href="/">
                <input type="button" value="担当者追加" />
            </a>
        </div>
    }
}
