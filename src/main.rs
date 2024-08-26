use ev::SubmitEvent;
use leptos::*;

/// Renders a select tag with string options.
#[component]
fn Select(items: Vec<String>) -> impl IntoView {
    let data = create_memo(move |_| items.clone());
    view! {
        <select>
            <For
                each=data
                key=|item| item.clone()
                children=move |item: String|{ view! {
                        <option
                            value=item.clone()
                            let:data
                        >
                            {item}
                        </option>
                }}
            />
        </select>
    }
}

/// Renders the main application.
#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("UnControlled".to_string());
    let (value, set_value) = create_signal(0i32);
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("<input> should be mounted").value();
        set_name(value);
    };
    let items = vec!["Hello", "World!"];
    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_value(new_value.parse().unwrap());
                }
                prop:value=move || value().to_string()
            >
                <option value="0">"Option 0"</option>
                <option value="1">"Option 1"</option>
                <option value="2">"Option 2"</option>
            </select>
            <button on:click=move |_| set_value.update(|n| {
                if *n == 2 {
                    *n = 0;
                } else {
                    *n += 1;
                }
            })>
            "Next Option"
            </button>
            <input type="submit" value="Submit" />
        </form>
        <Select items=items.iter().map(|x| x.to_string()).collect() />
        <p>{name}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
