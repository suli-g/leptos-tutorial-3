use ev::SubmitEvent;
use leptos::*;

/// Renders the main application.
#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("UnControlled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element()
            .expect("<input> should be mounted")
            .value();
        set_name(value);
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit" />
        </form>
        <p>{name}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
