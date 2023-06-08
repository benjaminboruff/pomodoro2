use crate::BreakTime;
use sycamore::prelude::*;

#[component]
pub fn BreakBottom<G: Html>(cx: Scope) -> View<G> {
    let break_time: &Signal<BreakTime> = use_context(cx);

    view! { cx,
        div {
            button(class="p-2 bg-red-400") { "-" }
            "Break: " (break_time.get().value()) " min"
            button(class="p-2 bg-red-400") { "+" }
        }
    }
}
