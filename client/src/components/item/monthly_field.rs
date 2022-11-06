use crate::model::item_page::{YearMonthState, YearMonthTitleList};
use web_sys::{HtmlInputElement, MouseEvent};
use yew::UseStateHandle;
use yew::{function_component, html, Callback, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct MonthlyFieldProperty {
    pub year_month_list_state_handle: UseStateHandle<YearMonthState>,
    pub items_state_hanle: UseStateHandle<Vec<YearMonthTitleList>>,
}

#[function_component(MonthlyField)]
pub fn delete_button(props: &MonthlyFieldProperty) -> Html {
    let year_month_list_state = props.year_month_list_state_handle.clone();
    let items_state = props.items_state_hanle.clone();
    let year_month_onclick = {
        let year_month_list_state = year_month_list_state.clone();
        Callback::from(move |e: MouseEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let name: String = input.name();
            let index = name.as_str().split("-").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let selected_yyyymm = &year_month_list_state.year_month_list[index - 1].yyyymm;
            let selectd_item = items_state
                .iter()
                .filter(|year_month_title_list| {
                    year_month_title_list.yyyymm == selected_yyyymm.to_string()
                })
                .collect::<Vec<&YearMonthTitleList>>();
            let title_count = selectd_item[0].title_count;
            let item_count = selectd_item[0].item_count;
            year_month_list_state.set(YearMonthState {
                year_month_list: year_month_list_state.year_month_list.clone(),
                selected_yyyymm: selected_yyyymm.to_string(),
                title_count,
                item_count,
            });
        })
    };
    let mut index: usize = 0;
    html! {
        <fieldset>
            {
                year_month_list_state.year_month_list.iter().map(|year_month|
                    {
                        index += 1;
                        let is_selected = year_month_list_state.selected_yyyymm == year_month.yyyymm;
                        html! {
                            <>
                                <input onclick={ year_month_onclick.clone() } id={ year_month.yyyymm.clone() } class="radio-input" type="radio"
                                    name={ format!("radio-{}", index) } value={ year_month.yyyymm.clone() } checked={is_selected} />
                                <label class="radio-label" for={ year_month.yyyymm.clone() }>
                                    { year_month.yyyymm.clone() }
                                </label>
                            </>
                        }
                    }
                ).collect::<Html>()
            }
        </fieldset>
    }
}
