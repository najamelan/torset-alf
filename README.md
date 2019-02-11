# Just the code I used to fuzz test torset

## LLVM bugs

https://github.com/rust-lang/rust/issues/53945

in fish shell:

```fish
cargo install afl

set -x RUSTFLAGS $RUSTFLAGS -C codegen-units=1 -C incremental=fuzz-incremental

cargo afl build --release
cargo afl fuzz -i in -o out target/release/torset-afl
```

## Needed before starting:

[-] Hmm, your system is configured to send core dump notifications to an
    external utility. This will cause issues: there will be an extended delay
    between stumbling upon a crash and having this information relayed to the
    fuzzer via the standard waitpid() API.

    To avoid having crashes misinterpreted as timeouts, please log in as root
    and temporarily modify /proc/sys/kernel/core_pattern, like so:

    echo core >/proc/sys/kernel/core_pattern

revert: echo "|/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h %e" > /proc/sys/kernel/core_pattern


[-] Whoops, your system uses on-demand CPU frequency scaling, adjusted
    between 781 and 3222 MHz. Unfortunately, the scaling algorithm in the
    kernel is imperfect and can miss the short-lived processes spawned by
    afl-fuzz. To keep things moving, run these commands as root:

    cd /sys/devices/system/cpu
    echo performance | tee cpu*/cpufreq/scaling_governor

    You can later go back to the original state by replacing 'performance' with
    'ondemand'. If you don't want to change the settings, set AFL_SKIP_CPUFREQ
    to make afl-fuzz skip this check - but expect some performance drop.

## Performance

Run in parallel!

## Results

are in out/fuzzer_stats
