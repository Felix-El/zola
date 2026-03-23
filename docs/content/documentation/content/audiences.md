+++
title = "Audiences"
description = "Filter pages and sections by audience"
weight = 70
+++

The audiences feature allows you to filter which pages and sections are included in your site build based on target audiences. This is useful when you want to maintain a single content repository but build different versions of your site for different audiences (e.g., a public site and an internal/beta site).

## Overview

The audiences feature works through two mechanisms:

1. **Config-level audiences**: Define which audiences are enabled globally in `config.toml`
2. **Content-level audiences**: Assign audiences to individual pages or sections in their front matter

Only content whose audiences overlap with the configured audiences will be included in the build. If the feature is disabled (no `audiences` in config), all content is included regardless.

## Configuration

### Enabling Audiences

In your `config.toml`, add an `audiences` array to specify which audiences should be included in the build:

```toml
# Build a site for "tech" and "general" audiences
audiences = ["tech", "general"]
```

If you don't specify `audiences` in the config, the feature is disabled and all content is included.

### Multiple Audience Configurations

You can create different config files for different builds. However, Zola does **not** support configuration file inheritance, so you'll need to maintain separate complete config files or use a build script to generate them.

**Option 1: Separate Complete Config Files** (Simple but repetitive)
```bash
# Build for tech and employees
zola build --config config.prod.toml

# Build for just general public
zola build --config config.public.toml
```

Where `config.public.toml` contains a complete copy of your configuration with only `audiences` changed:
```toml
base_url = "https://example.com"
title = "My Site"
# ... all other settings ...
audiences = ["general"]
```

**Option 2: Generate Config Files** (DRY approach)

Create a base config and use a build script or template engine to generate audience-specific versions:

```bash
#!/bin/bash
# generate_configs.sh

cat config.base.toml config.audiences.public.toml > config.public.toml
cat config.base.toml config.audiences.prod.toml > config.prod.toml
```

With `config.audiences.public.toml` containing just:
```toml
audiences = ["general"]
```

## Page and Section Front Matter

### Assigning Audiences to Pages

Add an `audiences` array to your page front matter:

```toml
+++
title = "Advanced Kubernetes Tips"
audiences = ["tech", "employees"]
+++

This content is for technical staff and employees.
```

### Assigning Audiences to Sections

Add an `audiences` array to section (`_index.md`) front matter:

```toml
+++
title = "Internal Docs"
audiences = ["employees"]
+++

Section content here.
```

## Filtering Logic

### Feature Disabled (No `audiences` in config)
- **Result**: All pages and sections are included
- **No warnings**: Content without audiences is treated normally

### Feature Enabled (config has `audiences`)

| Config Audiences | Page/Section Audiences | Result | Behavior |
|---|---|---|---|
| `["tech", "general"]` | not set (None) | **EXCLUDE** | Warning logged: "Skipping {path} because audiences not assigned but config.audiences is set" |
| `["tech", "general"]` | `["tech"]` | **INCLUDE** | Has overlap on "tech" |
| `["tech", "general"]` | `["marketing"]` | **EXCLUDE** | No overlap; silently filtered out |
| `["tech", "general"]` | `["tech", "general"]` | **INCLUDE** | Multiple overlaps |
| `["tech", "general"]` | `["tech", "marketing"]` | **INCLUDE** | Partial overlap on "tech" |

## Using Audiences in Templates

`config.audiences` is available in all templates. It is `null` when the feature is disabled, or an array of strings when enabled.

**Important**: Tera's `in` operator raises a render error if the right-hand side is `null`. Always guard with `config.audiences` before using `in`.

### Check if a specific audience is active

```html
{% if config.audiences and "internal" in config.audiences %}
  <a href="/internal/">Internal Docs</a>
{% endif %}
```

The `and` short-circuits: if `config.audiences` is `null` the `in` check is never evaluated.

### Check if any one of several audiences is active

```html
{% if config.audiences and ("internal" in config.audiences or "beta" in config.audiences) %}
  <div class="preview-banner">Preview content</div>
{% endif %}
```

### Show the active audiences (e.g. for debugging)

```html
{% if config.audiences %}
  Building for: {{ config.audiences | join(sep=", ") }}
{% endif %}
```

### Branch on feature enabled vs. disabled

```html
{% if config.audiences %}
  {# Audience filtering is on #}
{% else %}
  {# No audience filtering — all content is shown #}
{% endif %}
```

## Gating Content Blocks with a Shortcode

For inline audience-gating within a page's body, you can create a **bodied shortcode**.
Create `templates/shortcodes/for_audiences.html`:

```jinja
{% if not config.audiences %}
{{ body }}
{% else %}
  {% set show = false %}
  {% for a in any %}
    {% if a in config.audiences %}{% set_global show = true %}{% endif %}
  {% endfor %}
  {% if show %}{{ body }}{% endif %}
{% endif %}
```

Usage in Markdown:

```
{% for_audiences(any=["internal", "partner"]) %}
This paragraph is only rendered when

1. the build targets the `internal` audience.
2. the build targets the `partner` audience.
3. the build does not use the audience feature at all.
{% end %}
```

When `config.audiences` is not set (feature disabled), the block always renders.

## Implementation Notes

- **Any overlap** means inclusion: if config has `["tech", "general"]` and a page has `["marketing", "tech"]`, it's included
- **Overlap requires string equality**: case-sensitive, same length, same characters
- **Section filtering** applies to the entire section tree: if a section doesn't match, all its pages are also skipped
- **Root section exempt**: `content/_index.md` is never filtered regardless of audiences, as it anchors the entire site
- **Performance**: Filtering is done early (at load time), so hidden pages practically do not contribute to execution time
