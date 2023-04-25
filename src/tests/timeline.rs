use crate::{
    models::{
        slot::Slot,
        timeline::{Timeline, TimelineOperations},
    },
    tests::utils::*,
};
use chrono::Duration;
use std::collections::BTreeSet;

#[test]
fn test_initialize() {
    let sample_slot = get_slot(Duration::hours(15), 2022, 10, 1, 05, 0);

    let expected_slot_in_timeline = Slot {
        start: sample_slot.start,
        end: sample_slot.end,
    };
    let mut expected_collection_in_timeline = BTreeSet::new();
    expected_collection_in_timeline.insert(expected_slot_in_timeline);
    let expected_timeline = Timeline {
        slots: vec![expected_slot_in_timeline].into_iter().collect(),
    };

    if let Some(timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        assert_eq!(expected_timeline, timeline);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_from() {
    let sample_slot = get_slot(Duration::hours(15), 2022, 10, 1, 05, 0);

    if let Some(mut timeline) = Timeline::initialize(sample_slot.start, sample_slot.end) {
        let slot_to_remove = get_slot(Duration::hours(5), 2022, 10, 1, 05, 0);
        dbg!(&slot_to_remove);

        if let Some(()) = timeline.remove_slots(vec![slot_to_remove]) {
            let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();
            let expected_result = vec![get_slot(Duration::hours(10), 2022, 10, 1, 10, 0)];
            dbg!(&expected_result);
            dbg!(&result);

            assert_eq!(expected_result, result);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_get_next() {
    let (slot1, slot2, slot3, slot4) = (
        get_slot(Duration::hours(2), 2022, 10, 1, 01, 0),
        get_slot(Duration::hours(3), 2022, 10, 1, 03, 0),
        get_slot(Duration::hours(4), 2022, 10, 1, 07, 0),
        get_slot(Duration::hours(10), 2022, 10, 1, 12, 0),
    );
    dbg!(&(slot1, slot2, slot3, slot4));
    let timeline = Timeline {
        slots: vec![slot1, slot2, slot3, slot4].into_iter().collect(),
    };
    dbg!(&timeline);

    if let Some(next_slot) = timeline.get_slot(1) {
        dbg!(&next_slot);
        assert_eq!(slot2, next_slot);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_halfday_from_fullday() {
    let slot_fullday = get_slot(Duration::hours(24), 2022, 10, 1, 00, 0);
    dbg!(&slot_fullday);
    let mut timeline = Timeline {
        slots: vec![slot_fullday].into_iter().collect(),
    };
    dbg!(&timeline);

    let slot_halfday_night = get_slot(Duration::hours(12), 2022, 10, 1, 12, 0);
    let slot_halfday_morning = get_slot(Duration::hours(12), 2022, 10, 1, 0, 0);
    let expected_result = vec![slot_halfday_morning];

    dbg!(&slot_halfday_night);
    dbg!(&expected_result);

    if let Some(()) = timeline.remove_slots(vec![slot_halfday_night]) {
        let result: Vec<Slot> = timeline.slots.clone().into_iter().collect();
        dbg!(&result);

        assert_eq!(expected_result, result);
    } else {
        assert!(false);
    }
}

#[test]
fn test_remove_afternoon_hours_from_fullday() {
    let slot_fullday = get_slot(Duration::hours(24), 2022, 10, 1, 00, 0);
    let mut timeline_fullday = Timeline {
        slots: vec![slot_fullday].into_iter().collect(),
    };

    let slot_afternoon = get_slot(Duration::hours(3), 2022, 10, 1, 12, 0);

    let expected_result = vec![
        get_slot(Duration::hours(12), 2022, 10, 1, 0, 0),
        get_slot(Duration::hours(9), 2022, 10, 1, 15, 0),
    ];

    dbg!(&timeline_fullday);
    dbg!(&slot_afternoon);
    dbg!(&expected_result);

    if let Some(()) = timeline_fullday.remove_slots(vec![slot_afternoon]) {
        let result: Vec<Slot> = timeline_fullday.slots.clone().into_iter().collect();
        dbg!(&result);

        assert_eq!(expected_result, result);
    } else {
        assert!(false);
    }
}
