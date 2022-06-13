# Benchmarks

Benchmarks are performed with [wrk](https://github.com/wg/wrk). All of the servers tested serve the same static page, benchmark is ran 3 times and the best one of them is chosen.

> **FUN FACT:** Microbenchmarks like this are not a good measure for "dynamic web apps" which most websites are, in that context, your bottleneck is going to be disk I/O and database queries and a metric like requests/sec doesn't measure anything meaningful. Here, there is a very specific goal -- to serve static files and the bottlenecks can be minimized and there are no external constraints hence why this benchmark exists. :)

## Tested On:

> Linux Kernel Version: 5.17.5-76051705-generic
> 
> CPU Description: Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz
> 
> CPU ID: GenuineIntel,6,142,10
> 
> CPU Architecture: x86_64
> 
> CPUs Available:  8
> 
> Total Memory: 25.1 GB

## Results

<div align="center">
  <table>
    <tr><td><img src="https://raw.githubusercontent.com/mufeedvh/binserve/master/assets/benchmarks.jpeg" width="500"></td></tr>
  </table>
</div>

## Binserve

```
$ wrk -c 500 -t 12 -d 5s http://127.0.0.1:1337/
Running 5s test @ http://127.0.0.1:1337/
  12 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.41ms  372.48us  32.88ms   92.11%
    Req/Sec    17.07k     2.14k   38.17k    96.87%
  1030467 requests in 5.10s, 239.79MB read
Requests/sec: 202074.22
Transfer/sec:     47.02MB
```

## NGINX Tuned:

**Source:** https://github.com/denji/nginx-tuning

```
$ wrk -c 500 -t 12 -d 5s http://127.0.0.1:8081/
Running 5s test @ http://127.0.0.1:8081/
  12 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.40ms    1.87ms  34.64ms   71.69%
    Req/Sec    17.07k     5.49k   38.06k    64.67%
  1023120 requests in 5.08s, 250.76MB read
Requests/sec: 201407.17
Transfer/sec:     49.36MB
```

## NGINX Default:

```
$ wrk -c 500 -t 12 -d 5s http://127.0.0.1:8081/
Running 5s test @ http://127.0.0.1:8081/
  12 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.24ms    2.55ms  34.47ms   78.72%
    Req/Sec     9.85k     1.98k   34.39k    92.87%
  593245 requests in 5.10s, 145.37MB read
Requests/sec: 116415.16
Transfer/sec:     28.53MB
```

## Lighttpd:

```
$ wrk -c 500 -t 12 -d 5 http://127.0.0.1:7822/
Running 5s test @ http://127.0.0.1:7822/
  12 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    49.76ms  102.76ms 475.80ms   86.45%
    Req/Sec     6.45k     3.15k   42.89k    77.46%
  384892 requests in 5.07s, 83.39MB read
Requests/sec:  75915.69
Transfer/sec:     16.45MB
```

## Caddy:

```
$ wrk -c 500 -t 12 -d 5s http://localhost:2015/
Running 5s test @ http://localhost:2015/
  12 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.56ms   18.86ms 188.27ms   87.31%
    Req/Sec     5.54k     1.25k   13.05k    74.34%
  335088 requests in 5.10s, 73.18MB read
Requests/sec:  65706.52
Transfer/sec:     14.35MB
```

## Apache2:

```
$ wrk -c 500 -t 12 -d 5 http://127.0.0.1:80/
Running 5s test @ http://127.0.0.1:80/
  12 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   178.92ms  244.04ms 804.40ms   79.40%
    Req/Sec    11.89k    11.40k   36.85k    67.58%
  314836 requests in 5.07s, 71.82MB read
Requests/sec:  62124.60
Transfer/sec:     14.17MB
```
