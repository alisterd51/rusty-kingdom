use crate::i18n::{Locale, t, use_i18n};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavMenu() -> impl IntoView {
    let i18n = use_i18n();

    let on_switch = move |_| {
        let new_locale = match i18n.get_locale() {
            Locale::en => Locale::fr,
            Locale::fr => Locale::en,
        };
        i18n.set_locale(new_locale);
    };

    view! {
        <nav>
            <A href="/">{t!(i18n, home)}</A>
            " | "
            <A href="/fortresses">{t!(i18n, fortresses)}</A>
            " | "
            <A href="/buildings">{t!(i18n, buildings)}</A>
            " | "
            <a href="https://github.com/alisterd51/rusty-kingdom">{t!(i18n, github)}</a>
            " | "
            <button on:click=on_switch>{t!(i18n, lang)}</button>
        </nav>
    }
}
