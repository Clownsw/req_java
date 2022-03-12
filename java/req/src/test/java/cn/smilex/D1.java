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

        HttpRequest httpRequest = HttpRequest.build()
                .setUrl("http://localhost:8080/test")
                .setMethod(Requests.REQUEST_METHOD.GET)
                .addHeader("a", "b")
                .addCookie("name", "xuda")
                .addCookie("age", "18")
                .setBody("this is Body");

        HttpResponse resp = req.request(httpRequest);
        System.out.println(resp.getBody());
    }

    @Test
    public void testFastRequest() {
        Requests req = new Requests();

        System.out.println(req.fast_get("https://www.baidu.com/"));
        System.out.println(req.fast_post("https://www.smilex.cn/"));
    }
}
