use crate::{
    app::{ResourceView, get_client, use_id_param},
    game::{ListBuildingsByFortressRequest, building_service_client::BuildingServiceClient},
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn FortressBuildingList() -> impl IntoView {
    let id_signal = use_id_param();
    let buildings_resource = LocalResource::new(move || {
        let fortress_id = id_signal();
        async move {
            let Some(fortress_id) = fortress_id else {
                return Err("Invalid Fortress ID".to_string());
            };
            let mut service = BuildingServiceClient::new(get_client());
            let request = tonic::Request::new(ListBuildingsByFortressRequest { fortress_id });
            service
                .list_buildings_by_fortress(request)
                .await
                .map(tonic::Response::into_inner)
                .map_err(|e| e.to_string())
        }
    });

    view! {
        <div>
            <h2>
                {move || {
                    id_signal()
                        .map_or_else(
                            || "Fortress not found".to_string(),
                            |id| format!("Buildings for Fortress #{id}"),
                        )
                }}
            </h2>
            <ResourceView
                resource=buildings_resource
                view=|resp| {
                    let buildings = resp.buildings;
                    let is_empty = buildings.is_empty();

                    view! {
                        <ul>
                            <For
                                each=move || buildings.clone()
                                key=|b| b.id
                                children=|b| {
                                    view! {
                                        <li>
                                            <A href=format!(
                                                "/buildings/{}",
                                                b.id,
                                            )>{format!("{} (Lvl {})", b.name, b.level)}</A>
                                        </li>
                                    }
                                }
                            />
                        </ul>
                        {if is_empty { Some(view! { <p>"No buildings found."</p> }) } else { None }}
                    }
                }
            />
            <br />
            {move || {
                id_signal()
                    .map_or_else(
                        || view! { <A href="/fortresses">"Back to Fortress List"</A> }.into_any(),
                        |id| {
                            view! { <A href=format!("/fortresses/{}", id)>"Back to Fortress"</A> }
                                .into_any()
                        },
                    )
            }}
        </div>
    }
}
