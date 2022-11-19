use serde::de::{self, Visitor};
use serde::Deserialize;
use serde::*;
use std::fmt;

/// How often a goal repeats.
/// Textual descriptions of a repetition from the front-end
/// (e.g. "4/week" or "mondays") are converted into this enum
/// via a custom serde deserializer.
/// This enum is used by the Goal struct for it's "repeat" field, to
/// determine how many tasks to generate from a goal.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repetition {
    DAILY(usize),
    HOURLY,
    WEEKLY(usize),
    WEEKDAYS,
    WEEKENDS,
    EveryXdays(usize),
    EveryXhours(usize),
    MONDAYS,
    TUESDAYS,
    WEDNESDAYS,
    THURSDAYS,
    FRIDAYS,
    SATURDAYS,
    SUNDAYS,
}

//How to implement serde deserialize: https://serde.rs/impl-deserialize.html
struct RepetitionVisitor;

impl<'de> Visitor<'de> for RepetitionVisitor {
    type Value = Repetition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string that follows the zinzen json schema either daily, hourly, weekly, mondays etc."
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match s {
            "daily" => Ok(Repetition::DAILY(1)),
            "hourly" => Ok(Repetition::HOURLY),
            "weekly" => Ok(Repetition::WEEKLY(1)),
            "weekdays" => Ok(Repetition::WEEKDAYS),
            "weekends" => Ok(Repetition::WEEKENDS),
            "mondays" => Ok(Repetition::MONDAYS),
            "tuesdays" => Ok(Repetition::TUESDAYS),
            "wednesdays" => Ok(Repetition::WEDNESDAYS),
            "thursdays" => Ok(Repetition::THURSDAYS),
            "fridays" => Ok(Repetition::FRIDAYS),
            "saturdays" => Ok(Repetition::SATURDAYS),
            "sundays" => Ok(Repetition::SUNDAYS),
            _ => {
                if s.contains('/') {
                    //e.g. '4/week'
                    let split = s.split('/').collect::<Vec<&str>>();
                    let num = split[0]
                        .parse::<usize>()
                        .expect("front end should use format x/period");
                    match split[1] {
                        "week" => Ok(Repetition::WEEKLY(num)),
                        "day" => Ok(Repetition::DAILY(num)),
                        _ => panic!("unrecognized repetition: {}", s),
                    }
                } else if s.contains(' ') {
                    //e.g. 'every 5 days'
                    let split = s.split(' ').collect::<Vec<&str>>();
                    let num = split[1]
                        .parse::<usize>()
                        .expect("front end should use format 'every x days' or 'every x hours' ");
                    let rep = split[2];
                    if rep == "days" {
                        Ok(Repetition::EveryXdays(num))
                    } else if rep == "hours" {
                        Ok(Repetition::EveryXhours(num))
                    } else {
                        panic!("front end should use format 'every x days' or 'every x hours' ");
                    }
                } else {
                    Err(E::custom("Error deserializing goal"))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for Repetition {
    fn deserialize<D>(deserializer: D) -> Result<Repetition, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(RepetitionVisitor)
    }
}

//The main reason Display is being implemented for repetition
// is so that the string representation of Repetition::MONDAYS-SUNDAYS matches the
//string representation of chrono::Weekday(). This makes it easy in the TimeSlotIterator to do
//If self.start.weekday().to_string() == self.repetition.unwrap().to_string().
//see: https://docs.rs/chrono/latest/src/chrono/weekday.rs.html#141
impl fmt::Display for Repetition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Repetition::DAILY(_) => "DAILY",
            Repetition::HOURLY => "HOURLY",
            Repetition::WEEKLY(_) => "WEEKLY",
            Repetition::WEEKDAYS => "WEEKDAYS",
            Repetition::WEEKENDS => "WEEKENDS",
            Repetition::EveryXdays(_) => "EveryXdays",
            Repetition::EveryXhours(_) => "EveryXhours",
            Repetition::MONDAYS => "Mon",
            Repetition::TUESDAYS => "Tue",
            Repetition::WEDNESDAYS => "Wed",
            Repetition::THURSDAYS => "Thu",
            Repetition::FRIDAYS => "Fri",
            Repetition::SATURDAYS => "Sat",
            Repetition::SUNDAYS => "Sun",
        })
    }
}
