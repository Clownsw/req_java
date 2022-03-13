package cn.smilex.req;

/**
 * @author smilex
 */
public final class Requests {

    static {
        synchronized (Requests.class) {
            System.loadLibrary("req_java");
            init();
        }
    }

    private Requests() {
    }

    public static Requests requests = new Requests();

    public enum REQUEST_METHOD {
        GET,
        POST,
        DELETE,
        PUT,
    }

    public String fast_get(String url) {
        return Requests._fast_request(url, false);
    }

    public String fast_post(String url) {
        return _fast_request(url, true);
    }

    public HttpResponse request(HttpRequest req) {
        var m = req.getHeaders();
        m.remove("cookie");
        m.put("cookie", RequestUtil.cookiesToStr(req.getCookies()));

        HttpResponse response = _request(req);
        if (response.getContentLength() == 0) {
            response.setContentLength(response.getBody().length());
        }
        response.setCookies(RequestUtil.parseHeaderCookie(response));
        return response;
    }

    private static native void init();

    private static native String _fast_request(String url, boolean isPost);

    private static native HttpResponse _request(HttpRequest req);
}
