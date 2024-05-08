use snafu::prelude::*;
use snafu::Backtrace;
use windows::core::Error;

pub type Result<T, E = AwnError> = std::result::Result<T, E>;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum AwnError {
    #[snafu(display("failed to send key {key}"))]
    KeyDownSendFailed {
        key: String,
        backtrace: Backtrace,
    },
    #[snafu(display("Application not found: pcwstr_title {title_converted}, raw_title {title_raw}"))]
    ApplicationNotFound {
        title_converted: String,
        title_raw: String,
    },
    #[snafu(display("win32 api {name} error {win_err:?}"))]
    Win32APIError {
        name: String,
        #[snafu(source)]
        win_err: Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Unknown error"))]
    Unknown {
        backtrace: Backtrace,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use snafu::ensure;
    use snafu::ErrorCompat;

    #[test]
    fn test_snafu_backtrace() {
        fn ensure() -> Result<()> {
            ensure!(false, UnknownSnafu);
            Ok(())
        }
        // test ensure
        if let Err(e) = ensure() {
            if let Some(bt) = ErrorCompat::backtrace(&e) {
                println!("{bt:?}");
            }
        };
    }
}