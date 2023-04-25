use crate::models::{
    goal::{Day, TimeFilter},
    slot::Slot,
    timeline::Timeline,
};

/// Applies time filter on the given timeline, then return filtered timeline
pub fn apply_filter(timeline: &Timeline, filter: &TimeFilter) -> Timeline {
    // Algorithm
    // - apply ontime filter (before_time and after_time fields)
    // - apply on_days filter
    // - apply not_on filter

    // - loop over timeline
    // -

    // filter_timing(timeline, before_time, after_time);

    todo!()
}

/// Filtering timeline based on before_time and after_time fields in TimeFilter
pub fn filter_timing(
    timeline: &Timeline,
    before_time: Option<usize>,
    after_time: Option<usize>,
) -> Timeline {
    todo!("filter_timing")
}

/// Filtering timeline based on on_days field in TimeFilter
pub fn filter_on_days(timeline: &Timeline, days: &Vec<Day>) -> Timeline {
    todo!("filter_on_days")
}

/// Filtering timeline based on not_on field in TimeFilter
pub fn filter_not_on(timeline: &Timeline, slots: &Vec<Slot>) -> Timeline {
    todo!("filter_not_on")
}
