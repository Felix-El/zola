# Zola themes in {{ term.name }}

{% for page in term.pages %}
- [{{ page.title }}]({{ page.permalink }})
{% endfor %}
