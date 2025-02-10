# Simple Benchmark

## macOS

input text: enwiki-20231101-pages-articles-multistream-index.txt

| tool                              | elapsed | rate     | ratio |
|:---------------------------------:|:-------:|:--------:|:-----:|
| rs-find-prefix-ascii(native)      | 2.62s   | 418 MB/s | 2.1x  |
| rs-find-prefix-ascii(wasi/wazero) | 3.00s   | 357 MB/s | 1.8x  |
| grep                              | 5.50s   | 200 MB/s | (1x)  |
