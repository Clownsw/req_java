package cn.smilex.req;

import java.util.HashMap;

/**
 * 请求对象
 *
 * @author smilex
 */
public class HttpRequest {
    private int method;
    private String url;
    private String body;
    private HashMap<String, String> params;
    private HashMap<String, String> headers;
    private HashMap<String, String> cookies;
    private int maxRedirect;

    private HttpRequest() {
        this.method = 0;
        body = "";
        params = new HashMap<>();
        headers = new HashMap<>();
        cookies = new HashMap<>();
        maxRedirect = 3;
    }

    public static HttpRequest build() {
        return new HttpRequest();
    }

    /* SET */
    public HttpRequest setMethod(Requests.REQUEST_METHOD method) {
        this.method = RequestUtil.requestMethodEnumToInt(method);
        return this;
    }

    public HttpRequest setUrl(String url) {
        this.url = url;
        return this;
    }

    public HttpRequest setBody(String body) {
        this.body = body;
        return this;
    }

    public HttpRequest setHeaders(HashMap<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public HttpRequest addHeader(String name, String value) {
        this.headers.put(name, value);
        return this;
    }

    public HttpRequest setCookies(HashMap<String, String> cookies) {
        this.cookies = cookies;
        return this;
    }

    public HttpRequest addCookie(String name, String value) {
        this.cookies.put(name, value);
        return this;
    }

    public HttpRequest setParams(HashMap<String, String> params) {
        this.params = params;
        return this;
    }

    public HttpRequest addParam(String name, String value) {
        this.params.put(name, value);
        return this;
    }

    public HttpRequest setMaxRedirect(int maxRedirect) {
        this.maxRedirect = maxRedirect;
        return this;
    }

    /* GET */
    public Requests.REQUEST_METHOD getMethod() {
        return RequestUtil.intToRequestMethodEnum(this.method);
    }

    public String getUrl() {
        return url;
    }

    public String getBody() {
        return body;
    }

    public HashMap<String, String> getHeaders() {
        return headers;
    }

    public HashMap<String, String> getCookies() {
        return cookies;
    }

    public HashMap<String, String> getParams() {
        return params;
    }

    public int getMaxRedirect() {
        return maxRedirect;
    }
}
