Okay this is what we have:

```text
  19663 keywords.txt
 100000 wordlist.txt
```

There are 22455 words in the wordlist to be replaced by one of the 19663
keywords.
The replacement string is slightly longer and therefore not suitable for
in-place replacement.
Output with replacement done is written to stdout, and a time written to
stderr.

```bash
cargo build && ./target/debug/fastsearchandreplace < wordlist.txt | wc -l
```

```text
   Compiling fastsearchandreplace v0.1.0 (file://[…])
    Finished dev [unoptimized + debuginfo] target(s) in 4.61 secs
time: 0.308494858
100000
```

