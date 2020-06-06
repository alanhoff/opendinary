use percent_encoding::percent_decode_str;

#[derive(Debug)]
pub struct Fetch {
    pub uri: String,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

pub fn create(config: &str, uri: &str) -> Fetch {
    let mut width: Option<usize> = None;
    let mut height: Option<usize> = None;

    for option in config.split(",") {
        if option.starts_with("w_") {
            width = Some(option.split("w_").nth(1).unwrap().parse().unwrap());
        }

        if option.starts_with("h_") {
            height = Some(option.split("h_").nth(1).unwrap().parse().unwrap());
        }
    }

    Fetch {
        uri: percent_decode_str(uri).decode_utf8().unwrap().to_string(),
        width,
        height,
    }
}
