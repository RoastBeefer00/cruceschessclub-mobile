use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="m-1 flex flex-col text-white">
          <div class="p-3 bg-gray-800 rounded-lg">
            <p class="text-2xl text-center font-extrabold">About Us</p>
            <div class="mt-4 p-3 bg-gray-800 rounded-lg">
                <p class="text-xl">
                    Cruces Chess Club is a Las Cruces based non-profit organization dedicated
                    to serving our Las Cruces chess community through financial
                    support and community outreach.
                    <br/>
                    <br/>
                    Our mission is to bring the community together to have fun and enjoy chess.
                    We aim to support the community by providing chess boards and timers as well as
                    enabling members to travel and compete.
                </p>
            </div>
          </div>
        </div>
    }
}
