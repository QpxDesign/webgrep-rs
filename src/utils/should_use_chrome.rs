use std::env;

pub fn should_use_chrome(chrome_arg_value: Option<bool>) -> bool {
    if chrome_arg_value.is_some() && chrome_arg_value.unwrap() {
        return true;
    }
    let key = "WEBGREP_ALWAYS_USE_CHROME";
    match env::var(key) {
        Ok(val) => {
            if val == "1" {
                return true;
            } else {
                return false;
            }
        }
        Err(e) => {
            return false;
        }
    }
}
