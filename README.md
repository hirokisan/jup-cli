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
$ [2024-10-17T11:38:52Z INFO  jup_cli] expected amount: 0.007395
```
