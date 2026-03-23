use std::path::Path;

use errors::Result;
use site::Site;

use crate::messages;

pub fn render_md(
    root_dir: &Path,
    config_file: &Path,
    output_dir: Option<&Path>,
    include_drafts: bool,
) -> Result<()> {
    let mut site = Site::new(root_dir, config_file)?;
    let output_path = output_dir.map(|p| p.to_path_buf()).unwrap_or_else(|| root_dir.join("md"));

    if include_drafts {
        site.include_drafts();
    }
    site.set_output_path(&output_path);
    site.enable_render_md_mode();
    site.load()?;
    messages::notify_site_size(&site);
    messages::warn_about_ignored_pages(&site);
    site.build()
}
