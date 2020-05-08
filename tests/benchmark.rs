use dropbox_sdk::files;
use dropbox_sdk::default_client::UserAuthDefaultClient;
use futures::io::AsyncReadExt;
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time;

mod common;

/// This test should be run with --nocapture to see the timing info output.
#[tokio::test]
#[ignore] // very time-consuming to run; should be run separately
async fn fetch_files() {
    let token = std::env::var("DBX_OAUTH_TOKEN").expect("DBX_OAUTH_TOKEN must be set");
    let client = Arc::new(UserAuthDefaultClient::new(token));

    const FOLDER: &str = "/fetch_small_files";
    const NUM_FILES: u32 = 100;
    const NUM_TEST_RUNS: u32 = 4;
    const FILE_SIZE: usize = 1024 * 1024; // 1 MiB
    const PARALLEL_OPS: usize = 20;

    println!("Setting up test environment");
    let setup_start = Instant::now();

    common::create_clean_folder(client.as_ref(), FOLDER).await;

    let common::CreateResult { file_tasks, path_generator, data_generator } = common::create_files(
        client.clone(), FOLDER, NUM_FILES, FILE_SIZE);

    let create_results = stream::iter(file_tasks)
        .buffer_unordered(PARALLEL_OPS)
        .collect::<Vec<_>>()
        .await;
    let mut all_succeeded = true;
    for (path, result) in create_results.into_iter() {
        if !matches!(result, Ok(Ok(_))) {
            println!("{} failed", path);
            all_succeeded = false;
        }
    }
    assert!(all_succeeded);

    println!("Test setup complete ({} secs). Starting benchmark.",
        setup_start.elapsed().as_secs_f64());

    let mut times = vec![];
    for _ in 0 .. NUM_TEST_RUNS {
        println!("sleeping 10 seconds before run");
        time::delay_for(Duration::from_secs(10)).await;
        let start = Instant::now();
        let mut tasks = vec![];
        for i in 0 .. NUM_FILES {
            let path = path_generator(i);
            let expected_bytes = data_generator(i);
            let c = client.clone();
            tasks.push(async move {
                loop {
                    let arg = files::DownloadArg::new(path.clone());
                    let result = match files::download(c.as_ref(), arg, None, None).await {
                        Ok(Ok(result)) => {
                            let mut read_bytes = Vec::new();
                            result.body.expect("result should have a body")
                                .read_to_end(&mut read_bytes).await.expect("read_to_end");
                            assert_eq!(&read_bytes, &expected_bytes);
                            Ok(())
                        }
                        Err(dropbox_sdk::Error::RateLimited { retry_after_seconds, .. }) => {
                            println!("{}: WARNING: rate-limited {} seconds", path, retry_after_seconds);
                            time::delay_for(Duration::from_secs(retry_after_seconds as u64)).await;
                            continue;
                        }
                        Ok(Err(e)) => Err(format!("{}: download failed: {:?}", path, e)),
                        Err(e) => Err(format!("{}: download failed: {:?}", path, e)),
                    };
                    break (path, result);
                }
            });
        }

        let results = stream::iter(tasks)
            .buffer_unordered(PARALLEL_OPS)
            .collect::<Vec<_>>()
            .await;
        let dur = start.elapsed();
        println!("test finished in {} seconds", dur.as_secs_f64());
        times.push(dur);

        let mut all_succeeded = true;
        for (path, result) in results.into_iter() {
            if let Err(msg) = result {
                println!("{}: {}", path, msg);
                all_succeeded = false;
            }
        }
        assert!(all_succeeded);
    }

    println!("{:?}", times);
    println!("average: {} seconds",
        times.iter().map(Duration::as_secs_f64).sum::<f64>() / times.len() as f64)
}
