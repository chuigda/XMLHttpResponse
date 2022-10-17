import sys
from urllib.request import Request, urlopen
from urllib.parse import quote, unquote


XmSocketAddr: str = ''

XmApiAuthToken: str = ''

XmSessionData: str = ''

HttpUri: str = ''

HttpHeaders: dict = {}

HttpQuery: dict = {}

HttpBody: str | None = None

ParsedHttpBody: dict | None = None

Xflags = 1


# for importing items


def xm_set_socket_addr(socket_addr: str):
    global XmSocketAddr
    XmSocketAddr = socket_addr


def xm_set_auth_token(auth_token: str):
    global XmApiAuthToken
    XmApiAuthToken = auth_token


def xm_set_http_uri(uri: str):
    global HttpUri
    HttpUri = uri


def xm_add_http_header(header_name: str, encoded_str: str):
    HttpHeaders[header_name] = unquote(encoded_str)


def xm_add_http_query(query_param: str, value: str):
    HttpQuery[query_param] = unquote(value)


def xm_set_http_body(body: str):
    global HttpBody
    HttpBody = body


def xm_add_parsed_http_body():
    global ParsedHttpBody
    ParsedHttpBody = {}


def xm_add_parsed_http_body_item(key: str, value: str):
    ParsedHttpBody[key] = unquote(value)


# reading variables


def x_acquire_session(session_id: str):
    req = Request('http://' + XmSocketAddr + '/xhr-xapi/session/get?session=' + session_id)
    req.add_header('x-xhr-api-auth-token', XmApiAuthToken)
    try:
        resp = urlopen(req)
        resp_text = resp.read().decode('utf-8')
        return resp_text
    except:
        pass


def x_put_session_data(session_id: str, session_data: str):
    req = Request('http://' + XmSocketAddr + '/xhr-xapi/session/set?session=' + session_id,
                  data=session_data.encode('utf-8'))
    req.method = 'post'
    req.add_header('x-xhr-api-auth-token', XmApiAuthToken)
    req.add_header('content-type', 'application/octet-stream')
    urlopen(req)


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
