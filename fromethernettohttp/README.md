On a Raspberry Pi (`192.168.2.152`)
```
sudo python3 l2_http_server.py
```

Then on a laptop on the same network
```
$ curl -v 192.168.2.152:8080
*   Trying 192.168.2.152:8080...
* Connected to 192.168.2.152 (192.168.2.152) port 8080 (#0)
> GET / HTTP/1.1
> Host: 192.168.2.152:8080
> User-Agent: curl/7.84.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< Server: MySimpleServer/1.0
< Content-Type: text/plain
< Connection: close
< Content-Length: 13
< 
Hello World
* Closing connection 0
```
