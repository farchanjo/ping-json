This project is for ping for json

Build for project:
```shell
cargo b -r
```

After build you have to copy:
```shell
cp -Rpv target/release/ping-json /usr/local/bin/
```

You can test it like that:
```shell
ping-json -c 6 200.150.150.1
```

This is the json result is:
```json
{"usec":20216,"time_string":"20.216216ms","timeout":false,"ip":"200.150.150.1","seq":1}
{"usec":19844,"time_string":"19.844731ms","timeout":false,"ip":"200.150.150.1","seq":2}
{"usec":19673,"time_string":"19.673384ms","timeout":false,"ip":"200.150.150.1","seq":3}
{"usec":19538,"time_string":"19.538368ms","timeout":false,"ip":"200.150.150.1","seq":4}
{"usec":20158,"time_string":"20.158634ms","timeout":false,"ip":"200.150.150.1","seq":5}
```