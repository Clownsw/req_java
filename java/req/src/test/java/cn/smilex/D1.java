package cn.smilex;

import cn.smilex.req.HttpRequest;
import cn.smilex.req.HttpResponse;
import cn.smilex.req.Requests;
import org.junit.Test;

import java.util.HashMap;

/**
 * @author smilex
 */
public class D1 {

    static {
        System.loadLibrary("req_java");
    }

    /**
     * 测试请求
     */
    @Test
    public void testRequest() {
        Requests req = new Requests();

        HashMap<String, String> headers = new HashMap<>();
        headers.put("a", "b");

        HttpRequest httpRequest = HttpRequest.build()
                .setUrl("https://www.google.com.hk/")
                .setMethod(Requests.REQUEST_METHOD.GET)
                .setHeaders(headers);

        HttpResponse resp = req.request(httpRequest);
    }

    @Test
    public void testFastRequest() {
        Requests req = new Requests();

        System.out.println(req.fast_get("https://www.google.com.hk/"));
        System.out.println(req.fast_post("https://www.google.com.hk/"));
    }
}
