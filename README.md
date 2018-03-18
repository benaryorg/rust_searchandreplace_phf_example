# code

This is doing single threaded search&replace in linear time (~1 second per ⅓
million @ 2.60GHz).
Those numbers already include all the I/O to be done for the replacement.

```bash
cargo build && ./target/debug/fastsearchandreplace < wordlist.txt | wc -l
```

```text
   Compiling fastsearchandreplace v0.1.0 (file://[…])
    Finished dev [unoptimized + debuginfo] target(s) in 11.9 secs
time: 14.926014173
5000000
```

# resources

```text
   48185 keywords.txt
 5000000 wordlist.txt
```

Each line in *wordlist.txt* is a single word.
Each line in *keywords.txt* are two tab-delimited words, first the key, second
the replacement.
The replacement string is of random length and therefore not suitable for
in-place replacement.
Output with replacement done is written to stdout, and a time written to
stderr.

## wordlist

You can generate new wordlists using:

```bash
# this will generate 5000000 for the wordlist and a hundredth of that for the
# keywords (note: keyword is dedupped)
./wordgen.py > wordlist.txt 2> keywords.txt
```

