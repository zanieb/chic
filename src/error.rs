use annotate_snippets::display_list::{DisplayList, FormatOptions};
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

/// An error formatter.
pub struct Error<'a> {
    snippet: Snippet<'a>,
}

impl<'a> Error<'a> {
    /// Create a new `Error` formatter.
    pub fn new(label: &'a str) -> Self {
        Self {
            snippet: Snippet {
                title: Some(Annotation {
                    label: Some(label),
                    id: None,
                    annotation_type: AnnotationType::Error,
                }),
                slices: vec![],
                footer: vec![],
                opt: FormatOptions {
                    color: true,
                    ..Default::default()
                },
            },
        }
    }

    /// Pass a new error to the formatter.
    pub fn error(
        mut self,
        line_start: usize,
        start: usize,
        end: usize,
        source: &'a str,
        label: &'a str,
    ) -> Self {
        self.snippet.slices.push(Slice {
            source,
            line_start,
            origin: None,
            fold: false,
            annotations: vec![SourceAnnotation {
                label,
                annotation_type: AnnotationType::Error,
                range: (start, end),
            }],
        });
        self
    }

    /// Create a new footer.
    pub fn help(mut self, label: &'a str) -> Self {
        self.snippet.footer.push(Annotation {
            label: Some(label),
            id: None,
            annotation_type: AnnotationType::Help,
        });
        self
    }

    pub fn into_string(self) -> String {
        DisplayList::from(self.snippet).to_string()
    }
}
