use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1>"Rusty-Kingdom Project Presentation"</h1>
            <p>
                "This is an incremental, multi-client, bot-friendly, multiplayer game with low latency."
                <br />
                "More generally, this project serves as an open-source technology demonstrator focused on performance."
            </p>
            <p>
                <strong>"Note: "</strong>
                "The gameplay is currently very basic."
            </p>

            <hr />

            <h3>"Current Architecture"</h3>

            <h4>"Frontends"</h4>
            <ul>
                <li>
                    <strong>"Web Client"</strong>
                    ": Developed in Rust using the "
                    <code>"Leptos"</code>
                    " framework, compiled to WebAssembly (WASM) via "
                    <code>"Trunk"</code>
                    "."
                </li>
                <li>
                    <strong>"CLI Client"</strong>
                    ": Developed in Rust (using "
                    <code>"Clap"</code>
                    " and "
                    <code>"Serde"</code>
                    "), designed to facilitate bot scripting via Bash and jq."
                </li>
            </ul>

            <h4>"Backend"</h4>
            <ul>
                <li>
                    <strong>"Main Microservice"</strong>
                    ": Developed in Rust, communicates with clients via gRPC/gRPC-web and with the CRUD service via gRPC."
                </li>
                <li>
                    <strong>"CRUD Microservice"</strong>
                    ": Developed in Rust, manages data persistence via gRPC and the "
                    <code>"Diesel"</code>
                    " ORM (PostgreSQL)."
                </li>
            </ul>

            <hr />

            <h3>"Technologies & Infrastructure"</h3>

            <h4>"Database"</h4>
            <ul>
                <li>"PostgreSQL"</li>
                <li>"(Redis: sidelined for now, negligible performance gain)"</li>
            </ul>

            <h4>"CI/CD"</h4>
            <ul>
                <li>"Docker: Containerization"</li>
                <li>"GitHub Actions: Tests, compilation, build, and registry publication"</li>
                <li>"(FluxCD: pending Kubernetes migration for CD)"</li>
            </ul>

            <h4>"Runtime"</h4>
            <ul>
                <li>"Docker Compose (Migration to Kubernetes in progress)"</li>
                <li>"Traefik: Reverse proxy and TLS certificate management"</li>
                <li>"Nginx: Static server for the Web Client"</li>
            </ul>

            <h4>"Other & WIP"</h4>
            <ul>
                <li>"Protobuf: Unified data schema between components"</li>
                <li>"Renovate: Automatic dependency updates"</li>
                <li>"(OpenTelemetry: Implementation in progress for tracing)"</li>
                <li>"(Authentication: Currently under consideration)"</li>
            </ul>
        </div>
    }
}
