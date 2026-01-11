use crate::i18n::{t, use_i18n};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div>
            <h2>{t!(i18n, page_not_found)}</h2>
            <A href="/">{t!(i18n, back_to_home)}</A>
        </div>
    }
}
