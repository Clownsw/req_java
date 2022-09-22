package cn.smilex;

import cn.smilex.req.HttpRequest;
import cn.smilex.req.HttpResponse;
import cn.smilex.req.Requests;
import org.junit.Test;

import java.io.*;

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
//                .setBody("this is Body")
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
                .setUrl("https://h5.pipix.com/s/NrGdANK/")
                .setMaxRedirect(0);

        HttpResponse response = Requests.requests.request(httpRequest);
        System.out.println(response.getBody());
        System.out.println(response.getRedirectUrls());
    }

    @Test
    public void testFastRequest() {

        System.out.println(Requests.requests.fast_get("https://www.baidu.com/"));
        System.out.println(Requests.requests.fast_post("https://www.smilex.cn/"));
    }

    @Test
    public void testDownloadAudio() throws IOException {
        HttpRequest httpRequest = HttpRequest.build()
                .setMethod(Requests.REQUEST_METHOD.GET)
                .setUrl("http://freetyst.nf.migu.cn/public/ringmaker01/dailyring02/2015/07/2015%E5%B9%B407%E6%9C%8801%E6%97%A5%E7%9B%9B%E5%94%90%E6%97%B6%E7%A9%BA%E5%86%85%E5%AE%B9%E5%87%86%E5%85%A5327%E9%A6%96/%E6%AD%8C%E6%9B%B2%E4%B8%8B%E8%BD%BD/MP3_320_16_Stero/%E9%9C%87%E5%8A%A8%E6%8B%89%E9%94%AF%E7%81%AB%E7%81%AB%E7%81%AB-7%E5%A6%B9.mp3?channelid=02&msisdn=44144151-e193-4685-9b44-38e0cf547454&Tim=1648121283717&Key=cc7bbb7ed0048c48")
                .setEnableDataByte(true);

        HttpResponse httpResponse = Requests.requests.request(httpRequest);
        System.out.println(httpResponse.getDataByte().length);

        File file = new File("D:\\1.mp3");
        FileOutputStream fos = new FileOutputStream(file);
        fos.write(httpResponse.getDataByte(), 0, httpResponse.getDataByte().length);
        fos.flush();
        fos.close();
    }

    @Test
    public void testDownloadImage() throws IOException {
        HttpRequest httpRequest = HttpRequest.build()
                .setMethod(Requests.REQUEST_METHOD.GET)
                .setUrl("https://w.wallhaven.cc/full/v9/wallhaven-v9v3r5.jpg")
                .setEnableDataByte(true);

        HttpResponse httpResponse = Requests.requests.request(httpRequest);
        System.out.println(httpResponse.getDataByte().length);

        File file = new File("D:\\wallhaven-v9v3r5.jpg");
        FileOutputStream fos = new FileOutputStream(file);
        fos.write(httpResponse.getDataByte(), 0, httpResponse.getDataByte().length);
        fos.flush();
        fos.close();
    }
}
