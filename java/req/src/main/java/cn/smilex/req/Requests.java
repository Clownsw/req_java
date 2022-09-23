package cn.smilex.req;

import javax.validation.constraints.NotNull;
import java.util.HashMap;

/**
 * @author smilex
 */
public final class Requests {

    static {
        synchronized (Requests.class) {
            System.loadLibrary("req_java");
        }
    }

    private Requests() {
    }

    private static final HashMap<String, String> DEFAULT_HEADER = new HashMap<>();
    public static Requests requests = new Requests();

    static {
        DEFAULT_HEADER.put("content-type", "application/x-www-form-urlencoded");
        DEFAULT_HEADER.put("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36");
    }

    public enum REQUEST_METHOD {
        GET,
        POST,
        DELETE,
        PUT,
    }

    public HttpResponse fast_get(String url) {
        return request(
                HttpRequest.build()
                        .setUrl(url)
                        .setMethod(REQUEST_METHOD.GET)
                        .setHeaders(DEFAULT_HEADER)
        );
    }

    public HttpResponse fast_post(String url) {
        return request(
                HttpRequest.build()
                        .setUrl(url)
                        .setMethod(REQUEST_METHOD.POST)
                        .setHeaders(DEFAULT_HEADER)
        );
    }

    public HttpResponse request(@NotNull HttpRequest req) {
        if (req.getUrl() == null || req.getUrl().isBlank() || req.getMethod() == null) {
            return null;
        }

        var headers = req.getHeaders();
        headers.put("cookie", RequestUtil.cookiesToStr(req.getCookies()));

        HttpResponse response = _request(req);
        if (response.getContentLength() == 0) {
            response.setContentLength(response.getBody().length());
        }
        response.setCookies(RequestUtil.parseHeaderCookie(response));
        return response;
    }

    private static native HttpResponse _request(HttpRequest req);
}
