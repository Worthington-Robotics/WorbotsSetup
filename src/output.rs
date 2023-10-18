use color_print::cprintln;

/// Common trait for output from both the CLI and app
pub trait OutputTrait {
	/// Display a progress message
	fn progress(&mut self, msg: impl AsRef<str>);
	/// Display a success message
	fn success(&mut self, msg: impl AsRef<str>);
	/// Display an instruction message
	fn instruction(&mut self, msg: impl AsRef<str>);
	/// Display a prompt to continue
	fn continue_prompt(&mut self);
}

/// Output object for both the CLI and app
pub type Output = CommonOutput;

/// CLI output
pub struct CommonOutput;

impl OutputTrait for CommonOutput {
	fn progress(&mut self, msg: impl AsRef<str>) {
		crate::utils::print_progress(&msg);
	}

	fn success(&mut self, msg: impl AsRef<str>) {
		cprintln!("<s,g>{}", msg.as_ref());
	}

	fn instruction(&mut self, msg: impl AsRef<str>) {
		cprintln!("<s>{}", msg.as_ref());
	}

	fn continue_prompt(&mut self) {
		crate::utils::continue_prompt();
	}
}

/// Null output
pub struct NullOutput;

impl OutputTrait for NullOutput {
	fn progress(&mut self, msg: impl AsRef<str>) {
		let _ = msg;
	}

	fn success(&mut self, msg: impl AsRef<str>) {
		let _ = msg;
	}

	fn instruction(&mut self, msg: impl AsRef<str>) {
		let _ = msg;
	}

	fn continue_prompt(&mut self) {}
}
