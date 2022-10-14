import sys
import collections.abc
from urllib.parse import quote, unquote


def x_printerr(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def x_export_var(var_name, value):
    if isinstance(value, list):
        x_export_array(var_name, value)
    else:
        print('$', var_name, '=', quote(str(value)), sep='')


__M_global_counter = 0


def __M_next_var_name():
    global __M_global_counter
    __M_global_counter += 1
    return '__M_vn' + str(__M_global_counter)


def x_export_array(var_name, array):
    export = ''
    for (i, item) in enumerate(array):
        item_var_name = __M_next_var_name()
        x_export_var(item_var_name, item)
        export = export + '$' + item_var_name
        if i != (len(array) - 1):
            export = export + ':'
    print('$', var_name, '=', export, sep='')


def x_fatal(reason):
    print('$FATAL=', quote(str(reason)), sep='')
    exit(0)


def x_redirect(dest):
    print('$REDIRECT=', quote(str(dest)), sep='')
    exit(0)


# Convince IDEs that we have already defined these things (though actually yes)
__M_detected_globals = globals()

if 'HttpUri' not in __M_detected_globals:
    HttpUri: str = ''

if 'HttpHeaders' not in __M_detected_globals:
    HttpHeaders: dict = {}

if 'HttpQuery' not in __M_detected_globals:
    HttpQuery: dict = {}

if 'HttpBody' not in __M_detected_globals:
    HttpBody: str | None = None

if 'ParsedHttpBody' not in __M_detected_globals:
    ParsedHttpBody: dict | None = None

# No need to import this again if it's already integrated
Xflags = 1


def x_http_header(header_name: str) -> str:
    return HttpHeaders.get(header_name)


def x_http_query(query_param: str) -> str | None:
    r_candidate = HttpQuery.get(quote(query_param))
    if r_candidate is not None:
        return unquote(r_candidate)
    else:
        return None


def x_http_body_value(key: str) -> str | None:
    if ParsedHttpBody is None:
        return None

    r_candidate = ParsedHttpBody.get(quote(key))
    if r_candidate is not None:
        return unquote(r_candidate)
    else:
        return None
