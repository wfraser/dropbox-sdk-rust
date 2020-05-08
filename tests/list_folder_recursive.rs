use dropbox_sdk::files;
use dropbox_sdk::default_client::UserAuthDefaultClient;
use futures::stream::{FuturesUnordered, TryStreamExt};
use std::collections::HashSet;
use std::sync::Arc;

mod common;

#[tokio::test]
#[ignore] // very time-consuming to run; should be run separately
async fn list_folder_recursive() {
    let token = std::env::var("DBX_OAUTH_TOKEN").expect("DBX_OAUTH_TOKEN must be set");
    let client = Arc::new(UserAuthDefaultClient::new(token));

    const FOLDER: &str = "/list_folder_recursive";
    const FOLDER_INNER: &str = "/list_folder_recursive/subfolder";
    const NUM_FILES: u32 = 30;
    const FILE_SIZE: usize = 100;

    common::create_clean_folder(client.as_ref(), FOLDER).await;
    common::create_clean_folder(client.as_ref(), FOLDER_INNER).await;

    let common::CreateResult { file_tasks: file_tasks_a, path_generator: files_a, .. }
        = common::create_files(client.clone(), FOLDER, NUM_FILES, FILE_SIZE);
    let common::CreateResult { file_tasks: file_tasks_b, path_generator: files_b, .. }
        = common::create_files(client.clone(), FOLDER_INNER, NUM_FILES, FILE_SIZE);

    file_tasks_a
        .into_iter()
        .chain(file_tasks_b)
        .map(|fut| async move {
            match fut.await {
                (_, Ok(Ok(()))) => Ok(()),
                (name, Ok(Err(e))) => Err(format!("failed to create {}: {}", name, e)),
                (name, Err(e)) => Err(format!("failed to create {}: {}", name, e)),
            }
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<()>()
        .await
        .expect("failed to create all files");

    let mut files = HashSet::new();
    for i in 0 .. NUM_FILES {
        files.insert(files_a(i));
        files.insert(files_b(i));
    }

    println!("Created a bunch of files. Now listing them.");

    let mut process_entries = |entries| {
        for metadata in entries {
            match metadata {
                files::Metadata::File(files::FileMetadata { path_lower, .. }) => {
                    let path_lower = path_lower.expect("missing path_lower in response");
                    assert!(files.remove(&path_lower), "got unexpected path {}", path_lower);
                }
                files::Metadata::Folder(files::FolderMetadata { path_lower, .. }) => {
                    let path_lower = path_lower.expect("missing path_lower in response");
                    assert!(matches!(path_lower.as_str(), FOLDER | FOLDER_INNER));
                }
                other => panic!("unexpected {:?}", other),
            }
        }
    };

    println!("list_folder");
    let (mut cursor, mut has_more) = match files::list_folder(
        client.as_ref(),
        files::ListFolderArg::new(FOLDER.to_owned())
            .with_recursive(true)
            .with_limit(10),
    ).await {
        Ok(Ok(files::ListFolderResult { entries, cursor, has_more, .. })) => {
            println!("{} entries", entries.len());
            process_entries(entries);
            assert!(has_more, "expected has_more from list_folder");
            (cursor, has_more)
        }
        e => panic!("unexpected result from list_folder: {:?}", e),
    };

    while has_more {
        println!("list_folder_continue");
        let next = match files::list_folder_continue(
            client.as_ref(),
            files::ListFolderContinueArg::new(cursor.clone()),
        ).await {
            Ok(Ok(files::ListFolderResult { entries, cursor, has_more, .. })) => {
                println!("{} entries", entries.len());
                process_entries(entries);
                (cursor, has_more)
            }
            e => panic!("unexpected result from list_folder_continue: {:?}", e),
        };
        cursor = next.0;
        has_more = next.1;
    }

    assert!(files.is_empty(), "leftover unfound files: {:?}", files);
}
