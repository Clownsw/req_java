package cn.smilex.req;

/**
 * @author smilex
 */
public class HttpBodyBuilder {
    private HttpBodyBuilder() {
    }

    public static HttpBodyInterface ofString(String value) {
        return new HttpStringBody(value);
    }

    public static HttpBodyInterface ofByteArr(byte[] value) {
        return new HttpByteArrBody(value);
    }
}
