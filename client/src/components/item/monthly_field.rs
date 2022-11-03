use crate::model::item_page::{YearMonth, YearMonthState};
use web_sys::HtmlInputElement;
use yew::UseStateHandle;
use yew::{events::Event, function_component, html, Callback, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct MonthlyFieldProperty {
    pub year_month_list_state_handle: UseStateHandle<YearMonthState>,
}

#[function_component(MonthlyField)]
pub fn delete_button(props: &MonthlyFieldProperty) -> Html {
    let year_month_list_state = props.year_month_list_state_handle.clone();
    let year_month_onchange = {
        let year_month_list_state = year_month_list_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let name: String = input.name();
            let index = name.as_str().split("-").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let mut original_year_month_list = vec![];
            year_month_list_state
                .year_month_list
                .iter()
                .for_each(|year_month| {
                    original_year_month_list.push(YearMonth {
                        yyyymm: year_month.yyyymm.clone(),
                        year: year_month.year.clone(),
                        month: year_month.month.clone(),
                    });
                });
            let selected_yyyymm = original_year_month_list[index - 1].yyyymm.clone();
            year_month_list_state.set(YearMonthState {
                year_month_list: original_year_month_list,
                selected_yyymm: selected_yyyymm,
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
                        let is_selected = year_month_list_state.selected_yyymm == year_month.yyyymm;
                        html! {
                            <>
                                <input onchange={ year_month_onchange.clone() } id={ year_month.yyyymm.clone() } class="radio-input" type="radio"
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
