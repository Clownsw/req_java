package cn.smilex.req;

/**
 * @author YangLuJia
 * @date 2022/9/22 17:06
 */
public class HttpBodyBuilder {
    private HttpBodyBuilder() {
    }

    public static HttpBodyInterface ofString(String value) {
        return new HttpStringBody(value);
    }

    public static HttpBodyInterface ofByteArrBody(byte[] value) {
        return new HttpByteArrBody(value);
    }
}
