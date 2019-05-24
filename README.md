# Prost microbenchmarks

```
sudo dtrace -c target/release/proto -x ustackframes=100 -n 'profile-97 / arg1 / { @[ustack()] = count(); }' -o out.user_stacks
~/version-controlled/FlameGraph/stackcollapse.pl out.user_stacks >out.folded
~/version-controlled/FlameGraph/flamegraph.pl out.folded > proto.svg
```

`cargo run --release --bin proto`
