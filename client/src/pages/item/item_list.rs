use crate::common::api::{backend_url, RELEASE_DATE_TEXT};
use crate::common::date_util::date_time_with_timezone_to_string;
use crate::common::select::{get_corresponding_color, RESUBMISSION_NONE, RESUBMISSION_OK};
use crate::components::common::header_link::HeaderLink;
use crate::components::item::monthly_field::MonthlyField;
use crate::model::item_page::{ItemListResponse, YearMonthState};
use reqwasm::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

#[function_component(ItemList)]
pub fn item_list() -> Html {
    // "/api/item"
    let items_state = use_state(|| vec![]);
    let year_month_list_state = use_state(|| YearMonthState {
        year_month_list: vec![],
        selected_yyyymm: "".to_string(),
        title_count: 0,
        item_count: 0,
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
                    let yyyymm_param = web_sys::Url::new(
                        web_sys::window()
                            .unwrap()
                            .location()
                            .href()
                            .unwrap()
                            .as_str(),
                    )
                    .unwrap()
                    .search_params()
                    .get("yyyymm");
                    let selected_yyyymm = match yyyymm_param {
                        Some(yyyymm) => {
                            format!("{}/{}", &yyyymm[0..4], &yyyymm[4..6])
                        }
                        None => RELEASE_DATE_TEXT.to_string(),
                    };
                    let year_month_title_list_index =
                        if fetched_items.year_month_title_list.len() > 0 {
                            fetched_items
                                .year_month_title_list
                                .iter()
                                .position(|x| x.yyyymm == selected_yyyymm)
                        } else {
                            None
                        };
                    year_month_list_state.set(YearMonthState {
                        year_month_list: fetched_items.year_month_list,
                        selected_yyyymm,
                        title_count: if fetched_items.year_month_title_list.len() > 0 {
                            match year_month_title_list_index {
                                Some(index) => {
                                    fetched_items.year_month_title_list[index].title_count
                                }
                                None => 0,
                            }
                        } else {
                            0
                        },
                        item_count: if fetched_items.year_month_title_list.len() > 0 {
                            match year_month_title_list_index {
                                Some(index) => {
                                    fetched_items.year_month_title_list[index].item_count
                                }
                                None => 0,
                            }
                        } else {
                            0
                        },
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
            <HeaderLink current_page={"item_list".to_string()} />
            <h2>{ "Monthly filter" }</h2>
            <MonthlyField year_month_list_state_handle={year_month_list_state.clone()} items_state_hanle={items_state.clone()} />
            <h2>{ format!("タイトル数: {}, アイテム数: {}", year_month_list_state.title_count, year_month_list_state.item_count) }</h2>
            <table border="3">
                <thead>
                    <tr>
                        <th>{ "入荷日" }</th>
                        <th>{ "解禁" }</th>
                        <th>{ "締切" }</th>
                        <th>{ "発注" }</th>
                        <th>{ "最終更新日" }</th>
                        <th>{ "編集" }</th>
                        <th>{ "タイトル" }</th>
                        <th>{ "アイテム" }</th>
                        <th>{ "管理番号" }</th>
                        <th>{ "SKU" }</th>
                        <th>{ "イラストステータス" }</th>
                        <th>{ "担当者" }</th>
                        <th>{ "デザインステータス" }</th>
                        <th>{ "担当者" }</th>
                        <th>{ "工場" }</th>
                        <th>{ "上代" }</th>
                        <th>{ "再入稿" }</th>
                        <th>{ "ダブルチェック" }</th>
                        <th>{ "ライン" }</th>
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
                                let is_display = if year_month_list_state.selected_yyyymm == year_month_title.yyyymm {
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
                                                { date_time_with_timezone_to_string(&Some(title.updated_at.clone())) }
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
                                            <td/>
                                            <td/>
                                        </tr>
                                    }
                                } else {
                                    title.items.iter().map(|item| {
                                        index += 1;
                                        let resubmission = match item.resubmission {
                                            true => RESUBMISSION_OK,
                                            false => RESUBMISSION_NONE,
                                        };
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
                                                            { date_time_with_timezone_to_string(&Some(title.updated_at.clone())) }
                                                        </td>
                                                        <td rowspan={date_column_rowspan.to_string()}>
                                                            <a href={format!("/item_edit/{}", &title.id)}>{ "編集"}</a>
                                                        </td>
                                                        <td class={get_corresponding_color(&title.project_type).to_string()} rowspan={date_column_rowspan.to_string()}>
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
                                                        <td class={get_corresponding_color(&item.illust_status).to_string()}>
                                                            {&item.illust_status}
                                                        </td>
                                                        <td class={get_corresponding_color(&item.illust_status).to_string()}>
                                                            {item.pic_illust.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td class={get_corresponding_color(&item.design_status).to_string()}>
                                                            {&item.design_status}
                                                        </td>
                                                        <td class={get_corresponding_color(&item.design_status).to_string()}>
                                                            {item.pic_design.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.maker_code.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.retail_price.clone().unwrap_or(0)}
                                                        </td>
                                                        <td>
                                                            {resubmission}
                                                        </td>
                                                        <td>
                                                            {item.double_check_person.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.line.clone()}
                                                        </td>
                                                        <td class={get_corresponding_color(&title.catalog_status).to_string()} rowspan={date_column_rowspan.to_string()}>
                                                            {&title.catalog_status}
                                                        </td>
                                                        <td class={get_corresponding_color(&title.announcement_status).to_string()} rowspan={date_column_rowspan.to_string()}>
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
                                                        <td class={get_corresponding_color(&item.illust_status).to_string()}>
                                                            {&item.illust_status}
                                                        </td>
                                                        <td class={get_corresponding_color(&item.illust_status).to_string()}>
                                                            {item.pic_illust.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td class={get_corresponding_color(&item.design_status).to_string()}>
                                                            {&item.design_status}
                                                        </td>
                                                        <td class={get_corresponding_color(&item.design_status).to_string()}>
                                                            {item.pic_design.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.maker_code.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.retail_price.clone().unwrap_or(0)}
                                                        </td>
                                                        <td>
                                                            {resubmission}
                                                        </td>
                                                        <td>
                                                            {item.double_check_person.clone().unwrap_or("".to_string())}
                                                        </td>
                                                        <td>
                                                            {item.line.clone()}
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
