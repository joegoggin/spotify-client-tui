use color_eyre::eyre::eyre;
use dirs::home_dir;
use log::error;

use crate::core::app::AppResult;

pub fn get_home_dir() -> AppResult<String> {
    match home_dir() {
        Some(home_dir) => Ok(format!("{}", home_dir.display())),
        None => {
            let error_message = "Unable to find home directory.";

            error!("{}", error_message);
            Err(eyre!(error_message))
        }
    }
}
