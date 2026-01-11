use crate::i18n::{t, use_i18n};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div>
            <h1>{t!(i18n, project_presentation)}</h1>
            <p>{t!(i18n, project_intro_1)}<br />{t!(i18n, project_intro_2)}</p>
            <p>
                <strong>{t!(i18n, note_label)}</strong>
                {t!(i18n, project_intro_note)}
            </p>

            <hr />

            <h3>{t!(i18n, architecture_presentation)}</h3>

            <h4>{t!(i18n, frontends)}</h4>
            <ul>
                <li>
                    <strong>{t!(i18n, web_client)}</strong>
                    {t!(i18n, web_client_desc_1)}
                    <code>{t!(i18n, leptos)}</code>
                    {t!(i18n, web_client_desc_2)}
                    <code>{t!(i18n, trunk)}</code>
                    {t!(i18n, web_client_desc_3)}
                </li>
                <li>
                    <strong>{t!(i18n, cli_client)}</strong>
                    {t!(i18n, cli_client_desc_1)}
                    <code>{t!(i18n, clap)}</code>
                    {t!(i18n, cli_client_desc_2)}
                    <code>{t!(i18n, serde)}</code>
                    {t!(i18n, cli_client_desc_3)}
                </li>
            </ul>

            <h4>{t!(i18n, backends)}</h4>
            <ul>
                <li>
                    <strong>{t!(i18n, main_microservice)}</strong>
                    {t!(i18n, main_microservice_desc_1)}
                </li>
                <li>
                    <strong>{t!(i18n, crud_microservice)}</strong>
                    {t!(i18n, crud_microservice_desc_1)}
                    <code>{t!(i18n, diesel)}</code>
                    {t!(i18n, crud_microservice_desc_2)}
                </li>
            </ul>

            <hr />

            <h3>{t!(i18n, technologies_infrastructure)}</h3>

            <h4>{t!(i18n, database)}</h4>
            <ul>
                <li>{t!(i18n, database_desc_1)}</li>
                <li>{t!(i18n, database_desc_2)}</li>
            </ul>

            <h4>{t!(i18n, cicd)}</h4>
            <ul>
                <li>{t!(i18n, cicd_desc_1)}</li>
                <li>{t!(i18n, cicd_desc_2)}</li>
                <li>{t!(i18n, cicd_desc_3)}</li>
            </ul>

            <h4>{t!(i18n, runtime)}</h4>
            <ul>
                <li>{t!(i18n, runtime_desc_1)}</li>
                <li>{t!(i18n, runtime_desc_2)}</li>
                <li>{t!(i18n, runtime_desc_3)}</li>
            </ul>

            <h4>{t!(i18n, other_wip)}</h4>
            <ul>
                <li>{t!(i18n, other_wip_desc_1)}</li>
                <li>{t!(i18n, other_wip_desc_2)}</li>
                <li>{t!(i18n, other_wip_desc_3)}</li>
                <li>{t!(i18n, other_wip_desc_4)}</li>
            </ul>
        </div>
    }
}
