use std::error::Error;
use dropbox_sdk::DropboxError;
use dropbox_sdk::files;

#[test]
fn error_downcast_test() {
    let lookup_err = files::LookupError::MalformedPath(Some("test".into()));

    // All these errors have a LookupError inside them.
    let errors: Vec<Box<dyn DropboxError>> = vec![
        Box::new(files::AddTagError::Path(lookup_err.clone())),
        Box::new(files::BaseTagError::Path(lookup_err.clone())),
        Box::new(files::DeleteError::PathLookup(lookup_err.clone())),
        Box::new(files::DownloadError::Path(lookup_err.clone())),
        Box::new(files::DownloadZipError::Path(lookup_err.clone())),
        Box::new(files::ExportError::Path(lookup_err.clone())),
        Box::new(files::GetCopyReferenceError::Path(lookup_err.clone())),
        Box::new(files::GetMetadataError::Path(lookup_err.clone())),
        Box::new(files::GetTemporaryLinkError::Path(lookup_err.clone())),
        Box::new(files::ListFolderContinueError::Path(lookup_err.clone())),
        Box::new(files::ListFolderError::Path(lookup_err.clone())),
        Box::new(files::ListRevisionsError::Path(lookup_err.clone())),
        Box::new(files::LockFileError::PathLookup(lookup_err.clone())),
        Box::new(files::PaperUpdateError::Path(lookup_err.clone())),
        Box::new(files::PreviewError::Path(lookup_err.clone())),
        Box::new(files::RelocationBatchError::FromLookup(lookup_err.clone())),
        Box::new(files::RelocationError::FromLookup(lookup_err.clone())),
        Box::new(files::RemoveTagError::Path(lookup_err.clone())),
        Box::new(files::RestoreError::PathLookup(lookup_err.clone())),
        Box::new(files::SearchError::Path(lookup_err.clone())),
        Box::new(files::SyncSettingsError::Path(lookup_err.clone())),
        Box::new(files::ThumbnailError::Path(lookup_err.clone())),
        Box::new(files::ThumbnailV2Error::Path(lookup_err.clone())),
    ];

    for e in errors {
        assert_eq!(Some(&lookup_err), e.downcast());
    }

    // Make sure we can get it from itself.
    assert_eq!(Some(&lookup_err), (&lookup_err as &dyn DropboxError).downcast::<files::LookupError>());
}

fn stuff(i: i32) -> Result<(), Box<dyn DropboxError>> {
    let e = files::LookupError::NotFound;
    if i % 2 == 0 {
        Err(Box::new(files::ListFolderError::Path(e)))
    } else {
        Err(Box::new(files::ListFolderContinueError::Path(e)))
    }
}

fn err_inner<'a, T: Error + 'static>(mut e: &'a (dyn Error + 'static)) -> Option<&'a T> {
    loop {
        if let Some(t) = e.downcast_ref() {
            return Some(t);
        }
        if let Some(next) = e.source() {
            if std::ptr::eq(e, next) {
                // Cycle detected.
                return None;
            }
            e = next;
        } else {
            return None;
        }
    }
}

#[test]
fn test_err() {
    let mut e = stuff(0).unwrap_err();
    if let Some(e) = e.downcast::<files::LookupError>() {
        println!("{e}");
    } else {
        panic!("ope");
    }

    if let Some(e) = err_inner::<files::LookupError>(e.as_ref()) {
        println!("{e}");
    } else {
        panic!("ope");
    }
}
