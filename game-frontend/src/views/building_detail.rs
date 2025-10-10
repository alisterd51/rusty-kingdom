use crate::{
    app::{ResourceView, get_client, use_id_param},
    common::Costs,
    game::{
        GetBuildingRequest, GetImproveBuildingCostsRequest, ImproveBuildingRequest,
        building_service_client::BuildingServiceClient,
    },
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn BuildingDetail() -> impl IntoView {
    let id_signal = use_id_param();
    let (refresh_trigger, set_refresh_trigger) = signal(0);
    let data_resource = LocalResource::new(move || {
        let id = id_signal();
        refresh_trigger.get();
        async move {
            let Some(id) = id else { return Ok(None) };
            let mut building_client = BuildingServiceClient::new(get_client());
            let building_req = tonic::Request::new(GetBuildingRequest { id });
            let building = match building_client.get_building(building_req).await {
                Ok(resp) => match resp.into_inner().building {
                    Some(b) => b,
                    None => return Ok(None),
                },
                Err(status) => {
                    if status.code() == tonic::Code::NotFound
                        || status.message().contains("not found")
                    {
                        return Ok(None);
                    }
                    return Err(status.to_string());
                }
            };
            let mut cost_client = BuildingServiceClient::new(get_client());
            let cost_req = tonic::Request::new(GetImproveBuildingCostsRequest { id });
            let costs = cost_client
                .get_improve_building_costs(cost_req)
                .await
                .map(|resp| resp.into_inner().costs)
                .map_err(|e| e.to_string())?;

            Ok(Some((building, costs)))
        }
    });
    let improve_action = Action::new_local(move |id: &i32| {
        let id = *id;
        async move {
            let mut client = BuildingServiceClient::new(get_client());
            let request = tonic::Request::new(ImproveBuildingRequest { id });
            if (client.improve_building(request).await).is_ok() {
                set_refresh_trigger.update(|n| *n += 1);
            }
        }
    });

    view! {
        <div>
            <h2>"Building Detail"</h2>
            <ResourceView
                resource=data_resource
                view=move |data_opt| {
                    match data_opt {
                        Some((b, costs)) => {
                            view! {
                                <ul>
                                    <li>"ID: " {b.id}</li>
                                    <li>"Name: " {b.name}</li>
                                    <li>"Level: " {b.level}</li>
                                    <li>"Fortress ID: " {b.fortress_id}</li>
                                </ul>
                                <UpgradeSection id=b.id costs=costs action=improve_action />
                                <div>
                                    <A href=format!(
                                        "/fortresses/{}",
                                        b.fortress_id,
                                    )>{format!("-> Go to Fortress #{}", b.fortress_id)}</A>
                                </div>
                            }
                                .into_any()
                        }
                        None => view! { "No data (Invalid ID or Not Found)" }.into_any(),
                    }
                }
            />
            <br />
            <A href="/buildings">"Back"</A>
        </div>
    }
}

#[component]
fn UpgradeSection(id: i32, costs: Option<Costs>, action: Action<i32, ()>) -> impl IntoView {
    view! {
        <h3>"Upgrade Building"</h3>
        <div>
            {costs
                .map_or_else(
                    || view! { <p>"No costs available"</p> }.into_any(),
                    |c| {
                        view! {
                            <p>
                                "Cost: " "Gold: " {c.gold} " " "Food: " {c.food} " " "Wood: "
                                {c.wood} " " "Energy: " {c.energy}
                            </p>
                        }
                            .into_any()
                    },
                )}
            <button
                on:click=move |_| {
                    action.dispatch(id);
                }
                disabled=move || action.pending().get()
            >
                {move || { if action.pending().get() { "Upgrading..." } else { "Upgrade" } }}
            </button>
        </div>
    }
}
