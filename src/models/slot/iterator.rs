use super::Slot;
use chrono::{Duration, NaiveDateTime};

/// Iterator for a `Slot` and provide some functionalities like count per duration interval
#[derive(Debug, Clone)]
pub struct SlotIterator {
    slot: Slot,
    pointer: NaiveDateTime,
    /// Duration interval for pointer to corss over slot
    interval: Duration,
}
impl SlotIterator {
    /// Initialize new SlotIterator with default interval duration to 1 day
    pub fn initialize(slot: Slot) -> SlotIterator {
        SlotIterator {
            slot,
            pointer: slot.start,
            interval: Duration::days(1),
        }
    }

    /// Create new SlotIterator with custom interval duration
    pub fn new(slot: Slot, interval_duration: Duration) -> SlotIterator {
        SlotIterator {
            slot,
            pointer: slot.start,
            interval: interval_duration,
        }
    }

    /// Get count between slot.start and slot.end based on Duration interval
    /// Example: if duration interval is 1 min, so it will count minutes
    ///  between slot.start and slot.end
    pub fn interval_count(&self) -> usize {
        let mut count = 0;
        self.clone().for_each(|_| count += 1);
        count
    }
}

impl Iterator for SlotIterator {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.slot.end {
            return None;
        }
        let next_pointer = self.pointer + self.interval;

        let slot = Slot {
            start: self.pointer,
            end: next_pointer,
        };
        self.pointer = next_pointer;

        Some(slot)
    }
}

//test case for Slot Iterator impl
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use chrono::NaiveDateTime;

    /// Testing walk through a slot from 5 days and iterate over each day
    #[test]
    fn test_walk_through_by_day() {
        let slot_duration = Duration::days(5);
        let interval_duration = Duration::days(1);

        let slot = Slot::mock(slot_duration, 2023, 05, 1, 0, 0);
        dbg!(&slot);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::days(1), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 2, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 3, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 4, 0, 0),
            Slot::mock(Duration::days(1), 2023, 05, 5, 0, 0),
        ];
        dbg!(&expected_result);

        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let mut result: Vec<Slot> = vec![];

        for slot in slot_iterator {
            result.push(slot);
        }
        dbg!(&expected_result, &result);

        assert_eq!(expected_result, result);
    }

    /// Testing walk through a slot from 5 days and iterate over each day
    #[test]
    fn test_walk_through_by_hours() {
        let slot_duration = Duration::hours(5);
        let interval_duration = Duration::hours(1);

        let slot = Slot::mock(slot_duration, 2023, 05, 1, 0, 0);
        dbg!(&slot);

        let expected_result: Vec<Slot> = vec![
            Slot::mock(Duration::hours(1), 2023, 05, 1, 0, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 1, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 2, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 3, 0),
            Slot::mock(Duration::hours(1), 2023, 05, 1, 4, 0),
        ];
        dbg!(&expected_result);

        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let mut result: Vec<Slot> = vec![];

        for slot in slot_iterator {
            result.push(slot);
        }
        dbg!(&expected_result, &result);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_interval_count_as_interval_duration_1_day() {
        let interval_duration = Duration::days(1);
        let expected_count: usize = 4;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let result = slot_iterator.interval_count();
        dbg!(&result);
        assert_eq!(expected_count, result);
    }

    #[test]
    fn test_interval_count_as_interval_duration_1_mintue() {
        let interval_duration = Duration::minutes(1);
        let expected_count: usize = 5760;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let result = slot_iterator.interval_count();
        dbg!(&result);

        assert_eq!(expected_count, result);
    }

    #[test]
    fn test_interval_count_as_interval_duration_1_hour() {
        let interval_duration = Duration::hours(1);
        let expected_count: usize = 96;

        let slot = Slot {
            start: NaiveDateTime::parse_from_str("2023-04-26T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end: NaiveDateTime::parse_from_str("2023-04-30T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        };
        let slot_iterator = SlotIterator::new(slot, interval_duration);
        dbg!(&slot, &slot_iterator);

        let result = slot_iterator.interval_count();
        dbg!(&result);

        assert_eq!(expected_count, result);
    }
}
