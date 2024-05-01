pub trait Logger {
  fn log(msg: &str);
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
  fn log(msg: &str) {
    println!("{}", msg);
  }
}
