use leptos::{html::Input, *};
use leptos_router::*;
use web_sys::SubmitEvent;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <ul>
                    <li>
                        <a href="/">Home</a>
                    </li>
                    <li>
                        <a href="/raw-tutorial">Tutorial</a>
                    </li>
                    <li>
                        <A href="users">Users</A>
                    </li>
                    <li>
                        <A href="users/titusmoore">Titus</A>
                    </li>
                </ul>
            </nav>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/raw-tutorial" view=TutorialComponent />
                <Route path="/users" view=Users>
                    <Route path=":id" view=SingleUser />
                    <Route path="" view=|| view! {
                        <p>"Hello From Users"</p>
                    }/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! {
        <div>
            <h1>"This is the users component"</h1>
            <Outlet />
        </div>
    }
}

#[component]
fn SingleUser() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let name = move || match id().as_str() {
        "titusmoore" => "Titus",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <div>
            <h1>"This is the users id component " {name}</h1>
        </div>
    }
}

#[component]
fn ChildButton(change_parent_value: WriteSignal<u32>) -> impl IntoView {
    let on_click = move |_| {
        change_parent_value.update(move |x| *x += 1);
    };

    view! {
        <button on:click=on_click>
            "Increment parent value"
        </button>
    }
}

#[component]
fn TestingParentElement<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! {
        <div>
            {render_prop()}
            <ul>
                {children}
            </ul>
        </div>
    }
}

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
fn TutorialComponent() -> impl IntoView {
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

    let (parent_value, set_parent_value) = create_signal(10);

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
        <div>
            "This is the parent value: "{parent_value}
            <ChildButton change_parent_value=set_parent_value />
        </div>
        <TestingParentElement render_prop={|| view! { <p>"This is from the render prop"</p> }}>
            <p>"This is a child 1"</p>
            <p>"This is a child 2"</p>
            <p>"This is a child 3"</p>
            <p>"This is a child 4"</p>
        </TestingParentElement>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let navigate = leptos_router::use_navigate();

    // Redirect when count hits 10
    create_effect(move |_| {
        if count() == 10 {
            navigate("/raw-tutorial", Default::default());
        }
    });

    view! {
        <div>
            <h1>"Home"</h1>
            <div>"Current Count: "{count}". Will Redirect at 10"</div>
            <button on:click=move |_| {
                set_count.update(|x| *x += 1);
            }>"Increment Count"</button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
