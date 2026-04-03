use crate::{
    app::{ResourceView, get_building_client, get_token},
    i18n::{t, use_i18n},
    pb::game::v1::ListBuildingsRequest,
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn BuildingList() -> impl IntoView {
    let i18n = use_i18n();
    let buildings_resource = LocalResource::new(move || async move {
        let token = get_token();
        let mut client = get_building_client(token);
        let request = tonic::Request::new(ListBuildingsRequest {});
        client
            .list_buildings(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(|e| e.to_string())
    });

    view! {
        <div>
            <h2>{t!(i18n, building_list)}</h2>
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
