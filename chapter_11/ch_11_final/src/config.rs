use clap::Parser;
use std::env;

/// Q&A 웹 서비스 API
#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// 로깅할 에러 수준(info, warn, error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// 서버가 대기할 PORT
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// 데이터베이스 사용자
    #[clap(long, default_value = "user")]
    pub db_user: String,
    #[clap(long, default_value = "password")]
    pub db_password: String,
    /// Postgres 데이터베이스 URL
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// 데이터베이스 연결 포트 번호
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// 데이터베이스 이름
    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        let config = Config::parse();

        if let Err(_) = env::var("BAD_WORDS_API_KEY") {
            panic!("BadWords API key not set");
        }

        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO key not set");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(8080))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user = env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();
        let db_host = env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        let db_name = env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            db_name,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    fn set_env() {
        env::set_var("BAD_WORDS_API_KEY", "yes");
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        env::set_var("POSTGRES_USER", "user");
        env::set_var("POSTGRES_PASSWORD", "password");
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_PORT", "5432");
        env::set_var("POSTGRES_DB", "rustwebdev");
    }

    #[test]
    fn unset_and_set_api_key() {
        let result = std::panic::catch_unwind(|| Config::new());
        assert!(result.is_err());

        set_env();

        let expected = Config {
            log_level: "warn".to_string(),
            port: 8080,
            db_user: "user".to_string(),
            db_password: "password".to_string(),
            db_host: "localhost".to_string(),
            db_port: 5432,
            db_name: "rustwebdev".to_string(),
        };

        let config = Config::new().unwrap();

        assert_eq!(config, expected);
    }
}
