package cn.smilex.req;

import java.util.ArrayList;
import java.util.IdentityHashMap;
import java.util.List;

/**
 * @author smilex
 */
public class HttpResponse {
    private String statusCode;
    private String body;
    private String version;
    private IdentityHashMap<String, String> headers;
    private List<HttpCookie> httpCookies;
    private List<String> redirectUrls;
    private long contentLength;
    private String remoteAddress;
    private byte[] dataByte;

    public HttpResponse() {
        headers = new IdentityHashMap<>();
        httpCookies = new ArrayList<>();
        redirectUrls = new ArrayList<>();
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

    public void setCookies(List<HttpCookie> httpCookies) {
        this.httpCookies = httpCookies;
    }

    public void setRemoteAddress(String remoteAddress) {
        this.remoteAddress = remoteAddress;
    }

    public void setRedirectUrls(List<String> redirectUrls) {
        this.redirectUrls = redirectUrls;
    }

    public void setDataByte(byte[] dataByte) {
        this.dataByte = dataByte;
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

    public List<HttpCookie> getCookies() {
        return httpCookies;
    }

    public String getRemoteAddress() {
        return remoteAddress;
    }

    public List<String> getRedirectUrls() {
        return redirectUrls;
    }

    public byte[] getDataByte() {
        return dataByte;
    }

    @Override
    public String toString() {
        return "HttpResponse{" +
                "statusCode='" + statusCode + '\'' +
                ", body='" + body + '\'' +
                ", version='" + version + '\'' +
                ", headers=" + headers +
                ", cookies=" + httpCookies +
                ", redirectUrls=" + redirectUrls +
                ", contentLength=" + contentLength +
                ", remoteAddress='" + remoteAddress + '\'' +
                '}';
    }
}
