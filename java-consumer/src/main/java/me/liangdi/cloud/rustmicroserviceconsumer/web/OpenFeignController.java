package me.liangdi.cloud.rustmicroserviceconsumer.web;

import me.liangdi.cloud.rustmicroserviceconsumer.service.OpenFeignRustService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/feign")
public class OpenFeignController {
    @Autowired
    OpenFeignRustService rustService;

    @RequestMapping("/foo")
    public String foo() {
        return rustService.foo();
    }
    @RequestMapping("/bar")
    public Object bar() {
        return rustService.bar();
    }
}
