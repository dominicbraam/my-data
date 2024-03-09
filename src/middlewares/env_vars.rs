use std::env;
use dotenvy::dotenv;
use std::fmt::Debug;
use std::str::FromStr;

// https://stackoverflow.com/questions/66171718/load-env-and-convert-it-to-struct-genericly-with-helper-function
pub fn get_env_var<T>(env_var_name: &str, default_value: Option<T>) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    // sets env vars from .env file
    // good for dev but for prod, opt for setting the vars directly
    dotenv().ok();

    match env::var(env_var_name).is_ok() {
        true => {
            let env_value = env::var(env_var_name).unwrap();
            env_value.parse::<T>().unwrap()
        },
        false => {
            default_value.expect("Environment variable needs a value")
        }
    }
}
