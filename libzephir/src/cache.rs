use redis::parse_redis_url;

pub fn create_cache() -> mouscache::Cache {
    let redis_dsn = std::env::var("REDIS_DSN");
    if redis_dsn.is_err() {
        return mouscache::memory();
    }

    let redis_dsn = redis_dsn.unwrap();
    if redis_dsn.is_empty() {
        return mouscache::memory();
    }

    let redis_url = parse_redis_url(redis_dsn.as_str()).unwrap();

    mouscache::redis(
        redis_url.host_str().unwrap(),
        redis_url.password(),
        Option::None,
    )
    .unwrap()
}
