#![allow(non_snake_case)]
#[cfg(feature = "server")]
use axum::Router;
use dioxus::logger::tracing::{info, warn, error};
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
    #[route("/")]
    HomePage,
}

#[component]
fn HomePage() -> Element {
    rsx! {
        h1 { "Dioxus Form Submission Test" }
        p { "This example demonstrates the difference between EventHandler from struct methods vs inline closures." }
        ProblematicForm {}
        hr {}
        WorkingForm {}
        hr {}
        DebugForm {}
    }
}

#[component]
fn SomeForm(on_submit: EventHandler<FormEvent>, children: Element) -> Element {
    rsx! {
        form {
            onsubmit: on_submit,
            {children}
        }
    }
}

#[derive(Clone)]
struct ProblematicFormState {
    message: Signal<String>,
    submit_count: Signal<u32>,
}

impl ProblematicFormState {
    fn new() -> Self {
        Self {
            message: use_signal(|| "Ready to submit (problematic)".to_string()),
            submit_count: use_signal(|| 0),
        }
    }

    fn handle_submit_problematic(&self) -> EventHandler<FormEvent> {
        let mut message = self.message;
        let mut submit_count = self.submit_count;

        EventHandler::new(move |ev: FormEvent| {
            info!("🔴 PROBLEMATIC: Event handler called - about to prevent_default()");

            // Log the event details
            info!("🔴 PROBLEMATIC: Calling prevent_default()...");

            ev.prevent_default(); // THIS SHOULD PREVENT THE PAGE RELOAD BUT DOESN'T

            info!("🔴 PROBLEMATIC: prevent_default() called successfully");

            let current_count = submit_count() + 1;
            submit_count.set(current_count);
            message.set(format!("Problematic form submitted {} times - prevent_default() called but page may still reload!", current_count));

            info!("🔴 PROBLEMATIC: State updated, spawning async task...");

            spawn(async move {
                info!("🔴 PROBLEMATIC: Async task started");
                // Simulate some async work
                some_long_running_task().await.expect("TODO: panic message");
                info!("🔴 PROBLEMATIC: Async task completed");
            });

            info!("🔴 PROBLEMATIC: Event handler completed");
        })
    }
}

#[component]
fn ProblematicForm() -> Element {
    let state = ProblematicFormState::new();

    rsx! {
        div {
            style: "border: 2px solid red; padding: 15px; margin: 10px 0; border-radius: 5px;",
            h2 { "🔴 Scenario A: Problematic Form (EventHandler from struct method)" }
            p { 
                style: "color: red; font-weight: bold;",
                "Status: {state.message}"
            }
            p { 
                style: "font-size: 0.9em; color: #666;",
                "This form uses EventHandler::new() returned from a struct method. Even though prevent_default() is called, the page will reload."
            }
            SomeForm {
                on_submit: state.handle_submit_problematic(),
                input { 
                    r#type: "text", 
                    name: "data_a", 
                    placeholder: "Enter some data...",
                    style: "margin: 5px; padding: 5px;"
                },
                button { 
                    r#type: "submit",
                    style: "margin: 5px; padding: 8px 15px; background: #ff4444; color: white; border: none; border-radius: 3px;",
                    "Submit Problematic Form"
                }
            }
        }
    }
}

#[derive(Clone)]
struct WorkingFormState {
    message: Signal<String>,
    submit_count: Signal<u32>,
}

impl WorkingFormState {
    fn new() -> Self {
        Self {
            message: use_signal(|| "Ready to submit (working)".to_string()),
            submit_count: use_signal(|| 0),
        }
    }

    fn do_submit_working(&self) {
        let mut message = self.message;
        let mut submit_count = self.submit_count;

        info!("🟢 WORKING: do_submit_working called");

        let current_count = submit_count() + 1;
        submit_count.set(current_count);
        message.set(format!("Working form submitted {} times - no page reload!", current_count));

        spawn(async move {
            info!("🟢 WORKING: Async task started");
            some_long_running_task().await.expect("TODO: panic message");
            info!("🟢 WORKING: Async task completed");
        });
    }
}

#[component]
fn WorkingForm() -> Element {
    let state = WorkingFormState::new();

    rsx! {
        div {
            style: "border: 2px solid green; padding: 15px; margin: 10px 0; border-radius: 5px;",
            h2 { "🟢 Scenario B: Working Form (inline closure)" }
            p { 
                style: "color: green; font-weight: bold;",
                "Status: {state.message}"
            }
            p { 
                style: "font-size: 0.9em; color: #666;",
                "This form uses an inline closure. It prevents the default behavior without explicitly calling prevent_default()."
            }
            SomeForm {
                on_submit: move |ev: FormEvent| {
                    info!("🟢 WORKING: Inline closure called");
                    info!("🟢 WORKING: NOT calling prevent_default() - but it still works!");
                    state.do_submit_working();
                    info!("🟢 WORKING: Inline closure completed");
                },
                input { 
                    r#type: "text", 
                    name: "data_b", 
                    placeholder: "Enter some data...",
                    style: "margin: 5px; padding: 5px;"
                },
                button { 
                    r#type: "submit",
                    style: "margin: 5px; padding: 8px 15px; background: #44aa44; color: white; border: none; border-radius: 3px;",
                    "Submit Working Form"
                }
            }
        }
    }
}

// Additional debug form to test different approaches
#[derive(Clone)]
struct DebugFormState {
    message: Signal<String>,
    submit_count: Signal<u32>,
}

impl DebugFormState {
    fn new() -> Self {
        Self {
            message: use_signal(|| "Ready to debug".to_string()),
            submit_count: use_signal(|| 0),
        }
    }

    fn handle_submit_with_explicit_prevent(&self) -> EventHandler<FormEvent> {
        let mut message = self.message;
        let mut submit_count = self.submit_count;

        EventHandler::new(move |ev: FormEvent| {
            warn!("🟡 DEBUG: Explicit prevent_default() in EventHandler::new");

            // Try calling prevent_default() at the very beginning
            ev.prevent_default();
            info!("🟡 DEBUG: prevent_default() called immediately");

            let current_count = submit_count() + 1;
            submit_count.set(current_count);
            message.set(format!("Debug form submitted {} times", current_count));

            info!("🟡 DEBUG: Completed without async");
        })
    }
}

#[component]
fn DebugForm() -> Element {
    let state = DebugFormState::new();

    rsx! {
        div {
            style: "border: 2px solid orange; padding: 15px; margin: 10px 0; border-radius: 5px;",
            h2 { "🟡 Scenario C: Debug Form (EventHandler with immediate prevent_default)" }
            p { 
                style: "color: orange; font-weight: bold;",
                "Status: {state.message}"
            }
            p { 
                style: "font-size: 0.9em; color: #666;",
                "This form tests EventHandler::new() with prevent_default() called immediately, without async operations."
            }
            SomeForm {
                on_submit: state.handle_submit_with_explicit_prevent(),
                input { 
                    r#type: "text", 
                    name: "data_c", 
                    placeholder: "Debug data...",
                    style: "margin: 5px; padding: 5px;"
                },
                button { 
                    r#type: "submit",
                    style: "margin: 5px; padding: 8px 15px; background: #ff8800; color: white; border: none; border-radius: 3px;",
                    "Submit Debug Form"
                }
            }
        }
    }
}

#[server(SomeLongRunningTask)]
pub async fn some_long_running_task() -> Result<(), ServerFnError<String>> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    Ok(())
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    info!("🚀 Starting Dioxus fullstack server");

    let address = dioxus::cli_config::fullstack_address_or_localhost();
    info!("📡 Server will listen on: {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let dioxus_serve_config = ServeConfigBuilder::default().build().unwrap();
    let dioxus_app_router = Router::new()
        .serve_dioxus_application(dioxus_serve_config, App);

    info!("✅ Server configured, starting to serve...");
    axum::serve(listener, dioxus_app_router.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    info!("🚀 Starting Dioxus client application");
    launch(App);
}