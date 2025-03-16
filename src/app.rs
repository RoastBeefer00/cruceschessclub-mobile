use crate::components::Nav;
use crate::pages::{About, Home};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
        <nav>
            <Nav/>
        </nav>
        <main>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/about") view=About/>
                // <Route path=path!("/users") view=Users/>
                // <Route path=path!("/users/:id") view=UserProfile/>
                // <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </main>
      </Router>
    }
}
