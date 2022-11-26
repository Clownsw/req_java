package cn.smilex.req;

import org.apache.commons.lang3.StringUtils;

import javax.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.IdentityHashMap;
import java.util.List;

/**
 * @author smilex
 */
public final class RequestUtils {
    private static final HashMap<String, String> DEFAULT_HEADER = new HashMap<>(2);
    private static final RequestUtils REQUEST_UTILS;

    static {
        synchronized (RequestUtils.class) {
            System.loadLibrary("req_java");
            REQUEST_UTILS = new RequestUtils();
            DEFAULT_HEADER.put("content-type", "application/x-www-form-urlencoded");
            DEFAULT_HEADER.put("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36");
        }
    }

    private RequestUtils() {
    }

    public static RequestUtils getRequestUtils() {
        return REQUEST_UTILS;
    }

    public HttpResponse fast_get(String url) {
        return request(
                HttpRequest.build()
                        .setUrl(url)
                        .setMethod(HttpMethod.GET)
                        .setHeaders(DEFAULT_HEADER)
        );
    }

    public HttpResponse fast_post(String url) {
        return request(
                HttpRequest.build()
                        .setUrl(url)
                        .setMethod(HttpMethod.POST)
                        .setHeaders(DEFAULT_HEADER)
        );
    }

    public HttpResponse request(@NotNull HttpRequest req) {
        if (StringUtils.isBlank(req.getUrl()) || req.getMethod() == null) {
            return null;
        }

        HashMap<String, String> headers = req.getHeaders();
        headers.put("cookie", cookiesToStr(req.getCookies()));

        HttpResponse response = _request(req);

        try {
            if (response.getContentLength() == 0) {
                if (req.isEnableDataByte()) {
                    response.setContentLength(response.getDataByte().length);
                } else {
                    response.setContentLength(response.getBody().length());
                }
            }
        } catch (Exception ignored) {
        }

        response.setCookies(parseHeaderCookie(response));
        return response;
    }

    private static native HttpResponse _request(HttpRequest req);

    public static String cookiesToStr(HashMap<String, String> cookies) {
        if (cookies == null || cookies.size() <= 0) {
            return "";
        }

        StringBuilder str = new StringBuilder();

        for (String key : cookies.keySet()) {
            String value = cookies.get(key);
            str.append(key).append("=").append(value).append(";");
        }

        return str.toString();
    }

    public static List<HttpCookie> parseHeaderCookie(HttpResponse map) {
        IdentityHashMap<String, String> headers = map.getHeaders();
        List<HttpCookie> httpCookies = new ArrayList<>(headers.size());

        for (String key : headers.keySet()) {
            String value = headers.get(key);

            if ("set-cookie".equalsIgnoreCase(key)) {
                String[] cookieArr = value.split(";");

                if (cookieArr.length > 0) {
                    HttpCookie httpCookie = new HttpCookie();

                    int i = 0;
                    for (String cookie : cookieArr) {
                        String[] cookieInfo = cookie.split("=");

                        if (cookieInfo.length != 2) {
                            continue;
                        }

                        String cookieKey = cookieInfo[0].trim().toLowerCase();
                        String cookieValue = cookieInfo[1].trim();

                        i++;

                        if (i == 1) {
                            httpCookie.setName(cookieKey);
                            httpCookie.setValue(cookieValue);
                            continue;
                        }

                        switch (cookieKey) {
                            case "domain":
                                httpCookie.setDoMain(cookieValue);
                                break;

                            case "max-age":
                                httpCookie.setMaxAge(Integer.parseInt(cookieValue));
                                break;

                            case "expires":
                                httpCookie.setExpires(cookieValue);
                                break;

                            case "path":
                                httpCookie.setPath(cookieValue);
                                break;
                        }
                    }
                    httpCookies.add(httpCookie);
                }
            }
        }

        return httpCookies;
    }
}
