use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Read;
use modules::Modules;
use routes::{RouteHandler, UrlParams};
use webutil::HtmlString;

pub fn file_handler<D: AsRef<Path>>(directory: D) -> FileHandler {
    FileHandler {
        directory: directory.as_ref().to_path_buf(),
    }
}

pub struct FileHandler {
    directory: PathBuf,
}

impl RouteHandler for FileHandler {
    fn handle(&self, _: &Modules, url: UrlParams) -> Vec<u8> {
        // Append the relative path to the root directory, then canonicalize it to weed out /../
        let path_param = url.get("").unwrap();
        let mut path = self.directory.clone();
        path.push(path_param);
        let path = path.canonicalize().unwrap();

        // Make sure the path is still within the base directory and is pointing to a file
        if !path.starts_with(&self.directory) && !path.is_file() {
            // TODO: Improve error handling
            return HtmlString::bless("<h1>Invalid URL</h1>").into()
        }

        // We've got a valid path, open up the file
        let file = OpenOptions::new()
            .read(true)
            .open(path);

        // Check if we could actually open it
        if let Ok(mut file) = file {
            // Read all the file's data
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            data
        } else {
            // We couldn't open the file, assume we didn't find it and return 404
            // TODO: Improve error handling
            HtmlString::bless("<h1>404</h1>").into()
        }
    }
}
