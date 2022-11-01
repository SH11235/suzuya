use crate::components::item::monthly_field::MonthlyField;
use crate::model::item_page::{ItemListResponse, YearMonthState};
use crate::common::api::backend_url;
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, Html,
    Properties, TargetCast,
};

#[function_component(ItemList)]
pub fn item_list() -> Html {
    // "/api/item"
    let year_month_list_state = use_state(|| vec![]);
    let get_url = format!("{}{}", backend_url(), "/api/item_list");

    {
        let year_month_list_state = year_month_list_state.clone();
        use_effect_with_deps(
            move |_| {
                // let items_info_state = items_info_state.clone();
                let year_month_list_state = year_month_list_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let fetched_items: ItemListResponse = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    let mut year_month_list = vec![YearMonthState {
                        yyyymm: "発売日未定".to_string(),
                        year: "".to_string(),
                        month: "".to_string(),
                        is_selected: true,
                    }];
                    fetched_items.year_month_list.iter().for_each(|year_month| {
                        year_month_list.push(YearMonthState {
                            yyyymm: year_month.yyyymm.clone(),
                            year: year_month.year.clone(),
                            month: year_month.month.clone(),
                            is_selected: false,
                        });
                    });
                    year_month_list_state.set(year_month_list);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="item-list-page">
            <h1>{ "Suzuya ItemList" }</h1>
            <h2>{ "Monthly filter" }</h2>
            <MonthlyField year_month_list_state_handle={year_month_list_state.clone()}/>
        </div>
    }
}
