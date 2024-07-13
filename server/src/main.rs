#[macro_use]
extern crate rocket;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[get("/expensiveCalc")]
fn expensive_calc() -> String {
    let result = fibonacci(40);
    format!("Fibonacci(40) = {}", result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![expensive_calc])
}
