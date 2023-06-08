use crate::components::break_bottom::BreakBottom;
use crate::components::session_top::SessionTop;
use crate::CountDown;
use sycamore::prelude::*;

#[component]
pub fn Timer<G: Html>(cx: Scope) -> View<G> {
    let countdown_timer: &Signal<CountDown> = use_context(cx);
    view! { cx,
        div {
            SessionTop {}
            div(class="p-4") {
                "Timer: " (countdown_timer.get().value())
                br {}
                button(class="bg-blue-400") { " Start/Stop "}
            }
            BreakBottom {}
        }
    }
}
