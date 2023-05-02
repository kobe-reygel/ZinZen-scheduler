// test case to fix if (other.start <= self.start) && (other.end >= self.end)
// Code snippet: else if (other.start <= self.start) && (other.end >= self.end) {

use chrono::Duration;

use crate::{models::slot::Slot, tests::utils::get_slot};

#[test]
fn test_subtract_few_hours_from_fullday() {
    /*
    slot_few_hours = Slot {
        start: 2022-01-02T05:00:00,
        end: 2022-01-02T15:00:00,
    }
    slot_full_day = Slot {
        start: 2022-01-02T00:00:00,
        end: 2022-01-03T00:00:00,
    }
    */

    let year = 2022;
    let month = 1;
    let day = 1;

    let slot_few_hours = get_slot(Duration::hours(10), year, month, day, 5, 0);
    let slot_full_day = get_slot(Duration::days(1), year, month, day, 0, 0);

    let expected_result: Vec<Slot> = vec![
        get_slot(Duration::hours(5), year, month, day, 0, 0),
        get_slot(Duration::hours(9), year, month, day, 15, 0),
    ];
    dbg!(&expected_result);

    let result = slot_few_hours - slot_full_day;
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_subtract_fullday_from_few_hours() {
    /*
    slot_full_day = Slot {
        start: 2022-01-02T00:00:00,
        end: 2022-01-03T00:00:00,
    }
    slot_few_hours = Slot {
        start: 2022-01-02T05:00:00,
        end: 2022-01-02T15:00:00,
    }
    */

    let year = 2022;
    let month = 1;
    let day = 1;

    let slot_full_day = get_slot(Duration::days(1), year, month, day, 0, 0);
    let slot_few_hours = get_slot(Duration::hours(10), year, month, day, 5, 0);

    let expected_result: Vec<Slot> = vec![
        get_slot(Duration::hours(5), year, month, day, 0, 0),
        get_slot(Duration::hours(9), year, month, day, 15, 0),
    ];
    dbg!(&expected_result);

    let result = slot_full_day - slot_few_hours;
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_subtract_same_datetime() {
    let year = 2022;
    let month = 1;
    let day = 1;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let slot1 = get_slot(duration, year, month, day, hour, min);
    let slot2 = get_slot(duration, year, month, day, hour, min);

    let expected_result: Vec<Slot> = vec![];
    dbg!(&expected_result);

    let result = slot1 - slot2;
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_subtract_when_no_overlap() {
    let year = 2022;
    let month = 1;
    let day = 1;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let slot1 = get_slot(duration, year, month, day, hour, min);
    let slot2 = get_slot(duration, year, month, day + 1, hour, min);

    let expected_result: Vec<Slot> = vec![slot1];
    dbg!(&expected_result);

    let result = slot1 - slot2;
    dbg!(&result);

    assert_eq!(expected_result, result);
}

#[test]
fn test_is_conflicts_with() {
    let year = 2023;
    let month = 5;
    let day = 5;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let base_slot = get_slot(duration, year, month, day, hour, min);
    let conflicted_last_of_base = get_slot(duration, year, month, day, 09, min);
    let conflicted_start_of_base = get_slot(duration, year, month, day - 1, 20, min);
    let not_conflicted_with_base = get_slot(duration, year, month, day + 1, hour, min);
    dbg!(
        &base_slot,
        &conflicted_last_of_base,
        &conflicted_start_of_base,
        &not_conflicted_with_base
    );

    let is_conflicted_start_of_base = base_slot.is_conflicts_with(&conflicted_start_of_base);
    dbg!(&is_conflicted_start_of_base);

    let is_conflicted_last_of_base = base_slot.is_conflicts_with(&conflicted_last_of_base);
    dbg!(&is_conflicted_last_of_base);

    let is_not_conflicted_with_base = base_slot.is_conflicts_with(&not_conflicted_with_base);
    dbg!(&is_not_conflicted_with_base);

    // assert_eq!(expected_result, result);
    assert_eq!(true, is_conflicted_last_of_base);
    assert_eq!(true, is_conflicted_start_of_base);
    assert_eq!(false, is_not_conflicted_with_base);
}

#[test]
fn test_is_contains_slot() {
    let year = 2023;
    let month = 5;
    let day = 5;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let base_slot = get_slot(duration, year, month, day, hour, min);
    let contained_in_base = get_slot(Duration::hours(3), year, month, day, 02, min);
    let equal_to_base = get_slot(duration, year, month, day, hour, min);
    let overflow_base_from_start = get_slot(Duration::hours(3), year, month, day - 1, 23, min);
    let overflow_base_from_end = get_slot(Duration::hours(3), year, month, day, 09, min);
    let not_contained_in_base = get_slot(duration, year, month, day + 1, hour, min);
    dbg!(
        &base_slot,
        &contained_in_base,
        &equal_to_base,
        &overflow_base_from_start,
        &overflow_base_from_end,
        &not_contained_in_base
    );

    let is_contained_in_base = base_slot.is_contains_slot(&contained_in_base);
    dbg!(&is_contained_in_base);

    let is_equal_to_base_contained = base_slot.is_contains_slot(&equal_to_base);
    dbg!(&is_equal_to_base_contained);

    let is_overflow_base_from_start = base_slot.is_contains_slot(&overflow_base_from_start);
    dbg!(&is_overflow_base_from_start);

    let is_overflow_base_from_end = base_slot.is_contains_slot(&overflow_base_from_end);
    dbg!(&is_overflow_base_from_end);

    let is_not_contained_in_base = base_slot.is_contains_slot(&not_contained_in_base);
    dbg!(&is_not_contained_in_base);

    assert_eq!(true, is_contained_in_base);
    assert_eq!(true, is_equal_to_base_contained);
    assert_eq!(false, is_overflow_base_from_start);
    assert_eq!(false, is_overflow_base_from_end);
    assert_eq!(false, is_not_contained_in_base);
}

#[test]
fn test_is_intersect_with_slot() {
    let year = 2023;
    let month = 5;
    let day = 5;
    let hour: u32 = 0;
    let min: u32 = 0;
    let duration = Duration::hours(10);

    let base_slot = get_slot(duration, year, month, day, hour, min);
    let intersected_last_of_base = get_slot(duration, year, month, day, 09, min);
    let intersected_start_of_base = get_slot(duration, year, month, day - 1, 20, min);
    let not_intersected_with_base = get_slot(duration, year, month, day + 1, hour, min);
    dbg!(
        &base_slot,
        &intersected_last_of_base,
        &intersected_start_of_base,
        &not_intersected_with_base
    );

    let is_intersected_start_of_base = base_slot.is_intersect_with_slot(&intersected_start_of_base);
    dbg!(&is_intersected_start_of_base);

    let is_intersected_last_of_base = base_slot.is_intersect_with_slot(&intersected_last_of_base);
    dbg!(&is_intersected_last_of_base);

    let is_not_intersected_with_base = base_slot.is_intersect_with_slot(&not_intersected_with_base);
    dbg!(&is_not_intersected_with_base);

    // assert_eq!(expected_result, result);
    assert_eq!(true, is_intersected_last_of_base);
    assert_eq!(true, is_intersected_start_of_base);
    assert_eq!(false, is_not_intersected_with_base);
}
