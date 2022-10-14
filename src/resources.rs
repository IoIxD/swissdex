use std::{fs::read_to_string};
use regex::Regex;
use phf::{self,phf_map};

pub static STYLE_MAP: phf::Map<&'static str, &'static str> = phf_map!{
    ".banner" => ".a",
    "#container" => "#b",
    "#searchbar" => "#c",
    "#text" => "#d",
    ".searchpair" => ".e",
    ".searchtext" => ".f",
    "#regexp" => "#g",
    "#submit" => "#h",
    ".name" => ".i",
};

lazy_static! {
    pub static ref STYLE_FILE: String = css_sanitize(
        read_to_string("./resources/style.css")
        .unwrap()
        );

    pub static ref CSS_IDCLASS_REGEX: Regex = match Regex::new(r"(#|\.)([a-z])*") {
        Ok(a) => a,
        Err(err) => {
            panic!("{}", err);
        }
    };
    pub static ref HTML_IDCLASS_REGEX: Regex = match Regex::new(r"(id|class)='([a-z])*") {
        Ok(a) => a,
        Err(err) => {
            panic!("{}", err);
        }
    };

    static ref COMMENT_REGEX: Regex = Regex::new(r"/\*(.*?)\*/").unwrap();
}


fn css_sanitize(css: String) -> String {
    let mut new_css =   css
                        .to_owned();
    CSS_IDCLASS_REGEX.find_iter(&css)
    .for_each(|mat| {
        let st = mat.as_str();
        match STYLE_MAP.get(st) {
            Some(a) => {
                new_css = new_css.replace(st, a);
            }
            None => {
            }
        };
    });
    new_css = new_css
            .replace("\n","")
            .replace("\r","")
            .replace("  ","")
            .replace(": ",":");

    new_css = COMMENT_REGEX.replace_all(&new_css, "").to_string();

    new_css
}

/*

    static ref CSS_REGEX: Regex = match Regex::new(r"(#|\.)[a-z]*?\s") {
        Ok(a) => a,
        Err(err) => {
            panic!("{}", err);
        }
    };
    \
}


fn map_from_css_file(css: String) -> HashMap<String, String> {
    let mut id = 0;
    let mut m: HashMap<String, String> = HashMap::new();
    CSS_REGEX.find_iter(&css)
      .for_each(|mat| {
        id += 1;
        m.insert(mat.as_str().to_string(), format!("{}",id));
    });
    m
}

fn css_sanitize(css: String) -> String {
    let mut new_css = String::new();
    CSS_REGEX.find_iter(&css)
        .for_each(|mat| {
            let st = mat.as_str();
            new_css = css.replace(st, STYLE_MAP.get(st).unwrap().as_str());
        });
    new_css
}
 */