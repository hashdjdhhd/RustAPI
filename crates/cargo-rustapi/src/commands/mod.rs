//! CLI commands

mod add;
mod docs;
mod doctor;
mod generate;
mod new;
mod run;
mod watch;

pub use add::{add, AddArgs};
pub use docs::open_docs;
pub use doctor::{doctor, DoctorArgs};
pub use generate::{generate, GenerateArgs};
pub use new::{new_project, NewArgs};
pub use run::{run_dev, RunArgs};
pub use watch::{watch, WatchArgs};
