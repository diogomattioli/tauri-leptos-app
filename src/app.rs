use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;

use crate::tauri_command;

tauri_command!(fn greet<'a>(name: &'a str) -> String);

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            set_greet_msg.set(greet(&name).await.unwrap_or_default());
        });
    };

    view! {
        <script>
            window.addEventListener("contextmenu", (e) => {
                const allowedTypes = ["INPUT", "TEXTAREA"];
                if (!allowedTypes.includes(e.target.nodeName)) {
                    e.preventDefault();
                }
            });
        </script>
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
