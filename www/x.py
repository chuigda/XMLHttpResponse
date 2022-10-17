import sys
from urllib.parse import quote, unquote


HttpUri: str = ''

HttpHeaders: dict = {}

HttpQuery: dict = {}

HttpBody: str | None = None

ParsedHttpBody: dict | None = None

Xflags = 1


# for importing items


def xm_set_http_uri(uri: str):
    HttpUri = uri


def xm_add_http_header(header_name: str, encoded_str: str):
    HttpHeaders[header_name] = unquote(encoded_str)


def xm_add_http_query(query_param: str, value: str):
    HttpQuery[query_param] = unquote(value)


def xm_set_http_body(body: str):
    HttpBody = body


def xm_add_parsed_http_body():
    global ParsedHttpBody
    ParsedHttpBody = {}


def xm_add_parsed_http_body_item(key: str, value: str):
    ParsedHttpBody[key] = unquote(value)


# reading variables


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


# exporting items


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


def x_export_var(var_name, value):
    if isinstance(value, list):
        x_export_array(var_name, value)
    else:
        print('$', var_name, '=', quote(str(value)), sep='')


def x_fatal(reason):
    print('$FATAL=', quote(str(reason)), sep='')
    exit(0)


def x_redirect(dest):
    print('$REDIRECT=', quote(str(dest)), sep='')
    exit(0)


def x_printerr(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)
