use std::collections::HashMap;

pub mod bindings;

#[derive(Debug)]
pub enum HighlightError {
    UnknownLanguage,
    TreeSitterError,
}

pub struct Highlighter {
    language_configs: HashMap<String, tree_sitter_highlight::HighlightConfiguration>,
    highlight_styles: Vec<String>,
}

impl Highlighter {
    pub fn new() -> Self {
        // C++ language config also need C language queries
        // https://github.com/tree-sitter/tree-sitter/issues/1050
        let mut cpp_config = tree_sitter_highlight::HighlightConfiguration::new(
            tree_sitter_cpp::language(),
            &format!(
                "{}\n{}",
                tree_sitter_cpp::HIGHLIGHT_QUERY,
                tree_sitter_c::HIGHLIGHT_QUERY
            ),
            "",
            "",
        )
        .unwrap();

        let mut rust_config = tree_sitter_highlight::HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = vec![];
        highlight_names.extend(cpp_config.query.capture_names().iter().cloned());
        highlight_names.extend(rust_config.query.capture_names().iter().cloned());

        highlight_names.sort();
        highlight_names.dedup();
        cpp_config.configure(&highlight_names);
        rust_config.configure(&highlight_names);

        let mut language_configs = HashMap::new();
        language_configs.insert("cpp".to_string(), cpp_config);
        language_configs.insert("rust".to_string(), rust_config);

        let highlight_styles = highlight_names
            .iter()
            .map(|name| format!("class=\"tshl-{}\"", name.replace(".", "_")))
            .collect();

        Self {
            language_configs,
            highlight_styles,
        }
    }

    pub fn highlight(&self, lang: &str, code: &str) -> Result<String, HighlightError> {
        use tree_sitter_highlight::{Highlight, Highlighter, HtmlRenderer};

        let config = self.language_configs.get(lang).ok_or(HighlightError::UnknownLanguage)?;

        let mut highlighter = Highlighter::new();
        let highlights = highlighter
            .highlight(config, code.as_bytes(), None, |lang| self.language_configs.get(lang))
            .map_err(|_| HighlightError::TreeSitterError)?;

        let mut html_renderer = HtmlRenderer::new();
        html_renderer
            .render(highlights, code.as_bytes(), &|Highlight(index)| {
                self.highlight_styles[index].as_bytes()
            })
            .map_err(|_| HighlightError::TreeSitterError)?;

        Ok(html_renderer.lines().collect())
    }

    pub fn supported_languages(&self) -> impl Iterator<Item = &str> {
        self.language_configs.keys().map(|s| &**s)
    }
}
