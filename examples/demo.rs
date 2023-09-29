#![deny(rust_2018_idioms)]

//! This example illustrates a few basic Dropbox API operations: getting an OAuth2 token, listing
//! the contents of a folder recursively, and fetching a file given its path.

use dropbox_sdk::files;
use dropbox_sdk::file_helpers::list::list_directory;
use dropbox_sdk::default_client::UserAuthDefaultClient;

use std::io::{self, Read};

enum Operation {
    Usage,
    List { path: String },
    Download { path: String },
}

fn parse_args() -> Operation {
    let mut list = false;
    let mut path: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => return Operation::Usage,
            "--list" if !list => {
                list = true;
            }
            s if s.starts_with('/') && path.is_none() => {
                path = Some(arg);
            }
            _ => {
                eprintln!("Unrecognized option {arg:?}");
                eprintln!();
                return Operation::Usage;
            }
        }
    }
    match (list, path) {
        (true, Some(path)) => Operation::List { path },
        (true, None) => Operation::List { path: "/".to_owned() },
        (false, Some(path)) => Operation::Download { path },
        (false, None) => Operation::Usage,
    }
}

fn main() {
    env_logger::init();

    let operation = parse_args();
    if let Operation::Usage = operation {
        eprintln!("usage: {} [option]", std::env::args().next().unwrap());
        eprintln!("    options:");
        eprintln!("        --help | -h      view this text");
        eprintln!("        --list [<path>]  list all files in your Dropbox under <path>");
        eprintln!("        <path>           print the file at the given path to stdout");
        eprintln!();
        eprintln!("    If a Dropbox OAuth token is given in the environment variable");
        eprintln!("    DBX_OAUTH_TOKEN, it will be used, otherwise you will be prompted for");
        eprintln!("    authentication interactively.");
        std::process::exit(1);
    }

    let auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    let client = UserAuthDefaultClient::new(auth);

    if let Operation::Download { path } = operation {
        eprintln!("downloading file {}", path);
        eprintln!();
        let mut bytes_out = 0u64;
        let download_arg = files::DownloadArg::new(path.clone());
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();
        'download: loop {
            let result = files::download(&client, &download_arg, Some(bytes_out), None);
            match result {
                Ok(Ok(download_result)) => {
                    let mut body = download_result.body.expect("no body received!");
                    loop {
                        // limit read to 1 MiB per loop iteration so we can output progress
                        #[allow(clippy::needless_borrow)] // 2022-02-09: this lint is wrong
                        let mut input_chunk = (&mut body).take(1024 * 1024);
                        match io::copy(&mut input_chunk, &mut stdout_lock) {
                            Ok(0) => {
                                eprint!("\r");
                                break 'download;
                            }
                            Ok(len) => {
                                bytes_out += len;
                                if let Some(total) = download_result.content_length {
                                    eprint!("\r{:.01}%",
                                        bytes_out as f64 / total as f64 * 100.);
                                } else {
                                    eprint!("\r{} bytes", bytes_out);
                                }
                            }
                            Err(e) => {
                                eprintln!("Read error: {}", e);
                                continue 'download; // do another request and resume
                            }
                        }
                    }
                },
                Ok(Err(files::DownloadError::Path(files::LookupError::NotFile))) => {
                    eprintln!("metadata for {path}: {:#?}",
                        files::get_metadata(&client, &files::GetMetadataArg::new(path.clone())));
                },
                Ok(Err(download_error)) => {
                    eprintln!("Download error: {}", download_error);
                },
                Err(request_error) => {
                    eprintln!("Error: {}", request_error);
                }
            }
            break 'download;
        }
    } else if let Operation::List { path } = operation {
        eprintln!("listing all files under {path}");
        match list_directory(&client, &path, true) {
            Ok(iterator) => {
                for entry_result in iterator {
                    match entry_result {
                        Ok(files::Metadata::Folder(entry)) => {
                            println!("Folder: {}", entry.path_display.unwrap_or(entry.name));
                        },
                        Ok(files::Metadata::File(entry)) => {
                            println!("File: {}", entry.path_display.unwrap_or(entry.name));
                        },
                        Ok(files::Metadata::Deleted(entry)) => {
                            panic!("unexpected deleted entry: {:?}", entry);
                        },
                        Err(e) => {
                            eprintln!("Error listing files: {}", e);
                            break;
                        },
                    }
                }
            },
            Err(e) => {
                eprintln!("Error listing files: {}", e);
            }
        }
    }
}
