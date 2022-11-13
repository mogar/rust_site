use std::fs::File;
use std::io::{prelude::*, BufReader};


/// Generate an index of blog posts
/// This will eventually iterate through markdown files in a
/// directory and populate an index at runtime
pub fn index() -> String {
    // TODO: get list blog posts in directory

    // for file in posts
    // generate summary and title
    // images?

    markdown::to_html("# Blog _Incoming_")
}

/// Small state enum for blog post parsing machine
/// Wordpress exported markdown files begin with metadata
/// Between `---` lines. This enum is used when we pull that
/// metadata out.
enum MkRead {
    Start,
    InPreamble,
    InText,
}

/// Render and display a single blog post
/// This will eventually parse and render a blog
/// post from a given markdown file.
pub fn post(slug: String) -> String {
    // TODO: convert slug to filename
    // just using the latest post for testing
    let fpath = "./markdown_posts/no-bad-kids/index.mdx";
    
    // set up file and reader
    let file = match File::open(fpath) {
        Ok(v) => v,
        Err(e) => return format!("<h1>Couldn't find post: {}</h2>", e),
    };
    let reader = BufReader::new(file);

    // fill contents and preamble from file
    let mut contents: String = "".to_string();
    let mut read_state: MkRead = MkRead::Start;
    for line in reader.lines() {
        match line {
            Err(e) => {
                contents.push_str(&format!("<h2>Error on lines {}</h2>", e));
                break;
                },
            Ok(l) => {
                if l == "---" {
                    // update state machine for post metadata
                    match read_state {
                        MkRead::Start => read_state = MkRead::InPreamble,
                        MkRead::InPreamble => read_state = MkRead::InText,
                        MkRead::InText => {
                            contents.push_str(&l);
                            contents.push('\n');
                        }
                    }
                } else {
                    // add content to correct var
                    match read_state {
                        MkRead::Start => {
                            contents.push_str(&format!("# couldn't read post {}\n", slug));
                            break;
                        },
                        MkRead::InPreamble => {
                            // TODO: add line to preamble
                        },
                        MkRead::InText => {
                            contents.push_str(&l);
                            contents.push('\n');
                        },
                    }
                }
            },
        }
    }

    // TODO: format SEO stuff separately
    
    markdown::to_html(&contents)
}