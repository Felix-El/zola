{%- macro nav(e) %}
{%- set parent = e.nav_ancestors | last %}
{% if e.nav_prev or parent or e.nav_next %}

{%- if e.nav_prev %}&#8656; [{{ e.nav_prev.title | default(value="Previous") }}]({{ e.nav_prev.permalink }}){% endif %}
{%- if e.nav_prev and parent %} | {% endif %}
{%- if parent %}&#8679; [{{ parent.title | default(value="Up") }}]({{ parent.permalink }}){% endif %}
{%- if e.nav_next and (e.nav_prev or parent) %} | {% endif %}
{%- if e.nav_next %}[{{ e.nav_next.title | default(value="Next") }}]({{ e.nav_next.permalink }}) &#8658;{% endif %}
{%- endif %}
{%- endmacro %}
{%- macro crumbbar(e) %}
{%- if e.nav_ancestors %}
@ {% for a in e.nav_ancestors %}[{{ a.title | default(value="…") }}]({{ a.permalink }}){% if not loop.last %} → {% endif %}{% endfor %} ↴
{%- endif %}
{%- endmacro %}
{{ self::nav(e=section) }}
{{ self::crumbbar(e=section) }}

---

{{ section.content }}
{%- if section.nav_children %}

## Contents

{%- for child in section.nav_children %}
- [{{ child.title | default(value=child.permalink) }}]({{ child.permalink }})
{%- endfor %}
{%- endif %}

---

{{ self::nav(e=section) }}
