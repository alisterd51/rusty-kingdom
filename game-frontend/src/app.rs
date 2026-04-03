use crate::{
    i18n::I18nContextProvider,
    pb::game::v1::{
        building_service_client::BuildingServiceClient,
        fortress_service_client::FortressServiceClient,
    },
    views::{
        building_detail::BuildingDetail, building_list::BuildingList,
        fortress_building_list::FortressBuildingList, fortress_detail::FortressDetail,
        fortress_list::FortressList, home::Home, nav_menu::NavMenu, not_found::NotFound,
    },
};
use leptos::prelude::*;
use leptos_oidc::{Auth, AuthErrorContext, AuthLoading, AuthParameters, AuthSignal, Challenge};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params_map,
    path,
};
use tonic::{
    Status,
    metadata::MetadataValue,
    service::{Interceptor, interceptor::InterceptedService},
};
use tonic_web_wasm_client::Client;

const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(val) => val,
    None => "https://rusty.anclarma.fr",
};
const FRONTEND_URL: &str = match option_env!("FRONTEND_URL") {
    Some(val) => val,
    None => "https://rusty.anclarma.fr",
};
const AUTH_URL: &str = match option_env!("AUTH_URL") {
    Some(val) => val,
    None => "https://auth.rusty.anclarma.fr",
};
const CLIENT_ID: &str = match option_env!("CLIENT_ID") {
    Some(val) => val,
    None => "rusty-client",
};

#[derive(Clone)]
pub struct AuthInterceptor {
    pub token: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        if !self.token.is_empty()
            && let Ok(meta) = MetadataValue::try_from(format!("Bearer {}", self.token))
        {
            req.metadata_mut().insert("authorization", meta);
        }
        Ok(req)
    }
}

pub fn get_token() -> String {
    match use_context::<AuthSignal>().map(|sig| sig.get()) {
        Some(Auth::Authenticated(session)) => session.access_token(),
        _ => String::new(),
    }
}

fn get_client() -> Client {
    Client::new(BACKEND_URL.to_string())
}

pub fn get_fortress_client(
    token: String,
) -> FortressServiceClient<InterceptedService<Client, AuthInterceptor>> {
    FortressServiceClient::with_interceptor(get_client(), AuthInterceptor { token })
}

pub fn get_building_client(
    token: String,
) -> BuildingServiceClient<InterceptedService<Client, AuthInterceptor>> {
    BuildingServiceClient::with_interceptor(get_client(), AuthInterceptor { token })
}

pub fn use_id_param() -> impl Fn() -> Option<i32> + Copy {
    let params = use_params_map();
    move || params.get().get("id").and_then(|i| i.parse::<i32>().ok())
}

#[component]
pub fn App() -> impl IntoView {
    let auth_parameters = AuthParameters {
        issuer: AUTH_URL.to_string() + "/auth/v1",
        client_id: CLIENT_ID.to_string(),
        redirect_uri: FRONTEND_URL.to_string(),
        post_logout_redirect_uri: FRONTEND_URL.to_string(),
        challenge: Challenge::S256,
        scope: Some("openid profile email".to_string()),
        audience: None,
    };
    let auth: AuthSignal = Auth::signal();
    provide_context(auth);

    let _ = Auth::init(auth_parameters);

    view! {
        <I18nContextProvider>
            <Router>
                <div class="app-container">
                    <NavMenu />

                    <main>
                        <AuthLoading>
                            <p>"Vérification de la session..."</p>
                        </AuthLoading>
                        <AuthErrorContext>
                            <div style="background-color: #fee; padding: 1rem; border: 1px solid red;">
                                <h3>"Erreur d'authentification OIDC"</h3>
                                <p>
                                    {move || {
                                        auth.get()
                                            .error()
                                            .map_or_else(
                                                || "Erreur inconnue".to_string(),
                                                |e| format!("{e:?}"),
                                            )
                                    }}
                                </p>
                            </div>
                        </AuthErrorContext>
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
