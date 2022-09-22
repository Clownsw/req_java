package cn.smilex.req;

import java.util.Objects;

/**
 * @author YangLuJia
 * @date 2022/9/22 17:03
 */
public class HttpStringBody implements HttpBodyInterface {
    private final String content;

    protected HttpStringBody(String value) {
        this.content = value;
    }

    public String getContent() {
        return content;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        HttpStringBody that = (HttpStringBody) o;
        return Objects.equals(content, that.content);
    }

    @Override
    public int hashCode() {
        return Objects.hash(content);
    }

    @Override
    public String toString() {
        return "HttpStringBody{" +
                "content='" + content + '\'' +
                '}';
    }
}
