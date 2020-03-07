# Rust 微服务 demo

## demo 1
使用 nacos 做为服务发现程序， java 程序调用 rust 微服务

### nacos 
nacos 是服务发现，配置管理，服务管理工具

[https://nacos.io/](https://nacos.io/)

### rust 微服务
- 使用 actix-web 作为 web 框架
- 端口: 8080

### java 服务消费程序
- 使用 spring boot 框架
- 通过 nacos java sdk
- 使用 openfeign client
- 端口 8000

### todo
- [ ] docker / podman config
- [ ] 更多业务逻辑
- [ ] nacos rust sdk
