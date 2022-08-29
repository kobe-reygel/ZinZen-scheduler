use crate::task::Task;
use crate::date_range::DateRange;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::option::Option;

/// How often can a task repeat
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Repetition {
	#[serde(rename = "daily")]
	DAILY,
}

impl From<Repetition> for Duration {
	fn from(a: Repetition) -> Self {
		match a {
			Repetition::DAILY => Duration::days(1),
		}
	}
}

/// A [Goal] is what one wants to do, it is used in conjunction with a span of time to generate a [Schedule]
#[derive(Deserialize, Debug, Default)]
pub struct Goal {
	/// Every goal has a unique ID
	pub id: usize,
	/// A goal's description
	pub title: String,
	/// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
	pub duration: usize,
	pub repetition: Option<Repetition>,
	/// Earliest start datetime for this Goal's Tasks
	#[serde(default)]
	pub start: Option<NaiveDateTime>,
	/// Deadline for this Goal's Tasks
	#[serde(default)]
	pub deadline: Option<NaiveDateTime>,
}

//#[cfg(test)]
impl Goal {
	pub fn new(id: usize) -> Self {
		Self {
			id,
			title: String::from("Test"),
			..Default::default()
		}
	}

	pub fn duration(mut self, duration: usize) -> Self {
		self.duration = duration;
		self
	}

	pub fn repetition(mut self, repetition: Repetition) -> Self {
		self.repetition = Some(repetition);
		self
	}

	pub fn start(mut self, start: NaiveDateTime) -> Self {
		self.start = Some(start);
		self
	}

	pub fn deadline(mut self, deadline: NaiveDateTime) -> Self {
		self.deadline = Some(deadline);
		self
	}

	pub fn generate_tasks(self, calendar_start: NaiveDateTime, calendar_end: NaiveDateTime) -> Vec<Task> {
		let mut tasks = Vec::new();
		match self.repetition {
            //If there is a repetion, split up the calendar_start - calendar_end by the repetition,
            //and generate a task for each block. e.g. if the repetition is daily, a task will be
            //generated for each day.
			Some(rep) => {
				let date_range = DateRange {
					start: calendar_start,
					end: calendar_end,
					interval: Some(Duration::from(rep)),
				};
				let mut i = 0;
				for (start,deadline) in date_range {
					let task_id = format!("{}{}", self.id, i);
					let t = Task::new(
						task_id.parse::<usize>().unwrap(),
						self.id,
						self.title.clone(),
						self.duration,
						start,
						deadline,
					);
					tasks.push(t);
					i = i + 1;
				}
			}
            //If there is no repetition, the task's start and deadline are equivalent to the goal's
            //start/deadline, or the calendar start/deadline if no goal start/deadline exists.
			None => {
				let task_id = format!("{}{}", self.id, 0);
				let t = Task::new(
					task_id.parse::<usize>().unwrap(),
					self.id,
					self.title,
					self.duration,
					self.start.unwrap_or(calendar_start),
					self.deadline.unwrap_or(calendar_end),
				);
				tasks.push(t);
			}
		}
		tasks
	}
}
