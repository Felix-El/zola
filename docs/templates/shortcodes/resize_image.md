{% set image = resize_image(path=path, width=width, height=height, op=op) %}
![]({{ root_path }}{{ image.static_path }})