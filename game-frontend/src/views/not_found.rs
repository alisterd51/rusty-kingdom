use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            <h2>"404 - Page not found"</h2>
            <A href="/">"Back to home"</A>
        </div>
    }
}
