use crate::components::input::Input;
use chrono::{DateTime, Utc};
use chrono_tz::{Asia::Tokyo, Tz};
use serde::{Deserialize, Serialize};
// use reqwasm::http::Request;
use reqwest::header;
use urlencoding::decode;
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StatusName {
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Color {
    WHITE,
    PINK,
    YELLOW,
    LIGHTBLUE,
    GREEN,
    GRAY,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ItemModel {
    pub id: i32,
    pub release_date: Option<String>,           // 発売日
    pub reservation_start_date: Option<String>, // 予約開始日(BtoBおよびBtoC)
    pub reservation_deadline: Option<String>,   // 予約締切日
    pub order_date: Option<String>,             // メーカーへの発注日
    pub title: String,
    pub project_type: String,
    pub last_updated: String, // 最終更新日（ステータス変更時）
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>, // 種類数
    pub illust_status: String,
    // pic：person in charge
    pub pic_illust_id: Option<i32>, // from user 「イラスト担当者」
    pub design_status: String,
    pub pic_design_id: Option<i32>, // from user 「デザイン担当者」
    pub maker_id: Option<i32>,      // from maker
    pub retail_price: Option<i32>,  // 上代
    pub double_check_person_id: Option<i32>, // from user 社員名
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>, // 備考
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ItemEdit {
    items: Vec<ItemModel>,
    // users: Vec<user::Model>,
    // makers: Vec<maker::Model>,
    project_type_list: Vec<StatusName>,
    illust_status_list: Vec<StatusName>,
    design_status_list: Vec<StatusName>,
    release_date: Option<String>,
    reservation_start_date: Option<String>,
    reservation_deadline: Option<String>,
    order_date: Option<String>,
    last_updated: String,
    catalog_status_list: Vec<StatusName>,
    announce_status_list: Vec<StatusName>,
}

#[derive(Properties, PartialEq)]
pub struct EditItemPageProperty {
    pub title: String,
}

#[function_component(EditItem)]
pub fn edit_item(props: &EditItemPageProperty) -> Html {
    let items = use_state(|| vec![]);
    let url = format!(
        "http://localhost:1123/api/item/{}",
        decode(&props.title.clone()).expect("UTF-8")
    );
    {
        let items = items.clone();
        use_effect_with_deps(
            move |_| {
                let items = items.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = reqwest::Client::new().get(&url);
                    let fetched_items: ItemEdit = client
                        .send()
                        .await
                        .expect("Failed to fetch items")
                        .json()
                        .await
                        .expect("Failed to parse items");
                    // let fetched_items: String = client
                    //     .send()
                    //     .await
                    //     .expect("Failed to fetch items")
                    //     .text()
                    //     .await
                    //     .expect("Failed to parse items");
                    // web_sys::console::log_1(&JsValue::from_str(&fetched_items));
                    web_sys::console::log_1(&JsValue::from_str(
                        &serde_json::to_string(&fetched_items).unwrap(),
                    ));
                    items.set(vec![fetched_items]);
                });
                || ()
            },
            (),
        );
    }

    html! {
      <div>
        <h1>{ "Edit Items" }</h1>
        {
            if items.len() == 0 {
                html! {
                    <p>{ "Loading..." }</p>
                }
            } else {
                let item = &items[0].items[0];
                let release_date = item.release_date.clone().unwrap_or_default();
                html! {
                    <>
                        { "発売日：" }<Input input_type="text" placeholder="yyyy-mm-dd" id="release_date" name="release_date" value={release_date} />
                        <br/>
                        // <div>{ &item.title }</div>
                        // <div>{ &item.project_type }</div>
                        // <div>{ &item.illust_status }</div>
                        // <div>{ &item.design_status }</div>
                        // <div>{ &item.catalog_status }</div>
                        // <div>{ &item.announcement_status }</div>
                        // <div>{ &item.last_updated }</div>
                    </>
                }
            }
        }
      </div>
    }
}
