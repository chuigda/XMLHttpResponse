if 'Xflags' not in globals():
    from x import *

counter = x_http_body_value('counter')
x_printerr(ParsedHttpBody)
if counter is None:
    x_export_var('counter', 0)
else:
    counter = int(counter)
    x_export_var('counter', counter + 1)
