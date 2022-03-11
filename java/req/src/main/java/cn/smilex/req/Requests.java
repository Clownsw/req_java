package cn.smilex.req;

import java.util.Map;

/**
 * @author smilex
 */
public class Requests {

    static {
        synchronized (Requests.class) {
            System.loadLibrary("req_java");
            init();
        }
    }

    public enum REQUEST_METHOD {
        GET,
        POST,
        DELETE,
        PUT,
        TRACE,
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

    public static native Map getMap();
    private static native void init();
    private native String _fast_request(String url, boolean isPost);
    private native HttpResponse _request(HttpRequest req);
}
