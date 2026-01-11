use crate::{
    i18n::I18nContextProvider,
    views::{
        building_detail::BuildingDetail, building_list::BuildingList,
        fortress_building_list::FortressBuildingList, fortress_detail::FortressDetail,
        fortress_list::FortressList, home::Home, nav_menu::NavMenu, not_found::NotFound,
    },
};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params_map,
    path,
};
use tonic_web_wasm_client::Client;

const BASE_URL: &str = match option_env!("GAME_API_URL") {
    Some(val) => val,
    None => "https://rusty.anclarma.fr",
};

pub fn get_client() -> Client {
    Client::new(BASE_URL.to_string())
}

pub fn use_id_param() -> impl Fn() -> Option<i32> + Copy {
    let params = use_params_map();
    move || params.get().get("id").and_then(|i| i.parse::<i32>().ok())
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18nContextProvider>
            <Router>
                <div class="app-container">
                    <NavMenu />

                    <main>
                        <Routes fallback=|| view! { <NotFound /> }>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/fortresses") view=FortressList />
                            <Route path=path!("/fortresses/:id") view=FortressDetail />
                            <Route
                                path=path!("/fortresses/:id/buildings")
                                view=FortressBuildingList
                            />
                            <Route path=path!("/buildings") view=BuildingList />
                            <Route path=path!("/buildings/:id") view=BuildingDetail />
                        </Routes>
                    </main>
                </div>
            </Router>
        </I18nContextProvider>
    }
}

#[component]
pub fn ResourceView<T, F, IV>(resource: LocalResource<Result<T, String>>, view: F) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> IV + Send + 'static,
    IV: IntoView,
{
    view! {
        <Transition fallback=|| {
            "Loading..."
        }>
            {move || {
                resource
                    .get()
                    .map(|res| match res {
                        Ok(data) => view(data).into_any(),
                        Err(e) => {
                            view! {
                                "Error: "
                                {e}
                            }
                                .into_any()
                        }
                    })
            }}
        </Transition>
    }
}
