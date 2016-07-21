use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Read;
use modules::Modules;
use routes::{RouteHandler, UrlParams};
use webutil::HtmlString;
use regex::Regex;

pub fn file_handler<D: AsRef<Path>>(directory: D) -> FileHandler {
    let regex = concat!(
        // Control characters (0x00–0x1f and 0x80–0x9f)
        r"(0x[0-1|8-9][0-f])|",
        // Reserved characters (/, \, ?, <, >, :, *, |, and ")
        r#"(/|\\|\?|<|>|:|\*|\||")|"#,
        // A string consisting of or ending with only periods or whitespace
        r"([.\s]+$)|",
        // Windows reserved filenames
        "(^(CON|PRN|AUX|NUL|COM1|COM2|COM3|COM4|COM5|COM6|COM7|COM8|COM9|LPT1|LPT2|LPT3|LPT4|LPT5",
        "|LPT6|LPT7|LPT8|LPT9)$)"
    );

    FileHandler {
        section_regex: Regex::new(regex).unwrap(),
        directory: directory.as_ref().to_path_buf(),
    }
}

pub struct FileHandler {
    section_regex: Regex,
    directory: PathBuf,
}

impl FileHandler {
    fn is_section_valid(&self, section: &str) -> bool {
        // Section can't be over 255 bytes
        if section.len() > 255 {
            return false;
        }

        // Section can't have a regex match
        !self.section_regex.is_match(section)
    }
}

impl RouteHandler for FileHandler {
    fn handle(&self, _: &Modules, url: UrlParams) -> HtmlString {
        // We manually handle the path processing just to be sure, first start by splitting it
        let path_param = url.get("").unwrap();
        let sections: Vec<_> = path_param.split(|c| c == '\\' || c == '/').collect();

        // Build up the path of the actual file we are looking for
        let mut path = self.directory.clone();
        for section in sections.iter() {
            // Make sure this specific section of the path is valid
            if !self.is_section_valid(section) {
                // TODO: Improve error handling
                return HtmlString::bless("<h1>Invalid URL</h1>");
            }

            // Add it to the path
            path.push(section);
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

            // Now send it over
            // TODO: Allow sending over data not as a HtmlString, or any string at all
            let data = String::from_utf8(data).unwrap();
            HtmlString::bless(data)
        } else {
            // We couldn't open the file, assume we didn't find it and return 404
            // TODO: Improve error handling
            HtmlString::bless("<h1>404</h1>")
        }
    }
}
