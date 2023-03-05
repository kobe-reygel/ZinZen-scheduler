use self::time_filter::TimeFilter;
use crate::repetition::Repetition;
use crate::slot::Slot;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDateTime;

pub mod time_filter;

/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
pub(crate) struct TimeSlotsIterator {
    timeline: Vec<Slot>,
    repetition: Option<Repetition>,
    filters: Vec<TimeFilter>,
}

impl TimeSlotsIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Vec<TimeFilter>,
    ) -> TimeSlotsIterator {
        let mut result = TimeSlotsIterator {
            timeline: vec![Slot {
                start: start,
                end: end,
            }],
            repetition,
            filters,
        };
        result.apply_filters();
        result
    }

    fn apply_filters(&mut self) {
        for filter in self.filters.iter() {
            match filter.filter_type {
                time_filter::FilterType::After => {
                    let mut result: Vec<Slot> = vec![];
                    for slot in self.timeline.iter_mut() {
                        let mut daily_slots = slot.divide_in_days();
                        for daily_slot in daily_slots.iter_mut() {
                            let after_datetime = daily_slot
                                .start
                                .with_hour(filter.after_time as u32)
                                .unwrap();
                            if daily_slot.start < after_datetime {
                                daily_slot.start = after_datetime;
                            }
                            result.push(*daily_slot);
                        }
                    }
                    self.timeline = result;
                }
                time_filter::FilterType::Before => todo!(),
                time_filter::FilterType::Weekdays => todo!(),
                time_filter::FilterType::Weekends => todo!(),
                time_filter::FilterType::Mondays => todo!(),
                time_filter::FilterType::Tuesdays => todo!(),
                time_filter::FilterType::Wednesdays => todo!(),
                time_filter::FilterType::Thursdays => todo!(),
                time_filter::FilterType::Fridays => todo!(),
                time_filter::FilterType::Saturdays => todo!(),
                time_filter::FilterType::Sundays => todo!(),
            }
        }
    }
}

impl Iterator for TimeSlotsIterator {
    type Item = Vec<Slot>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.repetition {
            Some(repetition) => match repetition {
                Repetition::DAILY(_) => todo!(),
                Repetition::HOURLY => todo!(),
                Repetition::Weekly(_) => todo!(),
                Repetition::WEEKDAYS => todo!(),
                Repetition::WEEKENDS => todo!(),
                Repetition::EveryXdays(_) => todo!(),
                Repetition::EveryXhours(repeat_hours) => {
                    let result = vec![];
                    let next_start_position = self.timeline[0]
                        .start
                        .checked_add_signed(Duration::hours(repeat_hours as i64))
                        .unwrap();
                    for slot in self.timeline.iter_mut() {
                        if next_start_position.lt(&slot.end) {
                            let result = Slot {
                                start: slot.start,
                                end: next_start_position,
                            };
                            slot.start = next_start_position;
                            return Some(result);
                        } else if next_start_position.eq(&slot.end) {
                            self.timeline.remove(0);
                        }
                    }
                }
                Repetition::MONDAYS => todo!(),
                Repetition::TUESDAYS => todo!(),
                Repetition::WEDNESDAYS => todo!(),
                Repetition::THURSDAYS => todo!(),
                Repetition::FRIDAYS => todo!(),
                Repetition::SATURDAYS => todo!(),
                Repetition::SUNDAYS => todo!(),
                Repetition::FlexDaily(_, _) => todo!(),
                Repetition::FlexWeekly(_, _) => todo!(),
            },
            None => {
                if self.timeline.len() > 0 {
                    let result = Some(self.timeline[0].clone());
                    self.timeline.remove(0);
                    return result;
                } else {
                    return None;
                };
            }
        }
    }
}

fn hour_is_within_bounds(after_time: usize, before_time: usize, hour: usize) -> bool {
    if before_time < after_time {
        hour < before_time || hour >= after_time
    } else {
        hour >= after_time && hour < before_time
    }
}

fn skip_to_after_time(mut original_time: NaiveDateTime, after_time: usize) -> NaiveDateTime {
    while original_time.hour() < after_time as u32 {
        original_time += Duration::hours(1);
    }
    original_time
}
