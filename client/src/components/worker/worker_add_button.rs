use yew::{function_component, html, Callback, Properties, UseStateHandle};

use crate::model::worker_page::WorkerState;

#[derive(Properties, PartialEq)]
pub struct AddButtonProperty {
    pub workers_state_handle: UseStateHandle<Vec<WorkerState>>,
}

#[function_component(AddButton)]
pub fn add_button(props: &AddButtonProperty) -> Html {
    let workers_state = props.workers_state_handle.clone();
    let add_onclick = Callback::from(move |_| {
        let workers_state = workers_state.clone();
        let mut new_workers = vec![];
        for worker_state in workers_state.iter() {
            new_workers.push(WorkerState {
                id: worker_state.id.clone(),
                name: worker_state.name.clone(),
                is_changed: worker_state.is_changed,
                is_saved: worker_state.is_saved,
            });
        }
        let length = new_workers.len();
        new_workers.push(WorkerState {
            id: format!("new_worker_{}", length),
            name: "".to_string(),
            is_changed: true,
            is_saved: false,
        });
        workers_state.set(new_workers);
    });
    html! {
        <button onclick={ add_onclick }>{ "担当者追加" }</button>
    }
}
