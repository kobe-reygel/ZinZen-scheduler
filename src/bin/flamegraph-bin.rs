extern crate scheduler;

/// To generate a flamegraph of the scheduler on your machine, follow the platform-specific instructions [here](https://github.com/flamegraph-rs/flamegraph).
/// If you're running inside WSL2 you'll probably need to follow https://gist.github.com/abel0b/b1881e41b9e1c4b16d84e5e083c38a13
/// Then `cargo flamegraph --bin flamegraph`
fn main() {
    // let path = Path::new("./tests/jsons/stable/algorithm-challenge/input.json");
    // let input = get_input_from_json(path).unwrap();
    // let _output = scheduler::run_scheduler(input);
}

// pub fn get_input_from_json<P: AsRef<Path>>(path: P) -> Result<&JsValue, Box<dyn Error>> {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let input = serde_json::from_reader(reader)?;
//     Ok(input)
// }
