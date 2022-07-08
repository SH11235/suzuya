use yew::{function_component, html};

#[function_component(Input)]
pub fn input_text() -> Html {
    html! {
      <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container">
          <a class="navbar-brand" href="#">{"Yew TODO App"}</a>
        </div>
      </nav>
    }
}
