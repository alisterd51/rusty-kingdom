use crate::{
    app::{ResourceView, get_client, use_id_param},
    i18n::{t, use_i18n},
    pb::game::v1::{
        CollectFortressEnergyRequest, CollectFortressFoodRequest, CollectFortressGoldRequest,
        CollectFortressWoodRequest, GetFortressRequest,
        fortress_service_client::FortressServiceClient,
    },
};
use leptos::prelude::*;
use leptos_router::components::A;

macro_rules! make_collect_action {
    ($req_type:ident, $method:ident, $trigger:expr) => {
        Action::new_local(move |id: &i32| {
            let id = *id;
            let trigger = $trigger;
            async move {
                let mut client = FortressServiceClient::new(get_client());
                let request = tonic::Request::new($req_type { id });
                match client.$method(request).await {
                    Ok(_) => trigger.update(|n| *n += 1),
                    Err(e) => leptos::logging::error!("Collect failed: {}", e),
                }
            }
        })
    };
}

#[allow(clippy::similar_names)]
#[component]
pub fn FortressDetail() -> impl IntoView {
    let i18n = use_i18n();
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
    let collect_gold_action = make_collect_action!(
        CollectFortressGoldRequest,
        collect_fortress_gold,
        set_refresh_trigger
    );
    let collect_food_action = make_collect_action!(
        CollectFortressFoodRequest,
        collect_fortress_food,
        set_refresh_trigger
    );
    let collect_wood_action = make_collect_action!(
        CollectFortressWoodRequest,
        collect_fortress_wood,
        set_refresh_trigger
    );
    let collect_energy_action = make_collect_action!(
        CollectFortressEnergyRequest,
        collect_fortress_energy,
        set_refresh_trigger
    );

    view! {
        <div>
            <h2>{t!(i18n, fortress_detail)}</h2>
            <ResourceView
                resource=fortress_resource
                view=move |fortress_opt| {
                    fortress_opt
                        .map_or_else(
                            || t!(i18n, no_data).into_view().into_any(),
                            |f| {
                                view! {
                                    <ul>
                                        <li>{t!(i18n, id)}": " {f.id}</li>
                                        <ResourceRow
                                            label=t!(i18n, gold).into_view().into_any()
                                            value=f.gold
                                            id=f.id
                                            action=collect_gold_action
                                        />
                                        <ResourceRow
                                            label=t!(i18n, food).into_view().into_any()
                                            value=f.food
                                            id=f.id
                                            action=collect_food_action
                                        />
                                        <ResourceRow
                                            label=t!(i18n, wood).into_view().into_any()
                                            value=f.wood
                                            id=f.id
                                            action=collect_wood_action
                                        />
                                        <ResourceRow
                                            label=t!(i18n, energy).into_view().into_any()
                                            value=f.energy
                                            id=f.id
                                            action=collect_energy_action
                                        />
                                    </ul>
                                    <div>
                                        <A href=format!(
                                            "/fortresses/{}/buildings",
                                            f.id,
                                        )>{t!(i18n, view_buildings)}</A>
                                    </div>
                                }
                                    .into_any()
                            },
                        )
                }
            />
            <br />
            <A href="/fortresses">{t!(i18n, back_to_fortresses)}</A>
        </div>
    }
}

#[component]
fn ResourceRow(label: AnyView, value: i32, id: i32, action: Action<i32, ()>) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <li>
            {label} ": " {value} " "
            <button
                on:click=move |_| {
                    action.dispatch(id);
                }
                disabled=move || action.pending().get()
            >
                {t!(i18n, collect)}
            </button>
        </li>
    }
}
