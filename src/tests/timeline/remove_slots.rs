use crate::models::{slot::Slot, timeline::Timeline};
use chrono::Duration;

#[test]
fn test_remove_from() {
    let sample_slot = Slot::mock(Duration::hours(15), 2022, 10, 1, 5, 0);

    if let Some(mut timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        let slot_to_remove = Slot::mock(Duration::hours(5), 2022, 10, 1, 5, 0);

        timeline.remove_slots(vec![slot_to_remove]);
        let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();
        let expected_result = vec![Slot::mock(Duration::hours(10), 2022, 10, 1, 10, 0)];

        assert_eq!(expected_result, result);
    } else {
        panic!();
    }
}

#[test]
fn test_remove_halfday_from_fullday() {
    let slot_fullday = Slot::mock(Duration::hours(24), 2022, 10, 1, 00, 0);
    let mut timeline = Timeline {
        slots: vec![slot_fullday].into_iter().collect(),
    };

    let slot_halfday_night = Slot::mock(Duration::hours(12), 2022, 10, 1, 12, 0);
    let slot_halfday_morning = Slot::mock(Duration::hours(12), 2022, 10, 1, 0, 0);
    let expected_result = vec![slot_halfday_morning];

    timeline.remove_slots(vec![slot_halfday_night]);
    let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();

    assert_eq!(expected_result, result);
}

#[test]
fn test_remove_afternoon_hours_from_fullday() {
    let slot_fullday = Slot::mock(Duration::hours(24), 2022, 10, 1, 00, 0);
    let mut timeline_fullday = Timeline {
        slots: vec![slot_fullday].into_iter().collect(),
    };

    let slot_afternoon = Slot::mock(Duration::hours(3), 2022, 10, 1, 12, 0);

    let expected_result = vec![
        Slot::mock(Duration::hours(12), 2022, 10, 1, 0, 0),
        Slot::mock(Duration::hours(9), 2022, 10, 1, 15, 0),
    ];

    timeline_fullday.remove_slots(vec![slot_afternoon]);
    let result: Vec<Slot> = timeline_fullday.slots.clone().into_iter().collect();

    assert_eq!(expected_result, result);
}

#[test]
fn test_based_on_i284_7days() {
    // Test based on failed test: issue-284-filter-days-of-week-7days

    let year: i32 = 2023;
    let month: u32 = 3;
    let day: u32 = 9;
    let start_hour = 8;
    let end_hour = 12;
    let duration = Duration::hours(end_hour - start_hour as i64);

    let slots: Vec<Slot> = vec![
        Slot::mock(duration, year, month, day, start_hour, 0),
        Slot::mock(duration, year, month, day + 1, start_hour, 0),
        Slot::mock(duration, year, month, day + 2, start_hour, 0),
        Slot::mock(duration, year, month, day + 3, start_hour, 0),
        Slot::mock(duration, year, month, day + 4, start_hour, 0),
        Slot::mock(duration, year, month, day + 5, start_hour, 0),
        Slot::mock(duration, year, month, day + 6, start_hour, 0),
        Slot::mock(duration, year, month, day + 7, start_hour, 0),
        Slot::mock(duration, year, month, day + 8, start_hour, 0),
        Slot::mock(duration, year, month, day + 9, start_hour, 0),
        Slot::mock(duration, year, month, day + 10, start_hour, 0),
    ];

    let mut timeline = Timeline {
        slots: slots.clone().into_iter().collect(),
    };

    let expected_result: Vec<Slot> = slots;

    timeline.remove_slots(vec![]);
    let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();

    assert_eq!(expected_result, result);
}

/// Test based on edge case in funciton filter_not_on when timeline have many slots and
/// have many slots to filter
/// - timeline: 5 days (Starting Mon 2023-05-01 to Fri 2023-05-05)
/// - slots_to_filter: 2023-05-02 00 to 05 and 2023-05-04 13 to 17
/// - Expected list of all 5 days except first 5 hours of 2023-05-02 and
/// except hours from 13 to 17 in 2023-05-04
#[test]
fn test_based_on_edge_case_in_filter_not_on() {
    let slots_to_filter: Vec<Slot> = vec![
        Slot::mock(Duration::hours(5), 2023, 5, 2, 0, 0),
        Slot::mock(Duration::hours(4), 2023, 5, 4, 13, 0),
    ];

    let mut timeline = Timeline::mock_as_days(5, 2023, 5, 1);

    let expected_result: Timeline = Timeline {
        slots: vec![
            Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
            Slot::mock(Duration::hours(19), 2023, 5, 2, 5, 0),
            Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
            Slot::mock(Duration::hours(13), 2023, 5, 4, 0, 0),
            Slot::mock(Duration::hours(7), 2023, 5, 4, 17, 0),
            Slot::mock(Duration::days(1), 2023, 5, 5, 0, 0),
        ]
        .into_iter()
        .collect(),
    };

    timeline.remove_slots(slots_to_filter);

    assert_eq!(expected_result, timeline);
}

/// Test based on edge case when asking to remove many slots same day
/// - timeline: 5 days (Starting Mon 2023-05-01 to Fri 2023-05-05)
/// - slots_to_filter:
///     - 2023-05-02 00 to 05
///     - 2023-05-02 20 to 22
///     - 2023-05-04 13 to 17
/// - Expected list
///     - 2023-05-01 full day
///     - 2023-05-02 05 to 20
///     - 2023-05-02 22 to 00
///     - 2023-05-03 full day
///     - 2023-05-04 00 to 13
///     - 2023-05-04 17 to 00
///     - 2023-05-05 full day
#[test]
fn test_many_filters_same_day() {
    let slots_to_filter: Vec<Slot> = vec![
        Slot::mock(Duration::hours(5), 2023, 5, 2, 0, 0),
        Slot::mock(Duration::hours(2), 2023, 5, 2, 20, 0),
        Slot::mock(Duration::hours(4), 2023, 5, 4, 13, 0),
    ];

    let mut timeline = Timeline::mock_as_days(5, 2023, 5, 1);

    let expected_result: Timeline = Timeline {
        slots: vec![
            Slot::mock(Duration::days(1), 2023, 5, 1, 0, 0),
            Slot::mock(Duration::hours(15), 2023, 5, 2, 5, 0),
            Slot::mock(Duration::hours(2), 2023, 5, 2, 22, 0),
            Slot::mock(Duration::days(1), 2023, 5, 3, 0, 0),
            Slot::mock(Duration::hours(13), 2023, 5, 4, 0, 0),
            Slot::mock(Duration::hours(7), 2023, 5, 4, 17, 0),
            Slot::mock(Duration::days(1), 2023, 5, 5, 0, 0),
        ]
        .into_iter()
        .collect(),
    };

    timeline.remove_slots(slots_to_filter);

    assert_eq!(expected_result, timeline);
}
