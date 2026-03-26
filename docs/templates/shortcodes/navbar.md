{#
  navbar — render-md navigation shortcode
  Emits prev / up / next links for the current page or section.

  Invoke from markdown CONTENT (not from a .md template file) with:
    {{ navbar() }}

  Requires `generate_navigation = true` in config.toml.
  nav_prev / nav_parent / nav_next are only available when rendering via the
  library (i.e. after populate_sections has run), not during raw markdown parsing.
#}
{%- set target = page | default(value=section) -%}
{%- set parent = target.nav_ancestors | last -%}
{%- if target.nav_prev or parent or target.nav_next %}

---

{%- if target.nav_prev %}
&#8656; [{{ target.nav_prev.title | default(value="Previous") }}]({{ target.nav_prev.permalink }})
{%- endif %}
{%- if target.nav_prev and parent %} | {% endif %}
{%- if parent %}
&#8679; [{{ parent.title | default(value="Up") }}]({{ parent.permalink }})
{%- endif %}
{%- if target.nav_next and (target.nav_prev or parent) %} | {% endif %}
{%- if target.nav_next %}
[{{ target.nav_next.title | default(value="Next") }}]({{ target.nav_next.permalink }}) &#8658;
{%- endif %}
{%- endif %}
