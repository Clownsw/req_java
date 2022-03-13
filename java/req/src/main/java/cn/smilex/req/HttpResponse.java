package cn.smilex.req;

import java.util.HashMap;
import java.util.IdentityHashMap;

/**
 * 响应对象
 *
 * @author smilex
 */
public class HttpResponse {
    private String statusCode;
    private String body;
    private String version;
    private IdentityHashMap<String, String> headers;
    private HashMap<String, String> cookies;
    private long contentLength;
    private String remoteAddress;

    public HttpResponse() {
        headers = new IdentityHashMap<>();
        cookies = new HashMap<>();
    }

    /* set */
    public HttpResponse setBody(String body) {
        this.body = body;
        return this;
    }

    public void setStatusCode(String statusCode) {
        this.statusCode = statusCode;
    }

    public void setVersion(String version) {
        this.version = version;
    }

    public void setHeaders(IdentityHashMap<String, String> headers) {
        this.headers = headers;
    }

    public void setContentLength(long contentLength) {
        this.contentLength = contentLength;
    }

    public void setCookies(HashMap<String, String> cookies) {
        this.cookies = cookies;
    }

    public void setRemoteAddress(String remoteAddress) {
        this.remoteAddress = remoteAddress;
    }

    /* get */
    public String getBody() {
        return body;
    }

    public String getStatusCode() {
        return statusCode;
    }

    public String getVersion() {
        return version;
    }

    public IdentityHashMap<String, String> getHeaders() {
        return headers;
    }

    public long getContentLength() {
        return contentLength;
    }

    public HashMap<String, String> getCookies() {
        return cookies;
    }

    public String getRemoteAddress() {
        return remoteAddress;
    }

    @Override
    public String toString() {
        return "HttpResponse{" +
                "statusCode='" + statusCode + '\'' +
                ", body='" + body + '\'' +
                ", version='" + version + '\'' +
                ", headers=" + headers +
                ", cookies=" + cookies +
                ", contentLength=" + contentLength +
                ", remoteAddress='" + remoteAddress + '\'' +
                '}';
    }
}
