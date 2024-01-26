fn greeting(name: String) -> String {
    let hello = String::from("Hello, ");
    let greeting = format!("{hello}{name}!");
    greeting
}

fn hello_world() -> String {
    String::from("Hello, World!")
}

#[test]
fn hello_world_test() {
    let want = String::from("Hello, World!");
    let result = hello_world();
    assert_eq!(want, result);
}
#[test]
fn greeting_test() {
    let want = String::from("Hello, Rusty!");
    let name = String::from("Rusty");
    let result = greeting(name);
    assert_eq!(want, result);
}
