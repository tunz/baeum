
Currently **unstable**. Current version is just like other evolutionary fuzzers.

# Usage

```
$ git clone https://github.com/tunz/baeum
$ cd baeum
$ make
...
$ ./baeum -i seeds -o output -- /path/to/target/program arg1 arg2 ...
Web Server: http://0.0.0.0:8000
...
```





## Reference
Inspired by [AFL](http://lcamtuf.coredump.cx/afl/),
[LibFuzzer](http://llvm.org/docs/LibFuzzer.html), and [honggfuzz](https://github.com/google/honggfuzz).
