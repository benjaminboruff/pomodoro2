use crate::BreakTime;
use sycamore::prelude::*;

#[component]
pub fn BreakBottom<G: Html>(cx: Scope) -> View<G> {
    let break_time: &Signal<BreakTime> = use_context(cx);
    let handle_dec = || {
        let current_break_time = break_time.get().value();
        if current_break_time > 1 {
            break_time.set(BreakTime(break_time.get().value() - 1));
        } else {
            break_time.set(BreakTime(1));
        }
    };
    let handle_inc = || break_time.set(BreakTime(break_time.get().value() + 1));

    view! { cx,
        div {
            button(on:click=move |_| handle_dec(), class="p-2 bg-red-400") { "-" }
            "Break: " (break_time.get().value()) " min"
            button(on:click=move |_| handle_inc(), class="p-2 bg-red-400") { "+" }
        }
    }
}
