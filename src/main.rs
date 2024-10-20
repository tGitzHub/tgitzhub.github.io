use yew::prelude::*;
use chrono::{Duration, Utc, DateTime};
use gloo::timers::callback::Interval;

#[function_component(Countdown)]
fn countdown() -> Html {
    let end_time = DateTime::parse_from_rfc3339("2024-10-21T11:00:00Z")
        .unwrap()
        .with_timezone(&Utc); // Adjust this for your target date
    let time_left = use_state(|| Duration::seconds((end_time - Utc::now()).num_seconds()));

    {
        let time_left = time_left.clone();
        use_effect(move || {
            let handle = Interval::new(1000, move || {
                let remaining = end_time - Utc::now();
                time_left.set(remaining);
            });
            || drop(handle)
        });
    }

    let days = time_left.num_days();
    let hours = (time_left.num_hours() % 24).to_string();
    let minutes = (time_left.num_minutes() % 60).to_string();
    let seconds = (time_left.num_seconds() % 60).to_string();

    html! {
        <div>
            <h1>{"Countdown to Factorio Space Age Update"}</h1>
            <p>{ format!("{} days {} hours {} minutes {} seconds", days, hours, minutes, seconds) }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<Countdown>::new().render();
}