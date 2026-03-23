use std::collections::HashMap;

use tera::{Context, Tera};

use errors::{Result, bail};

const DEFAULT_TPL_HTML: &str = include_str!("default_tpl.html");
const DEFAULT_TPL_MD: &str = include_str!("default_tpl.md");

macro_rules! render_default_tpl {
    ($filename: expr, $url: expr) => {{
        let tpl = if $filename.ends_with(".md") { DEFAULT_TPL_MD } else { DEFAULT_TPL_HTML };
        let mut context = Context::new();
        context.insert("filename", $filename);
        context.insert("url", $url);
        Tera::one_off(tpl, &context, true).map_err(std::convert::Into::into)
    }};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcodeFileType {
    Markdown,
    Html,
}

#[derive(Debug, Clone)]
pub struct ShortcodeDefinition {
    pub file_type: ShortcodeFileType,
    pub tera_name: String,
    /// If a `.md` variant of this shortcode exists alongside a `.html` variant,
    /// its template name is stored here. Used by render-md mode to pick the
    /// markdown-native shortcode instead of the HTML one.
    pub md_tera_name: Option<String>,
}
impl ShortcodeDefinition {
    pub fn new(file_type: ShortcodeFileType, tera_name: &str) -> ShortcodeDefinition {
        let tera_name = tera_name.to_string();

        ShortcodeDefinition { file_type, tera_name, md_tera_name: None }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ShortcodeInvocationCounter {
    amounts: HashMap<String, usize>,
}
impl ShortcodeInvocationCounter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get(&mut self, str: &str) -> usize {
        let nth = self.amounts.entry(str.into()).or_insert(0);
        *nth += 1;
        *nth
    }
    pub fn reset(&mut self) {
        self.amounts.clear();
    }
}

/// Fetches all the shortcodes from the Tera instances
pub fn get_shortcodes(tera: &Tera) -> HashMap<String, ShortcodeDefinition> {
    // Two-pass build: first collect everything keyed by shortcode name,
    // then merge `.md` variants into their `.html` counterpart's `md_tera_name`
    // so that both are accessible without one overwriting the other.
    let mut html_defs: HashMap<String, ShortcodeDefinition> = HashMap::new();
    let mut md_overrides: HashMap<String, String> = HashMap::new(); // name -> md tera_name

    for (identifier, template) in tera.templates.iter() {
        let is_md = template.name.ends_with(".md");
        let ext_len = if is_md { "md".len() } else { "html".len() };
        let file_type = if is_md { ShortcodeFileType::Markdown } else { ShortcodeFileType::Html };

        if template.name.starts_with("shortcodes/") {
            let head_len = "shortcodes/".len();
            let name =
                identifier[head_len..(identifier.len() - ext_len - 1)].to_string();
            if is_md {
                md_overrides.insert(name, template.name.clone());
            } else {
                html_defs.insert(name, ShortcodeDefinition::new(file_type, &template.name));
            }
            continue;
        }

        if template.name.starts_with("__zola_builtins/shortcodes/") {
            let head_len = "__zola_builtins/shortcodes/".len();
            let name = &identifier[head_len..(identifier.len() - ext_len - 1)];
            // We don't keep the built-ins one if the user provided one
            if !html_defs.contains_key(name) && !md_overrides.contains_key(name) {
                if is_md {
                    md_overrides.insert(name.to_string(), template.name.clone());
                } else {
                    html_defs.insert(
                        name.to_string(),
                        ShortcodeDefinition::new(file_type, &template.name),
                    );
                }
            }
        }
    }

    // Merge: attach md_overrides onto their html counterparts; if no html counterpart
    // exists (md-only shortcode), insert it as a primary md definition.
    for (name, md_tera_name) in md_overrides {
        if let Some(def) = html_defs.get_mut(&name) {
            def.md_tera_name = Some(md_tera_name);
        } else {
            html_defs.insert(
                name,
                ShortcodeDefinition::new(ShortcodeFileType::Markdown, &md_tera_name),
            );
        }
    }

    html_defs
}

/// Renders the given template with the given context, but also ensures that, if the default file
/// is not found, it will look up for the equivalent template for the current theme if there is one.
/// Lastly, if it's a default template (index, section or page), it will just return an empty string
/// to avoid an error if there isn't a template with that name
pub fn render_template(
    name: &str,
    tera: &Tera,
    context: Context,
    theme: &Option<String>,
) -> Result<String> {
    if let Some(template) = check_template_fallbacks(name, tera, theme) {
        return tera.render(template, &context).map_err(std::convert::Into::into);
    }

    // maybe it's a default one?
    match name {
        "index.html" | "section.html" | "index.md" | "section.md" => render_default_tpl!(
            name,
            "https://www.getzola.org/documentation/templates/pages-sections/#section-variables"
        ),
        "page.html" | "page.md" => render_default_tpl!(
            name,
            "https://www.getzola.org/documentation/templates/pages-sections/#page-variables"
        ),
        "single.html" | "list.html" | "single.md" |  "list.md" => {
            render_default_tpl!(name, "https://www.getzola.org/documentation/templates/taxonomies/")
        }
        _ => bail!("Tried to render `{}` but the template wasn't found", name),
    }
}

/// Rewrites the path of duplicate templates to include the complete theme path
/// Theme templates  will be injected into site templates, with higher priority for site
/// templates. To keep a copy of the template in case it's being extended from a site template
/// of the same name, we reinsert it with the theme path prepended
pub fn rewrite_theme_paths(tera_theme: &mut Tera, theme: &str) {
    let theme_basepath = format!("{}/templates/", theme);
    let mut new_templates = HashMap::new();
    for (key, template) in &tera_theme.templates {
        let mut tpl = template.clone();
        tpl.name = format!("{}{}", theme_basepath, key);
        new_templates.insert(tpl.name.clone(), tpl);
    }
    // Contrary to tera.extend, hashmap.extend does replace existing keys
    // We can safely extend because there's no conflicting paths anymore
    tera_theme.templates.extend(new_templates);
}

/// Checks for the presence of a given template. If none is found, also looks for a
/// fallback in theme and default templates. Returns the path of the most specific
/// template found, or none if none are present.
pub fn check_template_fallbacks<'a>(
    name: &'a str,
    tera: &'a Tera,
    theme: &Option<String>,
) -> Option<&'a str> {
    // check if it is in the templates
    if tera.templates.contains_key(name) {
        return Some(name);
    }

    // check if it is part of a theme
    if let Some(ref t) = *theme {
        let theme_template_name = format!("{}/templates/{}", t, name);
        if let Some((key, _)) = tera.templates.get_key_value(&theme_template_name) {
            return Some(key);
        }
    }

    // check if it is part of ZOLA_TERA defaults
    let default_name = format!("__zola_builtins/{}", name);
    if let Some((key, _)) = tera.templates.get_key_value(&default_name) {
        return Some(key);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::templates::{check_template_fallbacks, get_shortcodes};

    use super::rewrite_theme_paths;
    use tera::Tera;

    #[test]
    fn can_rewrite_all_paths_of_theme() {
        let mut tera = Tera::parse("test-templates/*.html").unwrap();
        rewrite_theme_paths(&mut tera, "hyde");
        // special case to make the test work: we also rename the files to
        // match the imports
        for (key, val) in &tera.templates.clone() {
            tera.templates.insert(format!("hyde/templates/{}", key), val.clone());
        }
        // Adding our fake base
        tera.add_raw_template("base.html", "Hello").unwrap();
        tera.build_inheritance_chains().unwrap();

        assert_eq!(
            tera.templates["hyde/templates/index.html"].parent,
            Some("base.html".to_string())
        );
        assert_eq!(
            tera.templates["hyde/templates/child.html"].parent,
            Some("index.html".to_string())
        );
    }

    #[test]
    fn template_fallback_is_successful() {
        let mut tera = Tera::parse("test-templates/*.html").unwrap();
        tera.add_raw_template("hyde/templates/index.html", "Hello").unwrap();
        tera.add_raw_template("hyde/templates/theme-only.html", "Hello").unwrap();

        // Check finding existing template
        assert_eq!(check_template_fallbacks("index.html", &tera, &None), Some("index.html"));

        // Check trying to find non-existent template
        assert_eq!(check_template_fallbacks("not-here.html", &tera, &None), None);

        // Check theme fallback
        assert_eq!(
            check_template_fallbacks("theme-only.html", &tera, &Some("hyde".to_string())),
            Some("hyde/templates/theme-only.html")
        );
    }

    #[test]
    fn can_overwrite_builtin_shortcodes() {
        let mut tera = Tera::parse("test-templates/*.html").unwrap();
        tera.add_raw_template("__zola_builtins/shortcodes/youtube.html", "Builtin").unwrap();
        tera.add_raw_template("shortcodes/youtube.html", "Hello").unwrap();
        let definitions = get_shortcodes(&tera);
        assert_eq!(definitions["youtube"].tera_name, "shortcodes/youtube.html");
    }
}
