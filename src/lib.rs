#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;

pub use sentry;

pub type Result<T> = Result<T, Error>;

custom_derive! {
	/// Just use this type as return error type for your handler functions (ex: `Result<String, sentry_rocket::Error>`)
	/// and after running the usual `let _guard = sentry::init(...);`
	/// and it will send any error to sentry.
	/// Note that you should use the `failure` crate to propagate your errors if you wish to have proper backtrace handling.
	/// Also, using `sentry_rocket::sentry` instead of `sentry` will ensure the versions match so that everything works properly
	/// (also, then you don't need `sentry` as a dependency for your project, only `sentry_rocket`)
	#[derive(NewtypeFrom, NewtypeDeref, NewtypeDebug)]
	pub struct Error(failure::Error);
}

impl<'r> rocket::response::Responder<'r> for Error {
	fn respond_to(self, req: &rocket::Request) -> rocket::response::Result<'r> {
		// Extract all error information
		let mut event = sentry::integrations::failure::event_from_error(&self);
		// Add request information
		event.request = Some(sentry::protocol::Request {
			url: format!(
				"scheme://{}{}", // how do I get scheme from request ?
				req.headers().get_one("Host").unwrap_or("<no-host-header>"),
				req.uri()
			)
			.parse()
			.ok(),
			method: Some(req.method().as_str().into()),
			headers: req
				.headers()
				.iter()
				.map(|h| (h.name().into(), h.value().into()))
				.collect(),
			..Default::default()
		});
		let uuid = sentry::capture_event(event); // Send event to sentry
		if uuid.is_nil() {
			panic!("Could not send request event to Sentry. Make sure you are using matching sentry versions. Consider using sentry_rocket::sentry instead of sentry to make sure versions do match")
		}
		Err(rocket::http::Status::InternalServerError)
	}
}
