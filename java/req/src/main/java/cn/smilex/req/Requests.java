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

    public HttpResponse request(HttpRequest req) {
        return _request(req);
    }

    private native HttpResponse _request(HttpRequest req);
}
