use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::home::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/parcels-rs.css"/>

        // sets the document title
        <Title text="Send nudes"/>

        // content for this welcome page
        <Router>
            <nav class="w-screen bg-slate-900 py-4 text-slate-50 text-xl flex flex-row justify-center">
                <ul class="flex gap-8 justify-center flex-row">
                    <li>
                        <span class="">"📦 parcels.rs"</span>
                    </li>
                    <li>
                        <span class="">"x/x packages delivered"</span>
                    </li>
                </ul>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}
