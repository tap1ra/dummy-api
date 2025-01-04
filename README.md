# dummy-api with rust

## how to use
```sh
$ cargo run -- --error-rate 30 --ports 3333 --data-file example.json

$ while : ; do curl -D - -s -o /dev/null http://127.0.0.1:3333/users ; sleep 1 ; done
HTTP/1.1 500 Internal Server Error
content-type: application/json
content-length: 48
date: Sat, 04 Jan 2025 08:46:01 GMT

HTTP/1.1 200 OK
content-type: application/json
content-length: 68
date: Sat, 04 Jan 2025 08:46:02 GMT

HTTP/1.1 500 Internal Server Error
content-type: application/json
content-length: 48
date: Sat, 04 Jan 2025 08:46:04 GMT

HTTP/1.1 200 OK
content-type: application/json
content-length: 68
date: Sat, 04 Jan 2025 08:46:05 GMT

HTTP/1.1 200 OK
content-type: application/json
content-length: 68
date: Sat, 04 Jan 2025 08:46:06 GMT
```