use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <>
        <h1>{ "Suzuya Home" }</h1>
        <ul>
          <li><a href="/item_list">{ "ItemList" }</a></li>
          <li><a href="/maker_list">{ "MakerList" }</a></li>
          <li><a href="/worker_list">{ "WorkerList" }</a></li>
        </ul>
      </>
    }
}
