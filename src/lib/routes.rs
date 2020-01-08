use crate::lib::config::CONFIG;

#[get("/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    if CONFIG.debug {
        println!("this!")
    }

    format!("Hello, {} year old named {}!", age, name)
}
