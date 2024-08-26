use leptos::*;

/// Renders an input element with a given `title` and `setter` 
#[component]
fn Input(
    setter: WriteSignal<String>,
) -> impl IntoView {
    
    view! {
        <input type="text"
            // event_target_value is similar to e.target.value in JS:
            on:input=move |ev| {
                setter(event_target_value(&ev))
            }

            // `prop:` updates a dom property rather than an attribute.
            />
        }
}

/// Renders the main application.
#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <Input setter=set_name />
        <p>{name}</p>
        }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
