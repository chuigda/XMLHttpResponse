import sys
from urllib.parse import quote, unquote


def x_printerr(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def x_export_var(var_name, value):
    print('$', var_name, '=', quote(str(value)), sep='')


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

