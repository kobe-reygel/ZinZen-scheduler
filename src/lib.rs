//! # ZinZen scheduler
//!
//! The ZinZen scheduler is a "calendar as a function".  
//! Input: A calendar start datetime and end datetime, plus a Directed Acyclical Graph of Goals/Budgets with time constraints.  
//! Output: A calendar that successfully allocates all Goals - or the maximum amount of Goals in that time period.  
//!
//! ```
//! use scheduler::scheduler
//!
//!     let json_input: serde_json::Value = serde_json::json!({
//!       "TODO_working_example"
//!     });
//!     let input: Input = serde_json::from_value(json_input).unwrap();
//!     let output = scheduler::run_scheduler(input);
//! ```
//!
//! ## Getting Started
//! This project is hosted on [Github](https://github.com/tijlleenders/ZinZen-scheduler). The Docs.rs / Crates.io version is probably (far) behind.  
//! Please submit an issue there if you've found something we need to improve or have a question regarding how things work.
//!
//! For more explanation, see the crate documentation.
//! There are no features to configure.
//!
//!
//!
//! ## Special Considerations
//!
//! We're not at 1.0 major version yet.  
//! Expect breaking changes for every minor (y in 0.x.y) release!
//!
//! ## Contributing
//!
//! Read the standard [Contributor Covenant Code of Conduct](https://github.com/tijlleenders/ZinZen-scheduler/blob/main/CONTRIBUTING.md).  
//! **TL;DR : Be nice.**  
//! We also use the principles of Robert C. Martin's 'Clean Code' - nicely summarized [on this Gist](https://gist.github.com/wojteklu/73c6914cc446146b8b533c0988cf8d29).  
//! If you find documentation missing, this is considered a bug, so please submit a bug report!
//!
//! ## License and legal stuff
//!
//! AGPL 3.0 See [LICENSE](LICENSE) file.
//!
//! &copy;2020-now ZinZen&reg;
//!
//! This code is licensed under AGPLv3 but this license does not implicitly grant
//! permission to use the trade names, trademarks, service marks, or product names
//! of the licensor, except as required for reasonable and customary use in
//! describing the origin of the Work and the content of the notice/attribution
//! file.
//!
//! ZinZen&reg; supports an open and collaborative process. Registering the
//! ZinZen&reg; trademark is a tool to protect the ZinZen&reg; identity and the
//! quality perception of the ZinZen&reg; projects.

use wasm_bindgen::prelude::*;
/// The data structures
pub mod models;

// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
/// The main wasm function to call
// #[wasm_bindgen]
// pub fn schedule(input: &JsValue) -> Result<JsValue, JsError> {
//     console_error_panic_hook::set_once();
//     let input = serde_wasm_bindgen::from_value(input.clone())?;
//     let final_tasks = run_scheduler();
//     Ok(serde_wasm_bindgen::to_value(&final_tasks)?)
// }

pub fn run_scheduler(start_date: String, end_date: String) -> () {

    // let date_start = &input;
    // let date_end = DateTime::from_naive_date_time(&input.calendar_end);
    // let goals = get_goals(&input);

    // let calendar = Calendar::new(&input, &goals);

    // while !calendar.has_finished_scheduling() {
    //     log::info!("\n{calendar:?}");

    //     #[derive(PartialEq)]
    //     enum Handling {
    //         DoNothing,
    //         Flexibility1,
    //         MostFlexibility,
    //         Impossible,
    //     }

    //     // determine flexibility
    //     // (Handling marker, flexibility measure, position in the calender unproccessed vector)
    //     let mut handling: (Handling, i32, Option<usize>) = (Handling::DoNothing, 0, None);
    //     let mut unprocessed: RefCell<Vec<usize>> = RefCell::new(vec![]);

    //     log::info!(
    //         "selected position in unprocesse vec of calendar {:?}",
    //         handling.2,
    //     );

    //     // calculate placement
    // }
    // log::info!("\n{calendar:?}");

    // calendar.result()
}
