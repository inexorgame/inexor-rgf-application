use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crate::runtime::Runtime;
use crate::runtime::RuntimeBuilder;

#[tokio::test(flavor = "multi_thread")]
async fn test_shutdown() {
    let start = Instant::now();
    RuntimeBuilder::new()
        .ignore_config_files()
        .disable_all_plugins(true)
        .pick_free_port()
        .init()
        .await
        .post_init()
        .await
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            let inner_runtime = runtime.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(500)).await;
                inner_runtime.get_shutdown_manager().do_shutdown();
            });
        })
        .await
        .run()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
    let elapsed = start.elapsed();
    // Shutdown
    assert!(elapsed > Duration::from_millis(500), "Shutdown at earliest after 2 seconds");
    assert!(elapsed < Duration::from_millis(600), "Shutdown at latest after 3 seconds");
}