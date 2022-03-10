package cn.smilex;

import cn.smilex.req.HttpRequest;
import cn.smilex.req.HttpResponse;
import cn.smilex.req.Requests;
import org.junit.Test;

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
        HttpRequest httpRequest = HttpRequest.build()
                .setUrl("https://www.google.com.hk/")
                .setMethod(Requests.REQUEST_METHOD.GET);

        HttpResponse resp = req.request(httpRequest);
        System.out.println(resp);
    }
}
