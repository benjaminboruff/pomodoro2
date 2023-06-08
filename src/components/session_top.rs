use crate::SessionTime;
use sycamore::prelude::*;

#[component]
pub fn SessionTop<G: Html>(cx: Scope) -> View<G> {
    let session_time: &Signal<SessionTime> = use_context(cx);
    let handle_dec = || {
        let current_session_time = session_time.get().value();
        if current_session_time > 1 {
            session_time.set(SessionTime(session_time.get().value() - 1));
        } else {
            session_time.set(SessionTime(1));
        }
    };
    let handle_inc =
        || session_time.set(SessionTime(session_time.get().value() + 1));

    view! {cx,
        div {
            button(on:click=move |_| handle_dec(), class="p-2 bg-green-400") { "-" }
            "Session: " (session_time.get().value()) " min"
            button(on:click=move |_| handle_inc(), class="p-2 bg-green-400") { "+" }
        }
    }
}
