use pages::home::Home;
use pages::item::{item_edit::ItemEdit, item_list::ItemList};
use pages::maker::maker_list::MakerList;
use pages::worker::worker_list::WorkerList;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod model;
mod pages;
mod settings;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/maker_list")]
    MakerList,
    #[at("/item_edit/:title_id")]
    ItemEdit { title_id: String },
    #[at("/worker_list")]
    WorkerList,
    #[at("/item_list")]
    ItemList,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route>  render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home/> }
        }
        Route::ItemList => {
            html! { <ItemList/> }
        }
        Route::MakerList => {
            html! { <MakerList/> }
        }
        ItemRoute::Edit { title_id } => {
            html! {
                <ItemEdit title_id={ title_id.clone() }/>
            }
        }
        Route::WorkerList => {
            html! { <WorkerList/> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<App>();
}
