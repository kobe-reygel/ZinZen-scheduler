use wasm_bindgen::prelude::*;
pub use goal::Goal;
pub use input::Input;
pub use output_formatter::{output_formatter, FinalOutput};
pub use repetition::Repetition;
use serde_wasm_bindgen::{from_value, to_value};
pub use slot::Slot;


mod errors;
/// API modules
mod goal;
pub mod input;
mod options_generator;
pub mod output_formatter;
mod repetition;
mod slot;
mod slot_generator;
mod task;
mod task_generator;
mod task_placer;
mod time_slot_iterator;
mod util;

// Test
#[cfg(test)]
mod unit_tests;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
interface Input {
    startDate: string;
    endDate: string;
    goals: number
}
"#;

// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn schedule(input: &JsValue) -> Result<JsValue, JsError> {
    use errors::Error;
    use output_formatter::*;
    use task_generator::task_generator;
    use task_placer::*;
    // JsError implements From<Error>, so we can just use `?` on any Error
    let input: Input = from_value(input.clone()).unwrap();
    let tasks = task_generator(input);
    let (scheduled_tasks, impossible_tasks) = task_placer(tasks);
    let output = match output_formatter(scheduled_tasks, impossible_tasks) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with task {title}:{id}. Tasks passed to output formatter should always have a confirmed_start/deadline.")
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    };

    Ok(to_value(&output)?)
}

pub fn run_scheduler(input: Input) -> FinalOutput {
    use errors::Error;
    use output_formatter::*;
    use task_generator::task_generator;
    use task_placer::*;
    let tasks = task_generator(input);
    let (scheduled_tasks, impossible_tasks) = task_placer(tasks);
    match output_formatter(scheduled_tasks, impossible_tasks) {
        Err(Error::NoConfirmedDate(title, id)) => {
            panic!("Error with task {title}:{id}. Tasks passed to output formatter should always have a confirmed_start/deadline.");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
        Ok(output) => output,
    }
}
