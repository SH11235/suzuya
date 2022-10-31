use crate::components::common::select_box::SelectBox;
use crate::components::common::text_box::TextBox;
use crate::components::item::item_detail::ItemDetail;
use crate::model::common::NameOptionIdPair;
use crate::model::item_page::{ItemListResponse, YearMonthListState, YearMonthState};
use crate::settings::api::backend_url;
use reqwasm::http::Request;
use urlencoding::decode;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_effect_with_deps, use_state, Callback, Html,
    Properties, TargetCast,
};

#[function_component(ItemList)]
pub fn item_list() -> Html {
    // "/api/item"
    // let items_info_state = use_state(|| vec![]);
    let year_month_list_state = use_state(|| vec![]);
    let get_url = format!("{}{}", backend_url(), "/api/item_list");

    {
        // let items_info_state = items_info_state.clone();
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
                    // console.log!("fetched_items: {:?}", fetched_items);
                    web_sys::console::log_1(&JsValue::from_str(&format!(
                        "fetched_items: {:?}",
                        fetched_items
                    )));
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

    let year_month_onchange = {
        let year_month_list_state = year_month_list_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val: String = input.value();
            let name: String = input.name();
            let index = name.as_str().split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            let mut original_year_month_list = vec![];
            year_month_list_state.iter().for_each(|year_month| {
                original_year_month_list.push(YearMonthState {
                    yyyymm: year_month.yyyymm.clone(),
                    year: year_month.year.clone(),
                    month: year_month.month.clone(),
                    is_selected: false,
                });
            });
            original_year_month_list[index - 1].is_selected = true;
            year_month_list_state.set(original_year_month_list);
        })
    };
    let mut index: usize = 0;
    html! {
        <div class="item-list-page">
            <h1>{ "Suzuya ItemList" }</h1>
            <h2>{ "Monthly filter" }</h2>
            <fieldset>
                {
                    year_month_list_state.iter().map(|year_month| 
                        {
                            index += 1;
                            html! {
                                <>
                                    <input onchange={ year_month_onchange.clone() } id={ year_month.yyyymm.clone() } class="radio-input" type="radio"
                                        name={ format!("radio-{}", index) } value={ year_month.yyyymm.clone() } checked={year_month.is_selected} />
                                    <label class="radio-label" for={ year_month.yyyymm.clone() }>
                                        { year_month.yyyymm.clone() }
                                    </label>
                                </>
                            }
                        }
                    ).collect::<Html>()
                }
            </fieldset>
        </div>
    }
}
