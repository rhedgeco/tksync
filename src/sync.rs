use derive_more::Display;
use regex::Regex;

const FONT_REGEX: &str = concat!(
    r#"@font-face \{\nfont-family:"(.*)";\n"#,
    r#"src:url\("(.*)"\) format\("woff2"\),"#,
    r#"url\("(.*)"\) format\("woff"\),"#,
    r#"url\("(.*)"\) format\("opentype"\);\n"#,
    r#"font-display:auto;font-style:(.*);"#,
    r#"font-weight:(.*);font-stretch:normal;\n\}"#,
);

#[derive(Display)]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Display)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

pub struct TkFont {
    pub name: String,
    pub woff2: String,
    pub woff: String,
    pub opentype: String,
    pub style: FontStyle,
    pub weight: FontWeight,
}

impl TkFont {
    pub fn parse_css(css: &str) -> Vec<TkFont> {
        let mut vec = Vec::new();

        let re = Regex::new(FONT_REGEX).expect("Invalid REGEX");
        for capture in re.captures_iter(css) {
            let font = TkFont {
                name: capture[1].into(),
                woff2: capture[2].into(),
                woff: capture[3].into(),
                opentype: capture[4].into(),
                style: match &capture[5] {
                    "normal" => FontStyle::Normal,
                    "italic" => FontStyle::Italic,
                    _ => unreachable!(),
                },
                weight: match &capture[6] {
                    "100" => FontWeight::Thin,
                    "200" => FontWeight::ExtraLight,
                    "300" => FontWeight::Light,
                    "400" => FontWeight::Normal,
                    "500" => FontWeight::Medium,
                    "600" => FontWeight::SemiBold,
                    "700" => FontWeight::Bold,
                    "800" => FontWeight::ExtraBold,
                    "900" => FontWeight::Black,
                    _ => unreachable!(),
                },
            };

            vec.push(font);
        }

        vec
    }
}
