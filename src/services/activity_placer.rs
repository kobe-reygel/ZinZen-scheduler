use std::rc::Rc;

use crate::models::{
    activity::{Activity, ActivityType, Status},
    calendar::{Calendar, Hour, ImpossibleActivity},
};

pub fn place(calendar: &mut Calendar, mut activities: Vec<Activity>) {
    loop {
        for activity in activities.iter_mut() {
            activity.update_overlay_with(&calendar.budgets);
        }
        let act_index_to_schedule = find_act_index_to_schedule(&activities);
        if act_index_to_schedule.is_none() {
            println!("Tried to schedule activity index None");
            break;
        }
        if activities[act_index_to_schedule.unwrap()].goal_id.len() > 5 {
            println!(
                "Next to schedule: {:?} {:?}",
                &activities[act_index_to_schedule.unwrap()].title,
                &activities[act_index_to_schedule.unwrap()].goal_id[0..5]
            );
        } else {
            println!(
                "Next to schedule: {:?} {:?}",
                &activities[act_index_to_schedule.unwrap()].title,
                &activities[act_index_to_schedule.unwrap()].goal_id
            );
        }
        let best_hour_index_and_size: Option<(usize, usize)> =
            activities[act_index_to_schedule.unwrap()].get_best_scheduling_index_and_length();
        let best_hour_index: usize;
        let best_size: usize;
        if best_hour_index_and_size.is_some() {
            best_hour_index = best_hour_index_and_size.unwrap().0;
            best_size = best_hour_index_and_size.unwrap().1;
            println!(
                "Best index:{:?} and size {:?}",
                &best_hour_index, &best_size
            );
        } else {
            activities[act_index_to_schedule.unwrap()].release_claims();
            if activities[act_index_to_schedule.unwrap()].activity_type == ActivityType::Budget {
                activities[act_index_to_schedule.unwrap()].status = Status::Processed;
                continue;
            } else {
                activities[act_index_to_schedule.unwrap()].status = Status::Impossible;
            }
            let impossible_activity = ImpossibleActivity {
                id: activities[act_index_to_schedule.unwrap()].goal_id.clone(),
                hours_missing: activities[act_index_to_schedule.unwrap()].duration_left,
                period_start_date_time: calendar.start_date_time,
                period_end_date_time: calendar.end_date_time,
            };
            calendar.impossible_activities.push(impossible_activity);
            continue;
        }
        println!("reserving {:?} hours...", best_size);
        for duration_offset in 0..best_size {
            Rc::make_mut(&mut calendar.hours[best_hour_index + duration_offset]);
            calendar.hours[best_hour_index + duration_offset] = Rc::new(Hour::Occupied {
                activity_index: act_index_to_schedule.unwrap(),
                activity_title: activities[act_index_to_schedule.unwrap()].title.clone(),
                activity_goalid: activities[act_index_to_schedule.unwrap()].goal_id.clone(),
            });
            //TODO: activity doesn't need to know about time_budets => remove completely
            calendar.update_budgets_for(
                &activities[act_index_to_schedule.unwrap()].goal_id.clone(),
                best_hour_index + duration_offset,
            );
            activities[act_index_to_schedule.unwrap()].duration_left -= 1;
        }
        if activities[act_index_to_schedule.unwrap()].duration_left == 0 {
            activities[act_index_to_schedule.unwrap()].status = Status::Scheduled;
            (activities[act_index_to_schedule.unwrap()]).release_claims();
        }

        dbg!(&calendar);
    }
    dbg!(&calendar);
}

fn find_act_index_to_schedule(activities: &[Activity]) -> Option<usize> {
    let mut act_index_to_schedule = None;
    for index in 0..activities.len() {
        if activities[index].status == Status::Scheduled
            || activities[index].status == Status::Impossible
            || activities[index].status == Status::Processed
        {
            continue;
        }
        match act_index_to_schedule {
            None => act_index_to_schedule = Some(index),
            Some(_) => match activities[index].flex() {
                0 => {
                    println!("Found activity index {:?} with flex 0...", &index);
                    continue;
                }
                1 => {
                    if activities[act_index_to_schedule.unwrap()].flex() == 1 {
                        break;
                    } else {
                        act_index_to_schedule = Some(index);
                        break;
                    }
                }
                _ => {
                    if activities[act_index_to_schedule.unwrap()].flex() < activities[index].flex()
                    {
                        act_index_to_schedule = Some(index);
                    }
                }
            },
        }
    }
    act_index_to_schedule
}
