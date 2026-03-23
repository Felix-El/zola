# Tags

{% for t in terms %}
- [{{ t.name }}]({{ t.permalink }}) ({{ t.page_count }})
{% endfor %}
