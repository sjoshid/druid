use std::path::Path;
use std::time::SystemTime;
use chrono::Local;
use druid::tracing::{TraceFilter, FilterEvents};

pub fn main() {
    let opts = FilterEvents::WindowConnected | FilterEvents::WindowSize;
    let filter = TraceFilter::create_filter();
}
