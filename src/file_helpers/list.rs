//! Helpers for listing directories.

use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

use crate::file_helpers::{Error, RRExt};
use crate::{files, UserAuthClient};

/// Make an iterator that yields directory entries under a given path, optionally recursively.
pub fn list_directory<'a, T: UserAuthClient>(
    client: &'a T,
    path: &str,
    recursive: bool,
) -> Result<DirectoryIterator<'a, T>, Error> {
    assert!(
        path.starts_with('/'),
        "path needs to be absolute (start with a '/')"
    );
    let requested_path = if path == "/" {
        // Root folder should be requested as empty string.
        String::new()
    } else {
        path.to_owned()
    };
    let result = list_folder_internal(
        client,
        files::list_folder,
        &files::ListFolderArg::new(requested_path).with_recursive(recursive),
    )?;
    let cursor = if result.has_more {
        Some(result.cursor)
    } else {
        None
    };
    Ok(DirectoryIterator {
        client,
        cursor,
        buffer: result.entries.into(),
    })
}

/// An iterator over directory entries which pages though the Dropbox API as necessary.
pub struct DirectoryIterator<'a, T: UserAuthClient> {
    client: &'a T,
    buffer: VecDeque<files::Metadata>,
    cursor: Option<String>,
}

impl<'a, T: UserAuthClient> Iterator for DirectoryIterator<'a, T> {
    type Item = Result<files::Metadata, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.buffer.pop_front() {
            Some(Ok(entry))
        } else if let Some(cursor) = self.cursor.take() {
            let result = match list_folder_internal(
                self.client,
                files::list_folder_continue,
                &files::ListFolderContinueArg::new(cursor),
            ) {
                Ok(r) => r,
                Err(e) => return Some(Err(e)),
            };
            self.buffer.extend(result.entries);
            if result.has_more {
                self.cursor = Some(result.cursor);
            }
            self.buffer.pop_front().map(Ok)
        } else {
            None
        }
    }
}

fn list_folder_internal<T, A, E>(
    client: &T,
    f: impl Fn(&T, &A) -> crate::Result<Result<files::ListFolderResult, E>>,
    arg: &A,
) -> Result<files::ListFolderResult, Error>
where
    T: UserAuthClient,
    A: Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut errors = 0;
    loop {
        match f(client, arg) {
            Ok(Ok(r)) => break Ok(r),
            Err(crate::Error::RateLimited {
                reason,
                retry_after_seconds,
            }) => {
                warn!(
                    "rate-limited ({}), waiting {} seconds",
                    reason, retry_after_seconds
                );
                if retry_after_seconds > 0 {
                    sleep(Duration::from_secs(u64::from(retry_after_seconds)));
                }
            }
            error => {
                errors += 1;
                if errors == 3 {
                    warn!("Error calling list_folder_continue: {:?}, failing", error);
                    return error.combine();
                } else {
                    warn!("Error calling list_folder_continue: {:?}, retrying.", error);
                }
            }
        }
    }
}
