/// Unwrap or exit with the specified message.
pub trait Expected<T>
where
    Self: Sized,
{
    /// Test the value. On success, return the unwrapped value.
    /// On error, display the specified message, and exit.
    fn or_die(self, msg: &str) -> T;
}

impl<T> Expected<T> for Option<T> {
    fn or_die(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => exit(msg),
        }
    }
}

fn exit<T>(msg: &str) -> T {
    eprintln!("{}", msg);
    std::process::exit(1);
}

impl<T, E> Expected<T> for Result<T, E> {
    fn or_die(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => exit(msg),
        }
    }
}
