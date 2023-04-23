package ie.dirc.api;

import java.net.URI;
import java.net.URISyntaxException;
import java.util.Enumeration;

import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.HttpStatusCodeException;
import org.springframework.web.client.RestTemplate;
import org.springframework.web.util.UriComponentsBuilder;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

@RestController
public class APIController {

    private final int port = 8080;

    @RequestMapping(value = "{service}/**", produces = "application/json")
    public ResponseEntity<String> mirrorRest(@RequestBody(required = false) String body, @PathVariable String service, HttpMethod method, HttpServletRequest request, HttpServletResponse response)
            throws URISyntaxException {

        String requestURL = request.getRequestURL().toString();
        String servicePath = requestURL.split("/" + service)[1];

        URI uri = new URI("http", null, service, port, null, null, null);
        uri = UriComponentsBuilder.fromUri(uri)
                .path(servicePath)
                .query(request.getQueryString())
                .build(true).toUri();

        HttpHeaders headers = new HttpHeaders();
        Enumeration<String> headerNames = request.getHeaderNames();
        while (headerNames.hasMoreElements()) {
            String headerName = headerNames.nextElement();
            headers.set(headerName, request.getHeader(headerName));
        }

        HttpEntity<String> httpEntity = new HttpEntity<>(body, headers);
        RestTemplate restTemplate = new RestTemplate();
        try {
            return restTemplate.exchange(uri, method, httpEntity, String.class);
        } catch (HttpStatusCodeException e) {
            return ResponseEntity.status(e.getStatusCode())
                    .headers(e.getResponseHeaders())
                    .body(e.getResponseBodyAsString());
        }

    }
}