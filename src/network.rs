use std::{
	io::{Error, ErrorKind, Result},
	process::Command,
	thread::sleep,
	time::{Duration, Instant},
};

fn scutil(verb: &str, service: &str) -> Result<String> {
	let out = Command::new("/usr/sbin/scutil").args(["--nc", verb, service]).output()?;

	if !out.status.success() {
		return Err(Error::new(ErrorKind::Other, format!("scutil --nc {verb} {service:?} failed: {}", out.status)));
	}

	Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn scutil_status(service: &str) -> Result<String> {
	scutil("status", service)
}

pub fn reconnect_and_wait(service: &str) -> Result<()> {
	// Stop + start
	let _ = scutil("stop", service);
	let _ = scutil("start", service);

	let timeout = Duration::from_secs(15);
	let start = Instant::now();
	loop {
		let s = scutil_status(service)?.to_lowercase();

		// Typical status output starts with "Connected" / "Disconnected" etc.
		if s.starts_with("connected") {
			return Ok(());
		}

		if start.elapsed() > timeout {
			return Err(Error::new(
				ErrorKind::TimedOut,
				format!("VPN did not reach Connected state within {timeout:?}. Last status: {s:?}"),
			));
		}

		sleep(Duration::from_millis(250));
	}
}
