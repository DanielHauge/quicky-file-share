#[path = "pages/download.rs"]
mod download;
#[path = "pages/home.rs"]
mod home;
#[path = "pages/not_found.rs"]
mod not_found;

use download::{Download};
use home::Home;
use not_found::NotFound;
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    Download { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Download { id } => {
            html! { <Download file_id={id} /> }
        }
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1 class="title">{ "Quicky file share!" }</h1>
            <hr class="style-seven"/>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
