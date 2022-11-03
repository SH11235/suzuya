use crate::common::api::backend_url;
use crate::common::date_util::date_time_with_timezone_to_string;
use crate::components::item::monthly_field::MonthlyField;
use crate::model::item_page::YearMonth;
use crate::model::item_page::{ItemListResponse, YearMonthState};
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
    let items_state = use_state(|| vec![]);
    let year_month_list_state = use_state(|| YearMonthState {
        year_month_list: vec![],
        selected_yyymm: "".to_string(),
    });
    let get_url = format!("{}{}", backend_url(), "/api/item_list");

    {
        let items_state = items_state.clone();
        let year_month_list_state = year_month_list_state.clone();
        use_effect_with_deps(
            move |_| {
                let items_state = items_state.clone();
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
                    let mut year_month_list = vec![YearMonth {
                        yyyymm: "発売日未定".to_string(),
                        year: "".to_string(),
                        month: "".to_string(),
                    }];
                    fetched_items.year_month_list.iter().for_each(|year_month| {
                        year_month_list.push(YearMonth {
                            yyyymm: year_month.yyyymm.clone(),
                            year: year_month.year.clone(),
                            month: year_month.month.clone(),
                        });
                    });
                    year_month_list_state.set(YearMonthState {
                        year_month_list,
                        selected_yyymm: "発売日未定".to_string(),
                    });
                    items_state.set(fetched_items.year_month_title_list);
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
            <h2>{ "ItemList" }</h2>
            <table border="3">
                <thead>
                    <tr>
                        <th>{ "発売月" }</th>
                        <th>{ "案内" }</th>
                        <th>{ "〆切" }</th>
                        <th>{ "発注" }</th>
                        <th>{ "編集" }</th>
                        <th>{ "タイトル" }</th>
                        <th>{ "アイテム" }</th>
                        <th>{ "品番" }</th>
                        <th>{ "SKU" }</th>
                        <th>{ "イラストステータス" }</th>
                        <th>{ "担当者" }</th>
                        <th>{ "デザインステータス" }</th>
                        <th>{ "担当者" }</th>
                        <th>{ "工場" }</th>
                        <th>{ "上代" }</th>
                        <th>{ "ダブルチェック" }</th>
                        <th>{ "カタログ" }</th>
                        <th>{ "告知物" }</th>
                        <th>{ "備考" }</th>
                    </tr>
                </thead>
                {
                    items_state.iter().map(|year_month_title| {
                        html!{
                            year_month_title.title_list.iter().map(|title| {
                                let mut index = 0;
                                let date_column_rowspan = title.items.len();
                                let date_column_rowspan = if date_column_rowspan == 0 {
                                    1
                                } else {
                                    date_column_rowspan
                                };
                                let is_display = if year_month_list_state.selected_yyymm == year_month_title.yyyymm {
                                    ""
                                } else {
                                    "none"
                                };
                                let display_style = format!("display: {};", is_display);
                                if title.items.len() == 0 {
                                    html! {
                                        <tr style={display_style.clone()}>
                                            <td rowspan={date_column_rowspan.to_string()}>
                                                { date_time_with_timezone_to_string(&title.release_date) }
                                            </td>
                                            <td rowspan={date_column_rowspan.to_string()}>
                                                { date_time_with_timezone_to_string(&title.reservation_start_date) }
                                            </td>
                                            <td rowspan={date_column_rowspan.to_string()}>
                                                { date_time_with_timezone_to_string(&title.reservation_deadline) }
                                            </td>
                                            <td rowspan={date_column_rowspan.to_string()}>
                                                { date_time_with_timezone_to_string(&title.order_date_to_maker) }
                                            </td>
                                            <td rowspan={date_column_rowspan.to_string()}>
                                                <a href={format!("/item_edit/{}", &title.id)}>{ "編集"}</a>
                                            </td>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                            <td/>
                                        </tr>
                                    }
                                } else {
                                    title.items.iter().map(|item| {
                                        index += 1;
                                        html! {
                                            <tr style={display_style.clone()}>
                                            {
                                                if index == 1 {
                                                    html! {
                                                        <>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            { date_time_with_timezone_to_string(&title.release_date) }
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            { date_time_with_timezone_to_string(&title.reservation_start_date) }
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            { date_time_with_timezone_to_string(&title.reservation_deadline) }
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            { date_time_with_timezone_to_string(&title.order_date_to_maker) }
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            <a href={format!("/item_edit/{}", &title.id)}>{ "編集"}</a>
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            {&title.name}
                                                        </td>
                                                        <td>
                                                            {&item.name}
                                                        </td>
                                                        <td>
                                                            {item.product_code.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {&item.sku.unwrap_or(0)}
                                                        </td>
                                                        <td>
                                                            {&item.illust_status}
                                                        </td>
                                                        <td>
                                                            {item.pic_illust.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {&item.design_status}
                                                        </td>
                                                        <td>
                                                            {item.pic_design.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.maker_code.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.retail_price.clone().unwrap_or(0)}
                                                        </td>
                                                        <td>
                                                            {item.double_check_person.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            {&title.catalog_status}
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            {&title.announcement_status}
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            {title.remarks.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        </>
                                                    }
                                                } else {
                                                    html! {
                                                        <>
                                                        <td>
                                                            {&item.name}
                                                        </td>
                                                        <td>
                                                            {item.product_code.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {&item.sku.unwrap_or(0)}
                                                        </td>
                                                        <td>
                                                            {&item.illust_status}
                                                        </td>
                                                        <td>
                                                            {item.pic_illust.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {&item.design_status}
                                                        </td>
                                                        <td>
                                                            {item.pic_design.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.maker_code.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.retail_price.clone().unwrap_or(0)}
                                                        </td>
                                                        <td>
                                                            {item.double_check_person.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        </>
                                                    }
                                                }
                                            }
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            }).collect::<Html>()
                        }
                    }).collect::<Html>()
                }
            </table>
        </div>
    }
}
