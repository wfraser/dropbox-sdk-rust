use dropbox_sdk::files;
use dropbox_sdk::client_trait::UserAuthClient;
use futures::io::Cursor;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

type PathUploadResult = (String, dropbox_sdk::Result<Result<(), files::UploadError>>);

pub struct CreateResult<F> {
    pub file_tasks: Vec<F>,
    pub path_generator: Box<dyn Fn(u32) -> String>,
    pub data_generator: Box<dyn Fn(u32) -> Vec<u8>>,
}

pub fn create_files(
    client: Arc<impl UserAuthClient + Send + Sync + 'static>,
    path: &'static str,
    num_files: u32,
    size: usize,
) -> CreateResult<impl Future<Output=PathUploadResult>> {

    let file_bytes = move |i| format!("This is file {}.\n", i)
        .into_bytes()
        .into_iter()
        .cycle()
        .take(size)
        .collect::<Vec<u8>>();
    let file_path = move |i| format!("{}/file{}.txt", path, i);

    println!("Creating {} files in {}", num_files, path);
    let mut file_tasks = vec![];
    for i in 0 .. num_files {
        let c = client.clone();
        file_tasks.push(async move {
            let path = file_path(i);
            let arg = files::CommitInfo::new(path.clone())
                .with_mode(files::WriteMode::Overwrite);
            loop {
                println!("{}: writing", path);
                let result = match files::upload(
                    c.as_ref(),
                    arg.clone(),
                    Box::pin(Cursor::new(file_bytes(i))),
                ).await {
                    Ok(Ok(_)) => {
                        println!("{}: done", path);
                        Ok(Ok(()))
                    }
                    Err(dropbox_sdk::Error::RateLimited { retry_after_seconds, .. }) => {
                        println!("{}: rate limited; sleeping {} seconds",
                            path, retry_after_seconds);
                        time::delay_for(Duration::from_secs(retry_after_seconds as u64)).await;
                        continue;
                    }
                    Ok(Err(e)) => {
                        println!("{}: upload failed: {:?}", path, e);
                        Ok(Err(e))
                    }
                    Err(e) => {
                        println!("{}: upload failed: {:?}", path, e);
                        Err(e)
                    }
                };
                break (path, result);
            }
        });
    }

    CreateResult {
        file_tasks,
        path_generator: Box::new(file_path),
        data_generator: Box::new(file_bytes),
    }
}

pub async fn create_clean_folder(client: &impl UserAuthClient, path: &str) {
    println!("Deleting any existing {} folder", path);
    match files::delete_v2(client, files::DeleteArg::new(path.to_owned())).await {
        Ok(Ok(_)) | Ok(Err(files::DeleteError::PathLookup(files::LookupError::NotFound))) => (),
        e => panic!("unexpected result when deleting {}: {:?}", path, e),
    }

    println!("Creating folder {}", path);
    match files::create_folder_v2(
        client, files::CreateFolderArg::new(path.to_owned()).with_autorename(false))
        .await
    {
        Ok(Ok(_)) => (),
        e => panic!("unexpected result when creating {}: {:?}", path, e),
    }
}
