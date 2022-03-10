package cn.smilex.req;

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

            case 4: {
                return Requests.REQUEST_METHOD.TRACE;
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

            case TRACE: {
                return 4;
            }
        }
    }
}
