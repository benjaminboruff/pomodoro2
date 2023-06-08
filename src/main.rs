use components::timer::Timer;
use sycamore::prelude::*;

mod components;

struct SessionTime(u8);
impl SessionTime {
    fn value(&self) -> u8 {
        self.0
    }
}
struct BreakTime(u8);
impl BreakTime {
    fn value(&self) -> u8 {
        self.0
    }
}

struct Countdown((u8, bool));
impl Countdown {
    fn time(&self) -> u8 {
        self.0 .0
    }
    fn session(&self) -> bool {
        self.0 .1
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // state
    let session_time = create_signal(cx, SessionTime(25));
    provide_context_ref(cx, session_time);
    let break_time = create_signal(cx, BreakTime(5));
    provide_context_ref(cx, break_time);
    let countdown_time = create_signal(cx, Countdown((25, true)));
    provide_context_ref(cx, countdown_time);

    view! { cx,
        Timer {}
    }
}

fn main() {
    sycamore::render(|cx| {
        view! {cx,
            div(class="container mx-auto") {
                App {}
            }
        }
    });
}
