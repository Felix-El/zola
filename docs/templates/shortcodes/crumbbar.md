{#
  crumbbar — breadcrumb navigation shortcode
  Renders:  @ [Root](/) → [Grandparent](url) → [Parent](url) → ↴

  `nav_ancestors` is populated only when `generate_navigation = true` in config.toml
  and the rendering has access to the library (i.e. from a .md template, not from
  within page content where the library is unavailable at render time).
#}
{%- set target = page | default(value=section) -%}
{%- if target.nav_ancestors %}
@ {%- for a in target.nav_ancestors %}{%- if a.title or loop.first %}{% if not loop.first %} → {% endif %}[{{ a.title | default(value="Home") }}]({{ a.permalink }}){%- endif %}{%- endfor %} ↴
{%- endif %}
