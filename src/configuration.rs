use std::env;

pub struct Configuration {
    pub key: String,
    pub secret: String,
    pub port: u16,
}

impl Configuration {
    pub fn new() -> Result<Configuration, String> {
        let key = Configuration::get_env_value("VASTTRAFIK_KEY")?;
        let secret = Configuration::get_env_value("VASTTRAFIK_SECRET")?;
        let port = match Configuration::get_env_value("PORT") {
            Ok(value) => match value.parse::<u16>() {
                Ok(number) => number,
                Err(_) => {
                    println!("PORT env is not a number -> Defaulting to port 8000");
                    8000
                }
            },
            Err(_) => 8000,
        };
        Ok(Configuration { key, secret, port })
    }

    fn get_env_value(env_variable: &str) -> Result<String, String> {
        match env::var(env_variable) {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("{} environment variable is not set.", env_variable)),
        }
    }
}
