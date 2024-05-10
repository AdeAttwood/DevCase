use cucumber::{given, then, World};

#[derive(Debug, Default, World)]
pub struct TestWorld {
    search: String,
    replace: String,
    input: String,
}

#[given(regex = r"^(Search|Replace|Input|Output) is '([^']+)'$")]
fn set_item(world: &mut TestWorld, input: String, output: String) {
    match input.as_str() {
        "Search" => world.search = output,
        "Replace" => world.replace = output,
        "Input" => world.input = output,
        _ => unreachable!(),
    }
}

#[then(regex = r"^Output is '([^']+)'$")]
fn assert_output(world: &mut TestWorld, expected: String) {
    let actual = dev_case::replace(&world.search, world.replace.clone(), world.input.clone());
    assert_eq!(actual, expected);
}

fn main() {
    futures::executor::block_on(TestWorld::run("tests/features/basic.feature"));
}
