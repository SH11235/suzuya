use crate::components::common::header_link::HeaderLink;
use crate::components::common::text_box::TextBox;
use crate::components::worker::worker_add_button::AddButton;
use crate::components::worker::worker_delete_button::DeleteButton;
use crate::components::worker::worker_save_button::SaveButton;
use crate::model::worker_page::WorkerState;
use crate::{common::api::backend_url, model::common::WorkerModel};
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, TargetCast,
};

#[function_component(WorkerList)]
pub fn worker_list() -> Html {
    let workers_state = use_state(std::vec::Vec::new);
    let get_url = format!("{}{}", backend_url(), "/api/worker_list");
    let mut index: usize = 0;
    {
        let workers_state = workers_state.clone();
        use_effect_with_deps(
            move |_| {
                let workers_state = workers_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let fetched_items: Vec<WorkerModel> = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    let workers = fetched_items
                        .iter()
                        .map(|worker| WorkerState {
                            id: worker.id.clone(),
                            name: worker.name.clone(),
                            is_changed: false,
                            is_saved: true,
                        })
                        .collect::<Vec<WorkerState>>();
                    workers_state.set(workers);
                });
                || ()
            },
            (),
        );
    }
    // idが一致する要素のis_changedをtrueにしてsetする
    let text_box_onchange = {
        let workers_state = workers_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let id = input.id();
            let val: String = input.value();
            let mut new_workers = vec![];
            for worker_state in workers_state.iter() {
                if worker_state.id == id {
                    new_workers.push(WorkerState {
                        id: worker_state.id.clone(),
                        name: val.clone(),
                        is_changed: true,
                        is_saved: worker_state.is_saved,
                    });
                } else {
                    new_workers.push(WorkerState {
                        id: worker_state.id.clone(),
                        name: worker_state.name.clone(),
                        is_changed: worker_state.is_changed,
                        is_saved: worker_state.is_saved,
                    });
                }
            }
            workers_state.set(new_workers);
        })
    };
    html! {
        <div class="worker-list-page">
            <h1>{ "担当者リスト" }</h1>
            <HeaderLink current_page={"worker_list".to_string()} />
            <table>
                <tbody>
                    <tr>
                        <th>{ "連番" }</th>
                        <th>{ "名前" }</th>
                    </tr>

            {
                for workers_state.iter().map(|worker| {
                    index += 1;
                    html! {
                        <tr>
                            <td>{ index }</td>
                            <td>
                                <TextBox onchange={text_box_onchange.clone()} input_type="text" placeholder="id" id={ worker.id.clone() }
                                    name={format!("{}-{}", "index", index) } value={ worker.name.clone() } />
                                <SaveButton input_id={ worker.id.clone() } workers_state_handle={ workers_state.clone() } is_changed={ worker.is_changed } is_saved={ worker.is_saved } />
                                <DeleteButton input_id={ worker.id.clone() } workers_state_handle={ workers_state.clone() } />
                            </td>
                        </tr>
                    }
                })
            }
                </tbody>
            </table>
            <AddButton workers_state_handle={ workers_state.clone() } />
        </div>
    }
}
