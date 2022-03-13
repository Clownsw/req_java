package cn.smilex.req;

/**
 * @author smilex
 */
public class Cookie {
    private String name;
    private String value;
    private String doMain;
    private String path;
    private int maxAge;
    private String expires;

    public void setName(String name) {
        this.name = name;
    }

    public void setValue(String value) {
        this.value = value;
    }

    public void setDoMain(String doMain) {
        this.doMain = doMain;
    }

    public void setPath(String path) {
        this.path = path;
    }

    public void setMaxAge(int maxAge) {
        this.maxAge = maxAge;
    }

    public void setExpires(String expires) {
        this.expires = expires;
    }

    public String getName() {
        return name;
    }

    public String getValue() {
        return value;
    }

    public String getDoMain() {
        return doMain;
    }

    public String getPath() {
        return path;
    }

    public int getMaxAge() {
        return maxAge;
    }

    public String getExpires() {
        return expires;
    }

    @Override
    public String toString() {
        return "Cookie{" +
                "name='" + name + '\'' +
                ", value='" + value + '\'' +
                ", path='" + path + '\'' +
                ", maxAge=" + maxAge +
                ", expires='" + expires + '\'' +
                '}';
    }
}
