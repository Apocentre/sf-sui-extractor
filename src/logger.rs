pub trait Logger {
  fn log(&self, msg: &str);
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
  fn log(&self, msg: &str) {
    println!("{}", msg);
  }
}
