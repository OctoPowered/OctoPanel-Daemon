use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set global default subscriber");
}
