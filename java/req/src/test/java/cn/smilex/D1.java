package cn.smilex;

import cn.smilex.blacksky.jni.json.Json;
import cn.smilex.req.Req;
import org.junit.Test;

/**
 * @author smilex
 */
public class D1 {

    static {
        System.loadLibrary("req_java");
    }

    @Test
    public void t1() {
        Req req = new Req();
        System.out.println(req.req_get("https://www.google.com.hk/"));
    }

    @Test
    public void t2() {
        Req req = new Req();
        System.out.println(req.req_post("https://www.google.com.hk/"));
    }

    @Test
    public void t3() {
        Req req = new Req();

        String id = "7070144802628570409";

        Json json = new Json(req.req_get("https://www.iesdouyin.com/web/api/v2/aweme/iteminfo/?item_ids=" + id));

        String url = json.getPointer("/item_list/0/video/play_addr/url_list/0").asPointerString();
        System.out.println("有水印: " + url);

        String url2 = url.replace("playwm", "play");
        System.out.println("无水印: " + url2);

        json.close();
    }

    @Test
    public void t4() {
        Req req = new Req();
        System.out.println(req.req_get("https://v.douyin.com/NNxAfce/"));
    }
}
