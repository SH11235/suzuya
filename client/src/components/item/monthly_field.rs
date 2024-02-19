use crate::model::item_page::YearMonth;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::UseStateHandle;
use yew::{function_component, html, Callback, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct MonthlyFieldProperty {
    pub year_month_list_state: UseStateHandle<Vec<YearMonth>>,
    pub selected_yyyymm_state: UseStateHandle<String>,
}

#[function_component(MonthlyField)]
pub fn monthly_field(props: &MonthlyFieldProperty) -> Html {
    let year_month_list_state = props.year_month_list_state.clone();
    let year_month_list: Vec<YearMonth> = year_month_list_state.iter().cloned().collect();
    let selected_yyyymm_state = props.selected_yyyymm_state.clone();
    let year_month_onclick = {
        let selected_yyyymm_state = selected_yyyymm_state.clone();
        let year_month_list = year_month_list.clone();
        Callback::from(move |e: MouseEvent| {
            let year_month_list = year_month_list.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            let name: String = input.name();
            let index = name.as_str().split('-').collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let selected_yyyymm = &year_month_list[index - 1].yyyymm;
            selected_yyyymm_state.set(selected_yyyymm.clone());
        })
    };
    let mut index: usize = 0;
    html! {
        <fieldset>
            {
                year_month_list.clone().iter().map(|year_month|
                    {
                        index += 1;
                        let is_selected = *selected_yyyymm_state == year_month.yyyymm;
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
