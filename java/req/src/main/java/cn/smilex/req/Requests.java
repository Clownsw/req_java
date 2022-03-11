package cn.smilex.req;

/**
 * @author smilex
 */
public class Requests {

    public enum REQUEST_METHOD {
        GET,
        POST,
        DELETE,
        PUT,
        TRACE,
    }

    static {
        System.loadLibrary("req_java");
    }

    public String fast_get(String url) {
        return _fast_request(url, false);
    }

    public String fast_post(String url) {
        return _fast_request(url, true);
    }

    public HttpResponse request(HttpRequest req) {
        return _request(req);
    }

    private native String _fast_request(String url, boolean isPost);

    private native HttpResponse _request(HttpRequest req);
}
