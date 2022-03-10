package cn.smilex.req;

import java.util.HashMap;

/**
 * 请求对象
 *
 * @author smilex
 */
public class HttpRequest {
    private int id;
    private int method;
    private String url;
    private HashMap<String, String> headers;
    private HashMap<String, String> cookies;

    private HttpRequest() {
        this.method = 0;
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

    public HttpRequest setHeaders(HashMap<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public HttpRequest setId(int id) {
        this.id = id;
        return this;
    }

    public HttpRequest setCookies(HashMap<String, String> cookies) {
        this.cookies = cookies;
        return this;
    }

    /* GET */
    public Requests.REQUEST_METHOD getMethod() {
        return RequestUtil.intToRequestMethodEnum(this.method);
    }

    public String getUrl() {
        return url;
    }

    public HashMap<String, String> getHeaders() {
        return headers;
    }

    public int getId() {
        return id;
    }

    public HashMap<String, String> getCookies() {
        return cookies;
    }
}
