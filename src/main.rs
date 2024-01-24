use leptos::{html::Input, *};
use web_sys::{Event, SubmitEvent};

/// Displays a progress bar towards a complete value.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 50)]
    max: u32,
    /// The current value of the progress bar.
    #[prop(into)]
    progress: Signal<u32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let progress = move || count() * 2;
    let my_values = vec!["Foo", "Bar", "Baz"];
    let mut tracking_len = 5;
    let dyn_items = (0..5)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (items, set_items) = create_signal(dyn_items);
    let (name, set_name) = create_signal("".to_string());
    let (uncontrolled, set_uncontrolled) = create_signal("".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let (conditional_display, set_conditional_display) = create_signal(false);

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        let value = input_element().expect("<input> to exist").value();
        set_uncontrolled(value);
    };

    let add_item = move |_| {
        set_items.update(|items| {
            items.push((tracking_len, create_signal(tracking_len + 1)));
        });

        tracking_len += 1;
    };

    let conditional_message = move || {
        if conditional_display() {
            Some("Conditional display is on")
        } else {
            None
        }
    };

    let (numeric, set_numeric) = create_signal(Ok(0));
    let on_numeric_input = move |ev| {
        set_numeric(event_target_value(&ev).parse::<u32>());
    };

    view! {
            <ProgressBar progress=count />
            <ProgressBar progress=Signal::derive(progress) />
            <button
                on:click=move |_| {
                    set_count.update(|x| *x += 1);
                }
                class:red=move || count.get() % 2 == 0
            >
                "Click me: "{count}
            </button>
            <ul>
                {my_values
                .into_iter()
                .map(|x| view! { <li>{x}</li> })
                .collect::<Vec<_>>()}
            </ul>
        <ul>
            <For
            each=items
            key=|counter| counter.0
            children=move |(_id, (count, set_count))| {
                view! {
                    <li>
                        <button on:click=move |_| {
                        set_count.update(|x| *x += 1);
                        }>
                        {count}
                        </button>
                    </li>
                }
            }
            />
        </ul>
        <button on:click=add_item>
        "Add item"
        </button>
        <input on:input=move |e| {
            set_name(event_target_value(&e));
        }
        prop:value=name />
        <p>"Hello "{name}"!"</p>
        <form on:submit=on_submit>
            <input type="text" value=uncontrolled node_ref=input_element />
            <input type="submit" value="Submit" />
        </form>
        <p>"Uncontrolled value: "{uncontrolled}"!"</p>
        <p>{conditional_message}</p>
        <button on:click=move |_| {
            set_conditional_display(!conditional_display());
        }>
        "Toggle conditional display"
        </button>
        <div>
            <input type="text" on:input=on_numeric_input placeholder="Numeric" />
            <p>"This is just raw numeric value: "{numeric}</p>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div>
                        <p>"Something went wrong!"</p>
                        <ul>
                            {move || errors.get()
                                        .into_iter()
                                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                        .collect_view()}
                        </ul>
                    </div>
                }>
                <p>"This is numeric value with error boundary: "{numeric}</p>
            </ErrorBoundary>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
