use axum::{debug_handler, routing::post, Json, Router};
use chrono::{Duration, Local};
use good_morning_lib::calendar::BasicEvent;
use ical_property::DateMaybeTime;

#[debug_handler]
async fn ical_proxy(body: String) -> Result<Json<Vec<BasicEvent>>, String> {
    let ical_url = body;
    if !ical_url.starts_with("https://calendar.google.com/calendar/ical") {
        return Err("Wrong prefix.".to_string());
    }
    let calendar_raw = reqwest::get(&ical_url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    let mut events = Vec::new();
    for cal in ical::IcalParser::new(calendar_raw.as_bytes()) {
        let cal = cal.map_err(|e| e.to_string())?;
        for event in cal.events {
            events.push(event);
        }
    }
    let all_events = good_morning_lib::calendar::filter_map_events(events.into_iter());
    let events_today =
        good_morning_lib::calendar::events_at_date(all_events, Local::now().date_naive());
    let mut events: Vec<BasicEvent> = Vec::new();
    for e in events_today.into_iter() {
        events.push(BasicEvent {
            summary: e.summary.ok_or("Event without summray".to_string())?,
            time: match e.start.ok_or("Event without start".to_string())? {
                // Look. Idk why the time offset is here. But it is.
                DateMaybeTime::DateTime(dt) => Some(dt.naive_local().time() + Duration::hours(1)),
                DateMaybeTime::Date(_) => None,
            },
        });
    }
    Ok(Json(events))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    let app = Router::new().route("/", post(ical_proxy));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
