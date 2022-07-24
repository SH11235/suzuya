use pages::edit_item::EditItem;
use pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/edit_item/:title")]
    EditItem { title: String },
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
        Route::EditItem { title } => {
            html! {
                <EditItem title={ title.clone() }/>
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<App>();
}
