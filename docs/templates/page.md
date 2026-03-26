{%- macro nav(e) %}
{%- set parent = e.nav_ancestors | last %}
{%- set has_parent = parent and (parent.title or e.nav_ancestors | length == 1) %}
{%- if e.nav_prev or has_parent or e.nav_next %}

{% if e.nav_prev %}&#8656; [{{ e.nav_prev.title | default(value="Previous") }}]({{ e.nav_prev.permalink }}){% endif %}
{%- if e.nav_prev and has_parent %} | {% endif %}
{%- if has_parent %}&#8679; [{{ parent.title | default(value="Home") }}]({{ parent.permalink }}){% endif %}
{%- if e.nav_next and (e.nav_prev or has_parent) %} | {% endif %}
{%- if e.nav_next %}[{{ e.nav_next.title | default(value="Next") }}]({{ e.nav_next.permalink }}) &#8658;{% endif %}
{%- endif %}
{%- endmacro %}
{%- macro crumbbar(e) %}
{%- if e.nav_ancestors %}
@ {%- for a in e.nav_ancestors %}{%- if a.title or loop.first %}{% if not loop.first %} → {% endif %}[{{ a.title | default(value="Home") }}]({{ a.permalink }}){%- endif %}{%- endfor %} ↴
{%- endif %}
{%- endmacro %}
{{ self::nav(e=page) }}
{{ self::crumbbar(e=page) }}

---

{{ page.content }}

---

{{ self::nav(e=page) }}
