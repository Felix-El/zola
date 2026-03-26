{#
  crumbbar — breadcrumb navigation shortcode
  Renders:  @ [Root](/) → [Grandparent](url) → [Parent](url) → ↴

  `nav_ancestors` is populated only when `generate_navigation = true` in config.toml
  and the rendering has access to the library (i.e. from a .md template, not from
  within page content where the library is unavailable at render time).
#}
{%- set target = page | default(value=section) -%}
{%- if target.nav_ancestors %}
@ {% for a in target.nav_ancestors %}[{{ a.title | default(value="…") }}]({{ a.permalink }}){% if not loop.last %} → {% endif %}{% endfor %} ↴
{%- endif %}
