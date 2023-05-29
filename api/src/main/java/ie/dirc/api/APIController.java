package ie.dirc.api;

import java.net.URI;
import java.net.URISyntaxException;
import java.net.UnknownHostException;
import java.util.Enumeration;

import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.ResponseEntity;
import org.springframework.http.client.HttpComponentsClientHttpRequestFactory;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.HttpStatusCodeException;
import org.springframework.web.client.RestClientException;
import org.springframework.web.client.RestTemplate;
import org.springframework.web.util.UriComponentsBuilder;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

@RestController
// @CrossOrigin(origins = "*", allowedHeaders = "*")
public class APIController {

    private final int port = 8080;

    // @CrossOrigin(origins = "*", allowedHeaders = "*")
    @RequestMapping(value = "{service}/**", produces = "application/json")
    public ResponseEntity<String> mirrorRest(@RequestBody(required = false) String body, @PathVariable String service,
            HttpMethod method, HttpServletRequest request)
            throws URISyntaxException {

        String requestURL = request.getRequestURL().toString();
        String servicePath = requestURL.split("/" + service, 2)[1];

        URI uri = new URI("http", null, service, port, null, null, null);
        uri = UriComponentsBuilder.fromUri(uri)
                .path(servicePath)
                .query(request.getQueryString())
                .build(true).toUri();

        System.out.print(method + " ");
        System.out.println(uri);
        if (body == null) {
            System.out.println("BODY = NULL");
        } else {
            System.out.println("BODY = " + body.replaceAll("\n", " "));
        }

        HttpHeaders headers = new HttpHeaders();
        Enumeration<String> headerNames = request.getHeaderNames();
        while (headerNames.hasMoreElements()) {
            String headerName = headerNames.nextElement();
            headers.set(headerName, request.getHeader(headerName));
        }

        System.out.println(headers);

        HttpEntity<String> httpEntity = new HttpEntity<>(body, headers);
        RestTemplate restTemplate = new RestTemplate(new HttpComponentsClientHttpRequestFactory());
        try {
            ResponseEntity<String> response = restTemplate.exchange(uri, method,
                    httpEntity, String.class);
            System.out.println("response: " + response);
            return response;
        } catch (HttpStatusCodeException e) {
            ResponseEntity<String> error = ResponseEntity.status(e.getStatusCode())
                    .headers(e.getResponseHeaders())
                    .body(e.getResponseBodyAsString());

            System.out.println("error: " + error);
            return error;
        } catch (RestClientException e) {
            Throwable rootCause = e.getRootCause();

            if (rootCause instanceof UnknownHostException) {
                return generate404();
            }

            return generateJSONErrorFromThrowable(rootCause);
        }

    }

    private ResponseEntity<String> generate404() {
        return ResponseEntity.status(404)
                .headers(new HttpHeaders() {
                    {
                        add("Content-Type", "application/json");
                    }
                })
                .body("{\"error\":{\"type\":\"PageNotFound\",\"message\":\"Page not found\"}}");
    }

    private ResponseEntity<String> generateJSONErrorFromThrowable(Throwable e) {
        String errorType = e.getClass().getSimpleName();
        String message = e.getMessage();

        return ResponseEntity.status(500)
                .headers(new HttpHeaders() {
                    {
                        add("Content-Type", "application/json");
                    }
                })
                .body(String.format("{\"error\":{\"type\":\"%s\",\"message\":\"%s\"}}", errorType, message));
    }
}
