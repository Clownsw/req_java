package cn.smilex.req;

import java.util.Arrays;

/**
 * @author smilex
 */
public class HttpByteArrBody implements HttpBodyInterface {
    private final byte[] content;

    protected HttpByteArrBody(byte[] value) {
        this.content = value;
    }

    public byte[] getContent() {
        return content;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        HttpByteArrBody that = (HttpByteArrBody) o;
        return Arrays.equals(content, that.content);
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(content);
    }

    @Override
    public String toString() {
        return "HttpByteArrBody{" +
                "content=" + Arrays.toString(content) +
                '}';
    }
}
