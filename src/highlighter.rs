use std::str::FromStr;
use syntect::easy::HighlightLines;
use syntect::highlighting::{
    Color, ScopeSelectors, Style, StyleModifier, Theme, ThemeItem, ThemeSettings,
};
use syntect::parsing::SyntaxSet;

pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
    file_extension: String,
}

impl Highlighter {
    pub fn new(file_extension: &str) -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme: Self::create_tokyonight_theme(),
            file_extension: file_extension.to_string(),
        }
    }

    fn create_tokyonight_theme() -> Theme {
        // TokyoNight Night color palette
        let bg = Color {
            r: 0x1a,
            g: 0x1b,
            b: 0x26,
            a: 0xff,
        };
        let fg = Color {
            r: 0xc0,
            g: 0xca,
            b: 0xf5,
            a: 0xff,
        };
        let comment = Color {
            r: 0x56,
            g: 0x5f,
            b: 0x89,
            a: 0xff,
        };
        let cyan = Color {
            r: 0x7d,
            g: 0xcf,
            b: 0xff,
            a: 0xff,
        };
        let green = Color {
            r: 0x9e,
            g: 0xce,
            b: 0x6a,
            a: 0xff,
        };
        let orange = Color {
            r: 0xff,
            g: 0x9e,
            b: 0x64,
            a: 0xff,
        };
        let magenta = Color {
            r: 0xbb,
            g: 0x9a,
            b: 0xf7,
            a: 0xff,
        };
        let blue = Color {
            r: 0x7a,
            g: 0xa2,
            b: 0xf7,
            a: 0xff,
        };
        let blue1 = Color {
            r: 0x2a,
            g: 0xc3,
            b: 0xde,
            a: 0xff,
        };
        let blue5 = Color {
            r: 0x89,
            g: 0xdd,
            b: 0xff,
            a: 0xff,
        };
        let red = Color {
            r: 0xf7,
            g: 0x76,
            b: 0x8e,
            a: 0xff,
        };
        let _yellow = Color {
            r: 0xe0,
            g: 0xaf,
            b: 0x68,
            a: 0xff,
        };
        let purple = Color {
            r: 0x9d,
            g: 0x7c,
            b: 0xd8,
            a: 0xff,
        };

        Theme {
            name: Some("TokyoNight Night".to_string()),
            author: Some("folke".to_string()),
            settings: ThemeSettings {
                foreground: Some(fg),
                background: Some(bg),
                caret: Some(fg),
                line_highlight: Some(Color {
                    r: 0x29,
                    g: 0x2e,
                    b: 0x42,
                    a: 0xff,
                }),
                selection: Some(Color {
                    r: 0x28,
                    g: 0x34,
                    b: 0x57,
                    a: 0xff,
                }),
                ..Default::default()
            },
            scopes: vec![
                // Comments
                ThemeItem {
                    scope: ScopeSelectors::from_str("comment").unwrap(),
                    style: StyleModifier {
                        foreground: Some(comment),
                        background: None,
                        font_style: None,
                    },
                },
                // Strings
                ThemeItem {
                    scope: ScopeSelectors::from_str("string").unwrap(),
                    style: StyleModifier {
                        foreground: Some(green),
                        background: None,
                        font_style: None,
                    },
                },
                // Numbers
                ThemeItem {
                    scope: ScopeSelectors::from_str("constant.numeric").unwrap(),
                    style: StyleModifier {
                        foreground: Some(orange),
                        background: None,
                        font_style: None,
                    },
                },
                // Constants
                ThemeItem {
                    scope: ScopeSelectors::from_str("constant").unwrap(),
                    style: StyleModifier {
                        foreground: Some(orange),
                        background: None,
                        font_style: None,
                    },
                },
                // Keywords
                ThemeItem {
                    scope: ScopeSelectors::from_str("keyword").unwrap(),
                    style: StyleModifier {
                        foreground: Some(cyan),
                        background: None,
                        font_style: None,
                    },
                },
                // Storage (fn, let, const, etc.)
                ThemeItem {
                    scope: ScopeSelectors::from_str("storage").unwrap(),
                    style: StyleModifier {
                        foreground: Some(magenta),
                        background: None,
                        font_style: None,
                    },
                },
                // Functions
                ThemeItem {
                    scope: ScopeSelectors::from_str("entity.name.function").unwrap(),
                    style: StyleModifier {
                        foreground: Some(blue),
                        background: None,
                        font_style: None,
                    },
                },
                // Types
                ThemeItem {
                    scope: ScopeSelectors::from_str("entity.name.type, storage.type").unwrap(),
                    style: StyleModifier {
                        foreground: Some(blue1),
                        background: None,
                        font_style: None,
                    },
                },
                // Variables
                ThemeItem {
                    scope: ScopeSelectors::from_str("variable").unwrap(),
                    style: StyleModifier {
                        foreground: Some(fg),
                        background: None,
                        font_style: None,
                    },
                },
                // Operators
                ThemeItem {
                    scope: ScopeSelectors::from_str("keyword.operator").unwrap(),
                    style: StyleModifier {
                        foreground: Some(blue5),
                        background: None,
                        font_style: None,
                    },
                },
                // Punctuation
                ThemeItem {
                    scope: ScopeSelectors::from_str("punctuation").unwrap(),
                    style: StyleModifier {
                        foreground: Some(blue5),
                        background: None,
                        font_style: None,
                    },
                },
                // Meta/Macros
                ThemeItem {
                    scope: ScopeSelectors::from_str("meta.macro, entity.name.function.macro")
                        .unwrap(),
                    style: StyleModifier {
                        foreground: Some(blue),
                        background: None,
                        font_style: None,
                    },
                },
                // Support (built-in functions)
                ThemeItem {
                    scope: ScopeSelectors::from_str("support.function").unwrap(),
                    style: StyleModifier {
                        foreground: Some(cyan),
                        background: None,
                        font_style: None,
                    },
                },
                // Namespaces/Modules
                ThemeItem {
                    scope: ScopeSelectors::from_str("entity.name.namespace, entity.name.module")
                        .unwrap(),
                    style: StyleModifier {
                        foreground: Some(purple),
                        background: None,
                        font_style: None,
                    },
                },
                // Attributes/Annotations
                ThemeItem {
                    scope: ScopeSelectors::from_str("meta.attribute, storage.modifier").unwrap(),
                    style: StyleModifier {
                        foreground: Some(blue),
                        background: None,
                        font_style: None,
                    },
                },
                // Invalid
                ThemeItem {
                    scope: ScopeSelectors::from_str("invalid").unwrap(),
                    style: StyleModifier {
                        foreground: Some(red),
                        background: None,
                        font_style: None,
                    },
                },
            ],
        }
    }

    pub fn highlight_line(&self, line: &str) -> Vec<(Style, String)> {
        let syntax = self
            .syntax_set
            .find_syntax_by_extension(&self.file_extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut h = HighlightLines::new(syntax, &self.theme);

        let ranges = h.highlight_line(line, &self.syntax_set).unwrap_or_default();

        ranges
            .into_iter()
            .map(|(style, text)| (style, text.to_string()))
            .collect()
    }
}

pub fn style_to_crossterm_color(
    style: Style,
) -> (crossterm::style::Color, crossterm::style::Color) {
    let fg = crossterm::style::Color::Rgb {
        r: style.foreground.r,
        g: style.foreground.g,
        b: style.foreground.b,
    };
    let bg = crossterm::style::Color::Rgb {
        r: style.background.r,
        g: style.background.g,
        b: style.background.b,
    };
    (fg, bg)
}
