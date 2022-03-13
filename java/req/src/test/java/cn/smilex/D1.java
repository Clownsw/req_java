package cn.smilex;

import cn.smilex.req.HttpRequest;
import cn.smilex.req.HttpResponse;
import cn.smilex.req.Requests;
import org.junit.Test;

/**
 * @author smilex
 */
public class D1 {

    /**
     * 测试请求
     */
    @Test
    public void testRequest() {
        HttpRequest httpRequest = HttpRequest.build()
                .setUrl("http://localhost:8080/test")
                .setMethod(Requests.REQUEST_METHOD.GET)
                .addHeader("a", "b")
                .addCookie("name", "x")
                .addCookie("age", "100")
                .setBody("this is Body")
                .addParam("name", "x")
                .addParam("age", "100");

        HttpResponse resp = Requests.requests.request(httpRequest);
        System.out.println(resp.getCookies());
    }

    @Test
    public void testRequest2() {
        HttpRequest httpRequest = HttpRequest.build()
                .setMethod(Requests.REQUEST_METHOD.GET)
                .setUrl("https://www.smilex.cn/");
//                .setUrl("https://www.baidu.com/");

        HttpResponse resp = Requests.requests.request(httpRequest);

        System.out.println(resp);
//        System.out.println(resp.getHeaders());
//        List<Cookie> cookies = resp.getCookies();
//        for (Cookie cookie : cookies) {
//            System.out.println(cookie);
//        }
    }

    @Test
    public void redirectTest() {
        HttpRequest httpRequest = HttpRequest.build()
                .setMethod(Requests.REQUEST_METHOD.GET)
                .setUrl("https://rustc.ml/eXyZSn/");

        HttpResponse response = Requests.requests.request(httpRequest);
        System.out.println(response.getBody());
    }

    @Test
    public void testFastRequest() {

        System.out.println(Requests.requests.fast_get("https://www.baidu.com/"));
        System.out.println(Requests.requests.fast_post("https://www.smilex.cn/"));
    }
}
