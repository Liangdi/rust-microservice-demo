package me.liangdi.cloud.rustmicroserviceconsumer.web;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

@RestController
@RequestMapping("/api/rest")
public class RestTemplateController {
    public static final String SERVICE = "rust-microservice";
    @Autowired
    private RestTemplate restTemplate;

    @RequestMapping("/foo")
    public String foo() {
        return restTemplate.getForObject("http://"+SERVICE+"/foo",String.class);
    }
    @RequestMapping("/bar")
    public Object bar() {
        return restTemplate.getForObject("http://"+SERVICE+"/bar",Object.class);
    }
}
