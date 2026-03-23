mod context;
mod markdown;
mod shortcode;

use shortcode::{Shortcode, extract_shortcodes, insert_md_shortcodes};

use errors::Result;

pub use crate::markdown::Rendered;
use crate::markdown::{markdown_to_html, render_md_with_meta};
pub use context::RenderContext;

/// Expands all shortcodes in `content`: markdown shortcodes are rendered
/// in-place; HTML shortcodes are replaced by their placeholder markers and
/// returned as a list for deferred rendering.  Returns `(processed_markdown,
/// html_shortcodes)`.
pub fn render_markdown(content: &str, context: &RenderContext) -> Result<(String, Vec<Shortcode>)> {
    if !content.contains("{{") && !content.contains("{%") {
        return Ok((content.to_string(), Vec::new()));
    }
    let definitions = context.shortcode_definitions.as_ref();
    let prefer_md = context.config.is_in_render_md_mode();
    let (content, shortcodes) = extract_shortcodes(content, definitions, prefer_md)?;
    insert_md_shortcodes(content, shortcodes, &context.tera_context, &context.tera)
}

pub fn render_content(content: &str, context: &RenderContext) -> Result<markdown::Rendered> {
    // Step 1: we render the MD shortcodes before rendering the markdown so they can get processed
    let (content, html_shortcodes) = render_markdown(content, context)?;

    // Step 2: we render the markdown and the HTML markdown at the same time
    // TODO: Here issue #1418 could be implemented
    // if do_warn_about_unprocessed_md {
    //     warn_about_unprocessed_md(unprocessed_md);
    // }
    markdown_to_html(&content, context, html_shortcodes)
}

/// Like [`render_content`] but for render-md mode: expands shortcodes and
/// extracts link/heading metadata via a lightweight pulldown-cmark pass.
/// The returned `Rendered::body` is markdown (not HTML).
pub fn render_content_md(content: &str, context: &RenderContext) -> Result<markdown::Rendered> {
    let (md_content, _html_shortcodes) = render_markdown(content, context)?;
    render_md_with_meta(&md_content, context)
}
