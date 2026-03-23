{% for asset in page.assets -%}
  {%- if asset is matching("[.](jpg|png)$") -%}
    {% set image = resize_image(path=asset, width=240, height=180) %}
![]({{ root_path }}{{ image.static_path }})
  {%- endif %}
{%- endfor %}