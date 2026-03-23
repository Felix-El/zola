+++
title = "Public Page"
audiences = ["public"]
+++

This page is for the public audience.

Prove that shortcodes can access `config.audiences`:
{{ audiences_info() }}

Prove that bodied shortcodes gate content correctly:
{% for_audiences(any=["public"]) %}sc-body-public-visible{% end %}
{% for_audiences(any=["internal"]) %}sc-body-internal-visible{% end %}
