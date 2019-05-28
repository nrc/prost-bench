# Prost microbenchmarks

```
sudo dtrace -c target/release/proto -x ustackframes=100 -n 'profile-97 / arg1 / { @[ustack()] = count(); }' -o out.user_stacks
~/version-controlled/FlameGraph/stackcollapse.pl out.user_stacks >out.folded
~/version-controlled/FlameGraph/flamegraph.pl out.folded > proto.svg
```

`cargo run --release --bin proto`

```
perf record -F 999 ./proto
perf script > out.perf
```

```
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-m
Roundtripped 1000 medium messages, time: 2638ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-m
Roundtripped 1000 medium messages, time: 2644ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-m
Roundtripped 1000 medium messages, time: 2640ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-opt
Roundtripped 1000 medium messages, time: 2436ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-opt
Roundtripped 1000 medium messages, time: 2448ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-opt
Roundtripped 1000 medium messages, time: 2433ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-m
Roundtripped 1000 medium messages, time: 2643ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-opt
Roundtripped 1000 medium messages, time: 2431ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-m
Roundtripped 1000 medium messages, time: 2644ms. 1000
root@ubuntu-c-32-64gib-sfo2-01:~/prost-bench# ./proto-opt
Roundtripped 1000 medium messages, time: 2433ms. 1000
```

2641
2439

7.6% improvement



```
google_message1_proto2/decode
                        time:   [1.4103 us 1.4113 us 1.4123 us]
                        thrpt:  [153.96 MiB/s 154.07 MiB/s 154.18 MiB/s]
                 change:
                        time:   [+10.498% +10.836% +11.170%] (p = 0.00 < 0.05)
                        thrpt:  [-10.048% -9.7765% -9.5009%]
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

google_message1_proto2/merge
                        time:   [404.32 ns 404.33 ns 404.35 ns]
                        thrpt:  [537.75 MiB/s 537.77 MiB/s 537.79 MiB/s]
                 change:
                        time:   [-3.0358% -2.9766% -2.9228%] (p = 0.00 < 0.05)
                        thrpt:  [+3.0108% +3.0679% +3.1309%]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

google_message1_proto3/encode
                        time:   [238.32 ns 238.34 ns 238.35 ns]
                        thrpt:  [884.24 MiB/s 884.30 MiB/s 884.36 MiB/s]
                 change:
                        time:   [+3.5930% +3.6419% +3.6716%] (p = 0.00 < 0.05)
                        thrpt:  [-3.5416% -3.5139% -3.4684%]
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe

google_message1_proto3/decode
                        time:   [1.3873 us 1.3918 us 1.3966 us]
                        thrpt:  [155.70 MiB/s 156.22 MiB/s 156.74 MiB/s]
                 change:
                        time:   [+0.2507% +0.6007% +0.9875%] (p = 0.00 < 0.05)
                        thrpt:  [-0.9779% -0.5971% -0.2501%]
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

google_message1_proto3/merge
                        time:   [336.59 ns 336.61 ns 336.64 ns]
                        thrpt:  [645.91 MiB/s 645.95 MiB/s 645.99 MiB/s]
                 change:
                        time:   [-0.3088% -0.2109% -0.0298%] (p = 0.00 < 0.05)
                        thrpt:  [+0.0298% +0.2114% +0.3098%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

     Running target/release/deps/varint-81c61a0070a5f103

running 12 tests
test decode_varint_large       ... bench:         793 ns/iter (+/- 5) = 1008 MB/s
test decode_varint_medium      ... bench:         454 ns/iter (+/- 0) = 1762 MB/s
test decode_varint_mixed       ... bench:         505 ns/iter (+/- 1) = 1584 MB/s
test decode_varint_small       ... bench:         290 ns/iter (+/- 0) = 2758 MB/s
```
master
```
test decode_varint_large       ... bench:       1,000 ns/iter (+/- 1) = 800 MB/s
test decode_varint_medium      ... bench:         662 ns/iter (+/- 0) = 1208 MB/s
test decode_varint_mixed       ... bench:         760 ns/iter (+/- 2) = 1052 MB/s
test decode_varint_small       ... bench:         310 ns/iter (+/- 1) = 2580 MB/s
```

opt round 2:

```

running 12 tests
test decode_varint_large       ... bench:         786 ns/iter (+/- 3) = 1017 MB/s
test decode_varint_medium      ... bench:         454 ns/iter (+/- 2) = 1762 MB/s
test decode_varint_mixed       ... bench:         511 ns/iter (+/- 11) = 1565 MB/s
test decode_varint_small       ... bench:         290 ns/iter (+/- 0) = 2758 MB/s
```
