use exor::run;
use tracing::error;

#[tokio::main]
pub async fn main() -> () {
    if let Err(e) = run().await {
        error!("Error running system: {e:?}");
    }
}
