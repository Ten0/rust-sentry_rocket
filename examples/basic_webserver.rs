#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use sentry_rocket::sentry;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[get("/fail")]
fn fail() -> Result<(), sentry_rocket::Error> {
	something_that_may_fail()?;
	Ok(())
}

fn something_that_may_fail() -> Result<(), failure::Error> {
	let some_result = Err(std::io::Error::new(
		std::io::ErrorKind::Other,
		"Something failed",
	));
	Ok(some_result?)
}

fn main() {
	let _guard = sentry::init("https://a94ae32be2584e0bbd7a4cbb95971fee@sentry.io/1041156");
	std::env::set_var("RUST_BACKTRACE", "1");
	sentry::integrations::panic::register_panic_handler();

	rocket::ignite().mount("/", routes![index, fail]).launch();
}
