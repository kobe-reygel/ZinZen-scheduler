#[cfg(feature = "generate-tests")]
use std::io::Write;
#[cfg(feature = "generate-tests")]
use std::path::PathBuf;

#[cfg(feature = "generate-tests")]
fn write_test(file: &mut std::fs::File, content: &str) -> Result<(), std::io::Error> {
    writeln!(file, "{}", content)?;
    Ok(())
}

#[cfg(feature = "generate-tests")]
fn get_test_dirs(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut dirs = std::fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    dirs.sort();

    Ok(dirs)
}
#[cfg(feature = "generate-tests")]
fn main() -> Result<(), std::io::Error> {
    let out_dir = String::from("./tests/");
    let mut result = vec!["".to_string()];

    result.push(get_run_test());
    result.push(create_tests_module());

    let mut rust_tests_file = std::fs::File::create(format!("{}/rust_tests.rs", out_dir))?;
    write_test(&mut rust_tests_file, result.join("\n").trim())?;
    Ok(())
}

#[cfg(not(feature = "generate-tests"))]
fn main() -> Result<(), std::io::Error> {
    Ok(())
}

#[cfg(feature = "generate-tests")]
fn get_run_test() -> String {
    include_str!("build_templates/run_test.rs").to_string()
}

#[cfg(feature = "generate-tests")]
fn get_test_fn_template(dir_name: &str, test_type: TestType) -> String {
    let test_name = dir_name.replace('-', "_");
    let mut test_fn_template: String = match test_type {
        TestType::Stable => include_str!("build_templates/test_fn_stable.rs").to_string(),
        TestType::Experimental => {
            include_str!("build_templates/test_fn_experimental.rs").to_string()
        }
    };

    test_fn_template = test_fn_template.replace("TEST_NAME", &test_name);
    test_fn_template = test_fn_template.replace("DIR_NAME", dir_name);

    test_fn_template
}

#[cfg(feature = "generate-tests")]
fn create_tests_module() -> String {
    // let mut result = vec!["".to_string()];
    let module_name = "e2e";
    let mut tests_mod = include_str!("build_templates/tests_mod.rs").to_string();

    tests_mod = tests_mod.replace("TEST_MODULE_NAME", module_name);
    tests_mod = tests_mod.replace(
        "//TEST_FUNCTIONS_STABLE",
        &create_test_functions("./tests/jsons/stable", TestType::Stable),
    );
    tests_mod = tests_mod.replace(
        "//TEST_FUNCTIONS_EXPERIMENTAL",
        &create_test_functions("./tests/jsons/experimental", TestType::Experimental),
    );

    tests_mod
}

#[cfg(feature = "generate-tests")]
fn create_test_functions(root_dir: &str, test_type: TestType) -> String {
    let mut dirs = get_test_dirs(root_dir).expect("Unable to read tests directory");
    let mut result = vec!["".to_string()];

    dirs.retain(|d| !d.file_name().unwrap().eq("benchmark-tests") && (d.is_dir()));

    for d in dirs.iter() {
        if let Ok(mut _dir) = d.read_dir() {
            result.push(get_test_fn_template(
                d.file_name().unwrap().to_str().unwrap(),
                test_type,
            ));
        }
    }

    result
        .into_iter()
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(feature = "generate-tests")]
#[derive(Clone, Copy)]
enum TestType {
    Stable,
    Experimental,
}
