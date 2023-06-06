#[path = "pages/home.rs"] mod home;
#[path = "pages/file.rs"] mod file;
#[path = "pages/not_found.rs"] mod not_found;

use yew_router::prelude::*;
use home::Home;
use not_found::NotFound;
use file::{File, FileProps};
use yew::{function_component, html, Html, props};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/file/:id")]
    File { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(route: Route) -> Html {
    match route {
        Route::File { id } => {
                let props = props!{
                    FileProps { file_id: id }
                };
                html! { <File ..props /> }
            }, 
        Route::Home => html!{ <Home /> },
        Route::NotFound => html!{ <NotFound /> }
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}