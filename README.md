# jup-cli

A Jupiter CLI that enables swap, etc.

## Install

```console
cargo install jup-cli
```

## Usage

```console
$ jup-cli swap --amount={amount} --mint-from={from} --mint-to={to} --key-pair-path={path-to-key-pair}
```

## Example

```console
$ jup-cli swap --dry-run --amount=1 --mint-from=7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU --mint-to=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --key-pair-path=key_pair.json
$ [2024-10-17T10:22:52Z INFO  jup_cli] expected amount: 0.007395
```

```console
$ jup-cli swap --amount=1 --mint-from=7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU --mint-to=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --key-pair-path=key_pair.json
❯ just swap
[2024-10-17T10:42:40Z INFO  jup_cli] expected amount: 0.007379
[2024-10-17T10:42:12Z INFO  jup_cli] Transaction: xxxxxxxx
```
