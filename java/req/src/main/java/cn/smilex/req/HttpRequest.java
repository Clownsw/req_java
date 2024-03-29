package cn.smilex.req;

import java.time.Duration;
import java.util.HashMap;

/**
 * @author smilex
 */
public class HttpRequest {
    private HttpMethod method;
    private String url;
    private HttpBodyInterface body;
    private HashMap<String, String> params;
    private HashMap<String, String> headers;
    private HashMap<String, String> cookies;
    private long timeOut;
    private int maxRedirect;
    private boolean enableDataByte;

    private HttpRequest() {
        this.method = HttpMethod.GET;
        this.body = null;
        params = new HashMap<>();
        headers = new HashMap<>();
        cookies = new HashMap<>();
        maxRedirect = 3;
        enableDataByte = false;
    }

    public static HttpRequest build() {
        return new HttpRequest();
    }

    /**
     * SET
     */
    public HttpRequest setMethod(HttpMethod method) {
        this.method = method;
        return this;
    }

    public HttpRequest setUrl(String url) {
        this.url = url;
        return this;
    }

    public HttpRequest setBody(HttpBodyInterface body) {
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

    public HttpRequest setEnableDataByte(boolean enableDataByte) {
        this.enableDataByte = enableDataByte;
        return this;
    }

    public HttpRequest setTimeOut(Duration timeOut) {
        this.timeOut = timeOut.toMillis();
        return this;
    }

    /**
     * GET
     */
    public HttpMethod getMethod() {
        return this.method;
    }

    public String getUrl() {
        return url;
    }

    public HttpBodyInterface getBody() {
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

    public Duration getTimeOut() {
        return Duration.ofMillis(this.timeOut);
    }

    public boolean isEnableDataByte() {
        return enableDataByte;
    }
}
