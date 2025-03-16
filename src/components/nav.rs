use leptos::prelude::*;

#[derive(Clone, PartialEq)]
enum Routes {
    Home,
    About,
    Schedule,
}

#[component]
pub fn Nav() -> impl IntoView {
    let (route, set_route) = signal(Routes::Home);
    let change_route = move |route: Routes| set_route.set(route);
    view! {
        <nav class="top-0 left-0 bg-gray-800 flex flex-row w-screen">
            <img src="public/ccc.png" class="max-h-10 max-w-10 mt-2 ml-2 mb-2" alt="CCC logo"/>
            // <span class="text-white pt-5 px-3 font-extrabold text-2xl">Cruces Chess Club</span>
            <ul class="flex flex-row [&_:is(li)]:pt-6 [&_:is(li)]:px-2 text-white">
                <li class=move || if route.get() == Routes::Home {"bg-gray-900"} else {""}>
                    <a href="/" on:click=move |_| change_route(Routes::Home)>"Home"</a>
                </li>
                <li class=move || if route.get() == Routes::About {"bg-gray-900"} else {""}>
                    <a href="/about" on:click=move |_| change_route(Routes::About)>"About"</a>
                </li>
                <li class=move || if route.get() == Routes::Schedule {"bg-gray-900"} else {""}>
                    <a href="/schedule" on:click=move |_| change_route(Routes::Schedule)>"Schedule"</a>
                </li>
                // <li class="hover:bg-gray-900">
                //     <a href="/lessons">"Lessons"</a>
                // </li>
                // <li class="hover:bg-gray-900">
                //     <a href="/gallery">"Gallery"</a>
                // </li>
                // <li class="hover:bg-gray-900">
                //     <a href="/tournaments">"Tournaments"</a>
                // </li>
            </ul>
        </nav>
    }
}
