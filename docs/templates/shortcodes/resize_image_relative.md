{% set mdata = get_image_metadata(path=path) %}
{% set image = resize_image(path=path, width=(mdata.width * scale)|int, op="fit_width") %}
![]({{ root_path }}{{ image.static_path }})