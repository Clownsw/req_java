package cn.smilex;

import cn.smilex.req.HttpRequest;
import cn.smilex.req.HttpResponse;
import cn.smilex.req.Requests;
import org.junit.Test;

import java.util.HashMap;
import java.util.Map;

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
        headers.put("c", "d");
        headers.put("e", "f");

        HttpRequest httpRequest = HttpRequest.build()
//                .setUrl("https://www.google.com.hk/")
                .setUrl("http://localhost:8888/test")
                .setMethod(Requests.REQUEST_METHOD.GET)
                .setHeaders(headers);

        HttpResponse resp = req.request(httpRequest);
        System.out.println(resp.getBody());
    }

    @Test
    public void testFastRequest() {
        Requests req = new Requests();

        System.out.println(req.fast_get("https://www.baidu.com/"));
        System.out.println(req.fast_post("https://www.smilex.cn/"));
    }

    @Test
    public void test() throws InterruptedException {
        var t = new Runnable() {
            @Override
            public void run() {
                Map map = Requests.getMap();
                System.out.println(map);
            }
        };

        var t1 = new Thread(t);
        var t2 = new Thread(t);

        t1.start();
        t2.start();

        t1.join();
        t2.join();
    }
}
