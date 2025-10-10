use crate::{
    app::{ResourceView, get_client, use_id_param},
    game::{
        CollectFortressEnergyRequest, CollectFortressFoodRequest, CollectFortressGoldRequest,
        CollectFortressWoodRequest, GetFortressRequest,
        fortress_service_client::FortressServiceClient,
    },
};
use leptos::prelude::*;
use leptos_router::components::A;

#[allow(clippy::similar_names)]
#[component]
pub fn FortressDetail() -> impl IntoView {
    let id_signal = use_id_param();
    let (refresh_trigger, set_refresh_trigger) = signal(0);
    let fortress_resource = LocalResource::new(move || {
        let id = id_signal();
        refresh_trigger.get();
        async move {
            let Some(id) = id else { return Ok(None) };
            let mut service = FortressServiceClient::new(get_client());
            let request = tonic::Request::new(GetFortressRequest { id });

            match service.get_fortress(request).await {
                Ok(resp) => Ok(resp.into_inner().fortress),
                Err(status) => {
                    if status.code() == tonic::Code::NotFound
                        || status.message().contains("not found")
                    {
                        Ok(None)
                    } else {
                        Err(status.to_string())
                    }
                }
            }
        }
    });

    macro_rules! make_collect_action {
        ($req_type:ident, $method:ident) => {
            Action::new_local(move |id: &i32| {
                let id = *id;
                async move {
                    let mut client = FortressServiceClient::new(get_client());
                    let request = tonic::Request::new($req_type { id });
                    match client.$method(request).await {
                        Ok(_) => set_refresh_trigger.update(|n| *n += 1),
                        Err(e) => leptos::logging::error!("Collect failed: {}", e),
                    }
                }
            })
        };
    }
    let collect_gold_action =
        make_collect_action!(CollectFortressGoldRequest, collect_fortress_gold);
    let collect_food_action =
        make_collect_action!(CollectFortressFoodRequest, collect_fortress_food);
    let collect_wood_action =
        make_collect_action!(CollectFortressWoodRequest, collect_fortress_wood);
    let collect_energy_action =
        make_collect_action!(CollectFortressEnergyRequest, collect_fortress_energy);

    view! {
        <div>
            <h2>"Fortress Detail"</h2>
            <ResourceView
                resource=fortress_resource
                view=move |fortress_opt| {
                    fortress_opt
                        .map_or_else(
                            || view! { "No data" }.into_any(),
                            |f| {
                                view! {
                                    <ul>
                                        <li>"ID: " {f.id}</li>
                                        <ResourceRow
                                            label="Gold"
                                            value=f.gold
                                            id=f.id
                                            action=collect_gold_action
                                        />
                                        <ResourceRow
                                            label="Food"
                                            value=f.food
                                            id=f.id
                                            action=collect_food_action
                                        />
                                        <ResourceRow
                                            label="Wood"
                                            value=f.wood
                                            id=f.id
                                            action=collect_wood_action
                                        />
                                        <ResourceRow
                                            label="Energy"
                                            value=f.energy
                                            id=f.id
                                            action=collect_energy_action
                                        />
                                    </ul>
                                    <div>
                                        <A href=format!(
                                            "/fortresses/{}/buildings",
                                            f.id,
                                        )>"View Buildings ->"</A>
                                    </div>
                                }
                                    .into_any()
                            },
                        )
                }
            />
            <br />
            <A href="/fortresses">"Back"</A>
        </div>
    }
}

#[component]
fn ResourceRow(label: &'static str, value: i32, id: i32, action: Action<i32, ()>) -> impl IntoView {
    view! {
        <li>
            {label} ": " {value} " "
            <button
                on:click=move |_| {
                    action.dispatch(id);
                }
                disabled=move || action.pending().get()
            >
                "Collect"
            </button>
        </li>
    }
}
