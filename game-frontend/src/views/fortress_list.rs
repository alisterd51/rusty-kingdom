use crate::{
    app::{ResourceView, get_client},
    i18n::{t, use_i18n},
    pb::game::v1::{
        CreateFortressRequest, DeleteFortressRequest, ListFortressesRequest,
        fortress_service_client::FortressServiceClient,
    },
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn FortressList() -> impl IntoView {
    let i18n = use_i18n();
    let (refresh_trigger, set_refresh_trigger) = signal(0);
    let fortresses_resource = LocalResource::new(move || {
        refresh_trigger.get();
        async move {
            let mut fortress_client = FortressServiceClient::new(get_client());
            let request = tonic::Request::new(ListFortressesRequest {});
            let response = fortress_client.list_fortresses(request).await;
            response
                .map(tonic::Response::into_inner)
                .map_err(|e| e.to_string())
        }
    });
    let create_action = Action::new_local(move |()| async move {
        let mut client = FortressServiceClient::new(get_client());
        let request = tonic::Request::new(CreateFortressRequest {});
        match client.create_fortress(request).await {
            Ok(_) => {
                set_refresh_trigger.update(|n| *n += 1);
            }
            Err(e) => leptos::logging::error!("Failed to create fortress: {}", e),
        }
    });
    let delete_action = Action::new_local(move |id: &i32| {
        let id = *id;
        async move {
            let mut client = FortressServiceClient::new(get_client());
            let request = tonic::Request::new(DeleteFortressRequest { id });
            match client.delete_fortress(request).await {
                Ok(_) => set_refresh_trigger.update(|n| *n += 1),
                Err(e) => leptos::logging::error!("Failed to delete fortress: {}", e),
            }
        }
    });

    view! {
        <div>
            <h2>{t!(i18n, fortress_list)}</h2>
            <div>
                <button
                    on:click=move |_| {
                        create_action.dispatch(());
                    }
                    disabled=move || create_action.pending().get()
                >
                    {move || {
                        if create_action.pending().get() {
                            t!(i18n, creating).into_view().into_any()
                        } else {
                            t!(i18n, create_new_fortress).into_view().into_any()
                        }
                    }}
                </button>
            </div>
            <ResourceView
                resource=fortresses_resource
                view=move |resp| {
                    view! {
                        <ul>
                            <For
                                each=move || resp.fortresses.clone()
                                key=|f| f.id
                                children=move |f| {
                                    view! {
                                        <li>
                                            <A href=format!(
                                                "/fortresses/{}",
                                                f.id,
                                            )>{t!(i18n, fortress)}" #"{f.id}</A>
                                            " "
                                            <button
                                                on:click=move |_| {
                                                    delete_action.dispatch(f.id);
                                                }
                                                disabled=move || delete_action.pending().get()
                                            >
                                                {t!(i18n, delete)}
                                            </button>
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    }
                }
            />
        </div>
    }
}
