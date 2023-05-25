use crate::models::{
    slot::{Slot, SlotConflict},
    task::{Task, TaskStatus},
};

pub(crate) fn find_best_slots(tasks_to_place: &Vec<Task>) -> Option<Vec<Slot>> {
    // TODO 2023-05-25  \ Avoid spliting slots which causing wrong scheduling
    // Issued while debugging test case bug_215

    if tasks_to_place.is_empty() {
        return None;
    }

    let mut slot_conflicts: Vec<SlotConflict> = vec![];
    let task = &tasks_to_place[0];

    for slot in task.slots.iter() {
        for hour_slot in slot.divide_into_1h_slots() {
            let mut count: usize = 0;
            'outer: for t in tasks_to_place {
                if t.status != TaskStatus::ReadyToSchedule {
                    continue;
                }
                if t.id == task.id {
                    continue;
                }

                for s in t.slots.iter() {
                    if s.is_intersect_with_slot(&hour_slot) {
                        count += 1;
                        continue 'outer;
                    }
                }
            }
            slot_conflicts.push(SlotConflict {
                slot: hour_slot,
                num_conflicts: count,
            });
        }
    }
    slot_conflicts.sort_by(|a, b| b.slot.start.partial_cmp(&a.slot.start).unwrap());

    slot_conflicts.sort_by(|a, b| b.num_conflicts.partial_cmp(&a.num_conflicts).unwrap());

    let mut result = vec![];
    for _dur in 0..task.duration {
        match slot_conflicts.pop() {
            Some(s) => result.push(s.slot),
            None => break,
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate};

    use super::*;

    #[test]
    fn test_empty_tasks() {
        let tasks_to_place = Vec::new();
        assert_eq!(find_best_slots(&tasks_to_place), None);
    }

    #[test]
    fn test_unschedulable_task() {
        todo!("not implemented");

        // let tasks_to_place = vec![Task {
        //     id: 1,
        //     status: TaskStatus::Scheduled,
        //     duration: 2,
        //     slots: vec![Slot {
        //         start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //         end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //     }],
        // }];
        // assert_eq!(find_best_slots(&tasks_to_place), None);
    }

    /// Test single task
    /// Expected:
    /// TODO
    #[test]
    fn test_single_task() {
        /*
        Task {
            id: 4,
            goal_id: "4",
            title: "me time",
            duration: 1,
            status: ReadyToSchedule,
            flexibility: 168,
            start: None,
            deadline: None,
            slots: [
                Slot {
                    start: 2023-01-03T00:00:00,
                    end: 2023-01-10T00:00:00,
                },
            ],
            tags: [],
            after_goals: None,
        },
        */

        let slot = Slot::mock(Duration::days(6), 2023, 01, 03, 0, 0);

        let task = Task {
            id: 1,
            goal_id: "1".to_owned(),
            title: "me time".to_owned(),
            duration: 1,
            status: TaskStatus::ReadyToSchedule,
            flexibility: 168,
            start: None,
            deadline: None,
            slots: vec![slot],
            tags: vec![],
            after_goals: None,
        };

        let expected = Some(vec![Slot::mock(Duration::hours(1), 2023, 01, 03, 0, 0)]);
        dbg!(&expected);

        let result = find_best_slots(&vec![task]);
        dbg!(&result);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_tasks() {
        todo!("not implemented");

        // let tasks_to_place = vec![
        //     Task {
        //         id: 1,
        //         status: TaskStatus::ReadyToSchedule,
        //         duration: 2,
        //         slots: vec![Slot {
        //             start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //             end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //         }],
        //     },
        //     Task {
        //         id: 2,
        //         status: TaskStatus::ReadyToSchedule,
        //         duration: 1,
        //         slots: vec![Slot {
        //             start: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //             end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //         }],
        //     },
        //     Task {
        //         id: 3,
        //         status: TaskStatus::ReadyToSchedule,
        //         duration: 1,
        //         slots: vec![
        //             Slot {
        //                 start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //                 end: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //             },
        //             Slot {
        //                 start: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //                 end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //             },
        //         ],
        //     },
        // ];
        // let expected = Some(vec![
        //     Slot {
        //         start: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //         end: NaiveDate::from_ymd(2023, 5, 25).and_hms(12, 0, 0),
        //     },
        //     Slot {
        //         start: NaiveDate::from_ymd(2023, 5, 25).and_hms(10, 0, 0),
        //         end: NaiveDate::from_ymd(2023, 5, 25).and_hms(11, 0, 0),
        //     },
        // ]);
        // assert_eq!(find_best_slots(&tasks_to_place), expected);
    }
}
