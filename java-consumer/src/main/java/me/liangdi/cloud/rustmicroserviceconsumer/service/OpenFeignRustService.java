package me.liangdi.cloud.rustmicroserviceconsumer.service;

import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.stereotype.Component;
import org.springframework.web.bind.annotation.GetMapping;

import java.util.HashMap;

@FeignClient(name = "rust-microservice",fallback = OpenFeignRustService.FallbackService.class)
public interface OpenFeignRustService {
    @GetMapping("/foo")
    public String foo();
    @GetMapping("/bar")
    public Object bar();
    @Component
    public static class FallbackService implements OpenFeignRustService{

        @Override
        public String foo() {
            return "foo fallback";
        }

        @Override
        public Object bar() {
            return new HashMap<>();
        }
    }
}

