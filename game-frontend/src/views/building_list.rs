use crate::{
    app::{ResourceView, get_client},
    game::{ListBuildingsRequest, building_service_client::BuildingServiceClient},
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn BuildingList() -> impl IntoView {
    let buildings_resource = LocalResource::new(move || async move {
        let mut client = BuildingServiceClient::new(get_client());
        let request = tonic::Request::new(ListBuildingsRequest {});
        client
            .list_buildings(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(|e| e.to_string())
    });

    view! {
        <div>
            <h2>"Building List"</h2>
            <ResourceView
                resource=buildings_resource
                view=|resp| {
                    view! {
                        <ul>
                            <For
                                each=move || resp.buildings.clone()
                                key=|b| b.id
                                children=|b| {
                                    view! {
                                        <li>
                                            <A href=format!(
                                                "/buildings/{}",
                                                b.id,
                                            )>{format!("{} #{}", b.name, b.id)}</A>
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
