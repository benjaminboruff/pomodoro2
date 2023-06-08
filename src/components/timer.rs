use crate::components::break_footer::BreakFooter;
use crate::components::session_header::SessionHeader;
use crate::{BreakTime, Countdown, SessionTime};
use sycamore::prelude::*;

#[component]
pub fn Timer<G: Html>(cx: Scope) -> View<G> {
    let countdown_timer: &Signal<Countdown> = use_context(cx);
    let session_time: &Signal<SessionTime> = use_context(cx);
    let break_time: &Signal<BreakTime> = use_context(cx);
    let handle_start_stop = || {
        if countdown_timer.get().session() {
            countdown_timer.set(Countdown((session_time.get().value(), true)));
        } else {
            countdown_timer.set(Countdown((break_time.get().value(), false)));
        }
    };
    view! { cx,
        div {
            SessionHeader {}
            div(class="p-4") {
                "Timer: " (countdown_timer.get().time())
                br {}
                button(on:click=move |_| handle_start_stop() , class="bg-blue-400") { " Start/Stop "}
            }
            BreakFooter {}
        }
    }
}
