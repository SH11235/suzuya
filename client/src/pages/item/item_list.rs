use crate::common::api::{backend_url, RELEASE_DATE_TEXT};
use crate::common::date_util::date_time_with_timezone_to_string;
use crate::common::select::{get_corresponding_color, RESUBMISSION_NONE, RESUBMISSION_OK};
use crate::components::common::header_link::HeaderLink;
use crate::components::item::monthly_field::MonthlyField;
use crate::model::item_page::{YearMonth, YearMonthTitleList};
use reqwasm::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

#[function_component(ItemList)]
pub fn item_list() -> Html {
    let title_list_group_by_year_month = use_state(std::vec::Vec::new);
    let selected_yyyymm = use_state(|| "".to_string());
    let selected_titles: YearMonthTitleList =
        match title_list_group_by_year_month.clone().iter().find(
            |year_month_title: &&YearMonthTitleList| year_month_title.yyyymm == *selected_yyyymm,
        ) {
            Some(titles) => titles.clone(),
            None => YearMonthTitleList {
                yyyymm: "".to_string(),
                year: "".to_string(),
                month: "".to_string(),
                title_count: 0,
                item_count: 0,
                title_list: vec![],
            },
        };
    let year_month_list_state = use_state(std::vec::Vec::new);
    let get_url = format!("{}{}", backend_url(), "/api/item_list");

    {
        let title_list_group_by_year_month = title_list_group_by_year_month.clone();
        let selected_yyyymm = selected_yyyymm.clone();
        let year_month_list_state = year_month_list_state.clone();
        use_effect_with_deps(
            move |_| {
                let title_list_group_by_year_month = title_list_group_by_year_month.clone();
                let selected_yyyymm = selected_yyyymm.clone();
                let year_month_list_state = year_month_list_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Request::get(&get_url);
                    let fetched_items: Vec<YearMonthTitleList> = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    // API ResponseをStateにセット
                    let cloned_items = fetched_items.clone();
                    title_list_group_by_year_month.set(cloned_items);
                    // 月別リストをStateにセット
                    year_month_list_state.set(
                        fetched_items
                            .iter()
                            .map(|year_month_title| YearMonth {
                                yyyymm: year_month_title.yyyymm.clone(),
                                year: year_month_title.year.clone(),
                                month: year_month_title.month.clone(),
                            })
                            .collect::<Vec<YearMonth>>(),
                    );
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
                    // URLのyyyymmパラメーターをStateにセット
                    // 無ければデフォルト値をセット
                    let yyyymm_string = match yyyymm_param {
                        Some(yyyymm) => {
                            format!("{}/{}", &yyyymm[0..4], &yyyymm[4..6])
                        }
                        None => RELEASE_DATE_TEXT.to_string(),
                    };
                    selected_yyyymm.set(yyyymm_string);
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
            <MonthlyField year_month_list_state={year_month_list_state.clone()} selected_yyyymm_state={selected_yyyymm.clone()} />
            <h2>{ format!("タイトル数: {}, アイテム数: {}", selected_titles.title_count, selected_titles.item_count) }</h2>
            <table border="3">
                <thead>
                    <tr>
                        <th>{ "入荷日" }</th>
                        <th>{ "納品日" }</th>
                        <th>{ "リスト提出日" }</th>
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
                        <th>{ "ライン" }</th>
                        <th>{ "カタログ" }</th>
                        <th>{ "告知物" }</th>
                        <th>{ "備考" }</th>
                    </tr>
                </thead>
                {
                    selected_titles.title_list.iter().map(|title_with_items| {
                        let mut index = 0;
                        let date_column_rowspan = title_with_items.items.len();
                        let date_column_rowspan = if date_column_rowspan == 0 {
                            1
                        } else {
                            date_column_rowspan
                        };
                        let is_display = if *selected_yyyymm == selected_titles.yyyymm {
                            ""
                        } else {
                            "none"
                        };
                        let display_style = format!("display: {};", is_display);
                        if title_with_items.items.is_empty() {
                            html! {
                                <tr style={display_style.clone()}>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&title_with_items.release_date) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&title_with_items.delivery_date) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&title_with_items.list_submission_date) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&title_with_items.reservation_start_date) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&title_with_items.reservation_deadline) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&title_with_items.order_date_to_maker) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        { date_time_with_timezone_to_string(&Some(title_with_items.updated_at.clone())) }
                                    </td>
                                    <td rowspan={date_column_rowspan.to_string()}>
                                        <a href={format!("/item_edit/{}", &title_with_items.id)}>{ "編集"}</a>
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
                                </tr>
                            }
                        } else {
                            title_with_items.items.iter().map(|item| {
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
                                                    { date_time_with_timezone_to_string(&title_with_items.release_date) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    { date_time_with_timezone_to_string(&title_with_items.delivery_date) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    { date_time_with_timezone_to_string(&title_with_items.list_submission_date) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    { date_time_with_timezone_to_string(&title_with_items.reservation_start_date) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    { date_time_with_timezone_to_string(&title_with_items.reservation_deadline) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    { date_time_with_timezone_to_string(&title_with_items.order_date_to_maker) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    { date_time_with_timezone_to_string(&Some(title_with_items.updated_at.clone())) }
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    <a href={format!("/item_edit/{}", &title_with_items.id)}>{ "編集"}</a>
                                                </td>
                                                <td class={get_corresponding_color(&title_with_items.project_type).to_string()} rowspan={date_column_rowspan.to_string()}>
                                                    {&title_with_items.name}
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
                                                    {item.retail_price.unwrap_or(0)}
                                                </td>
                                                <td>
                                                    {resubmission}
                                                </td>
                                                <td>
                                                    {item.line.clone()}
                                                </td>
                                                <td class={get_corresponding_color(&title_with_items.catalog_status).to_string()} rowspan={date_column_rowspan.to_string()}>
                                                    {&title_with_items.catalog_status}
                                                </td>
                                                <td class={get_corresponding_color(&title_with_items.announcement_status).to_string()} rowspan={date_column_rowspan.to_string()}>
                                                    {&title_with_items.announcement_status}
                                                </td>
                                                <td rowspan={date_column_rowspan.to_string()}>
                                                    {title_with_items.remarks.clone().unwrap_or("".to_string())}
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
                                                    {item.retail_price.unwrap_or(0)}
                                                </td>
                                                <td>
                                                    {resubmission}
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
            </table>
        </div>
    }
}
