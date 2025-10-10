use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavMenu() -> impl IntoView {
    view! {
        <nav>
            <A href="/">"Home"</A>
            " | "
            <A href="/fortresses">"Fortresses"</A>
            " | "
            <A href="/buildings">"Buildings"</A>
            " | "
            <a href="https://github.com/alisterd51/rusty-kingdom">"GitHub"</a>
        </nav>
    }
}
