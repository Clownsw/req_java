package cn.smilex.req;

/**
 * 响应对象
 *
 * @author smilex
 */
public class HttpResponse {
    private String body;

    /* set */
    public HttpResponse setBody(String body) {
        this.body = body;
        return this;
    }

    /* get */
    public String getBody() {
        return body;
    }

    @Override
    public String toString() {
        return "HttpResponse{" +
                "body='" + body + '\'' +
                '}';
    }
}
