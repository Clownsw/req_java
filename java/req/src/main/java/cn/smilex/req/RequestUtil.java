package cn.smilex.req;

import java.util.*;

/**
 * 工具类
 *
 * @author smilex
 */
public final class RequestUtil {
    public static Requests.REQUEST_METHOD intToRequestMethodEnum(int method) {
        switch (method) {
            default:
            case 0: {
                return Requests.REQUEST_METHOD.GET;
            }

            case 1: {
                return Requests.REQUEST_METHOD.POST;
            }

            case 2: {
                return Requests.REQUEST_METHOD.PUT;
            }

            case 3: {
                return Requests.REQUEST_METHOD.DELETE;
            }
        }
    }

    public static int requestMethodEnumToInt(Requests.REQUEST_METHOD method) {
        switch (method) {
            default:
            case GET: {
                return 0;
            }

            case POST: {
                return 1;
            }

            case PUT: {
                return 2;
            }

            case DELETE: {
                return 3;
            }
        }
    }

    public static String cookiesToStr(HashMap<String, String> cookies) {
        if (cookies.size() <= 0) {
            return "";
        }

        StringBuilder str = new StringBuilder();

        for (String key : cookies.keySet()) {
            String value = cookies.get(key);
            str.append(key).append("=").append(value).append(";");
        }

        return str.toString();
    }

    public static List<Cookie> parseHeaderCookie(HttpResponse map) {
        List<Cookie> cookies = new ArrayList<>();
        IdentityHashMap<String, String> headers = map.getHeaders();

        for (String key : headers.keySet()) {
            var value = headers.get(key);

            if (key.equals("set-cookie") || key.equals("Set-Cookie")) {
                String[] split = value.split(";");
                Cookie cookie = new Cookie();

                if (split.length > 0) {

                    int i = 0;
                    for (String s : split) {
                        String[] split1 = s.split("=");

                        String k = split1[0].trim();
                        String v = split1[1].trim();

                        i++;

                        if (i == 1) {
                            cookie.setName(k);
                            cookie.setValue(v);
                            continue;
                        }

                        switch (k) {
                            case "domain":
                            case "Domain":
                                cookie.setDoMain(v);
                                break;

                            case "max-age":
                            case "Max-Age":
                                cookie.setMaxAge(Integer.parseInt(v));
                                break;
                            case "expires":
                            case "Expires":
                                cookie.setExpires(v);
                                break;

                            case "path":
                            case "Path":
                                cookie.setPath(v);
                                break;
                        }
                    }

                    cookies.add(cookie);
                }
            }
        }

        return cookies;
    }
}
