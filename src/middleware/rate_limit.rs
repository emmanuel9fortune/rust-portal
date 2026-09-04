use tower_governor::governor::GovernorConfigBuilder;

pub fn create_rate_limiter() -> impl Clone {
    GovernorConfigBuilder::default()
    .per_second(2)
    .burst_size(10)
    .finish()
    .expect("Failed to create rate limiter configuration");

}
