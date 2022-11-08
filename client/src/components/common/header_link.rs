use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderLinkProperty {
    pub current_page: String,
}

#[function_component(HeaderLink)]
pub fn select_box(props: &HeaderLinkProperty) -> Html {
    html! {
        <header class="header-link">
            <ul>
                {
                    if props.current_page == "home" {
                        html! {
                            <li class="current-page"><a class="link-button" href="/">{ "Home" }</a></li>
                        }
                    } else {
                        html! {
                            <li><a class="link-button" href="/">{ "Home" }</a></li>
                        }
                    }
                }
                {
                    if props.current_page == "item_list" {
                        html! {
                            <li class="current-page"><a class="link-button" href="/item_list">{ "ItemList" }</a></li>
                        }
                    } else {
                        html! {
                            <li><a class="link-button" href="/item_list">{ "ItemList" }</a></li>
                        }
                    }
                }
                {
                    if props.current_page == "item_new" {
                        html! {
                            <li class="current-page"><a class="link-button" href="/item_new">{ "ItemNew" }</a></li>
                        }
                    } else {
                        html! {
                            <li><a class="link-button" href="/item_new">{ "ItemNew" }</a></li>
                        }
                    }
                }
                {
                    if props.current_page == "maker_list" {
                        html! {
                            <li class="current-page"><a class="link-button" href="/maker_list">{ "MakerList" }</a></li>
                        }
                    } else {
                        html! {
                            <li><a class="link-button" href="/maker_list">{ "MakerList" }</a></li>
                        }
                    }
                }
                {
                    if props.current_page == "worker_list" {
                        html! {
                            <li class="current-page"><a class="link-button" href="/worker_list">{ "WorkerList" }</a></li>
                        }
                    } else {
                        html! {
                            <li><a class="link-button" href="/worker_list">{ "WorkerList" }</a></li>
                        }
                    }
                }
            </ul>
        </header>
    }
}
