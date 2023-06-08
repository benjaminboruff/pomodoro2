use crate::SessionTime;
use sycamore::prelude::*;

#[component]
pub fn SessionTop<G: Html>(cx: Scope) -> View<G> {
    let session_time: &Signal<SessionTime> = use_context(cx);
    view! {cx,
        div {
            button(class="p-2 bg-green-400") { "-" }
            "Session: " (session_time.get().value()) " min"
            button(class="p-2 bg-green-400") { "+" }
        }
    }
}
