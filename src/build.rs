use crate::utils::{capitalise, parse_key_value_pairs, read_dir, tree, display_tree};
use grass;
use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::fs;
use std::fs::DirBuilder;
use std::path::Path;

lazy_static! {
    static ref DOCS_DIR_PATTERN: Regex = Regex::new(r"^\./docs").unwrap();
    static ref MARKDOWN_EXT_PATTERN: Regex = Regex::new(r".md$").unwrap();
    static ref MARKDOWN_METADATA_PATTERN: Regex = Regex::new(r"^---\n[\s\S]*?\n---").unwrap();
}

pub fn build() -> () {
    let template = fs::read_to_string("template.html").unwrap();
    let options = grass::Options::default();
    let paths = read_dir("./docs");
    let styles = match grass::from_path(
        "styles/style.scss",
        &options.style(grass::OutputStyle::Compressed),
    ) {
        Ok(value) => value,
        Err(err) => {
            panic!("Sass: {}", err);
        }
    };
    let nav_tree = tree("./docs");
    display_tree(nav_tree, 0);
    
    
    for path in paths {
        let text = fs::read_to_string(&path).unwrap();
        let dest = MARKDOWN_EXT_PATTERN
            .replace(&DOCS_DIR_PATTERN.replace(&path, "./dist"), ".html")
            .to_string();

        DirBuilder::new()
            .recursive(true)
            .create(Path::new(&dest).parent().unwrap())
            .unwrap();

        let metadata_string = match MARKDOWN_METADATA_PATTERN.find(&text) {
            None => "".to_string(),
            matched => matched.unwrap().as_str().replace("---", "").to_string(),
        };
        let metadata = parse_key_value_pairs(&metadata_string);

        let dest_string = String::from(&dest);

        let dest_path = Path::new(&dest_string);

        let filename = capitalise(match dest_path.file_stem().unwrap().to_str().unwrap() {
            "index" => match dest_path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
            {
                "dist" => "M Specification",
                path => path,
            },
            path => path,
        });

        let markdown = parse_md(&MARKDOWN_METADATA_PATTERN.replace(&text, ""));
        let write_result = fs::write(
            dest,
            template
                .replace(
                    "%heading%",
                    metadata.get("heading").unwrap_or(&filename.to_string()),
                )
                .replace("%content%", &markdown)
                .replace("%styles%", &styles),
        );
        match write_result {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        };
    }
}

fn parse_md(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(text, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
