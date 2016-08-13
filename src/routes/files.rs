use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Read;
use webapp::status::StatusCode;
use modules::Modules;
use routes::{RouteHandler, UriParams, BodyParams, RouteResult};

pub fn file_handler<D: AsRef<Path>>(directory: D) -> FileHandler {
    FileHandler {
        directory: directory.as_ref().to_path_buf(),
    }
}

pub struct FileHandler {
    directory: PathBuf,
}

impl RouteHandler for FileHandler {
    fn handle(&self, _: &Modules, url: UriParams, _body: BodyParams) -> RouteResult {
        // Append the relative path to the root directory
        let path_param = url.get("").unwrap();
        let mut path = self.directory.clone();
        path.push(path_param);

        // Canonicalize it to weed out /../
        // If we couldn't canonicalize, the file doesn't exit
        let path = if let Ok(path) = path.canonicalize() {
            path
        } else {
            return RouteResult::Error(StatusCode::NotFound);
        };

        // Make sure the path is still within the base directory and is pointing to a file
        if !path.starts_with(&self.directory) && !path.is_file() {
            // TODO: Improve error handling
            return RouteResult::Error(StatusCode::NotFound);
        }

        // We've got a valid path, open up the file
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .unwrap(); // At this point we know it exists

        // Read all the file's data
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        RouteResult::Raw(data)
    }
}
