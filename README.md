# jup-cli

A Jupiter CLI that enables swap, etc.

## Install

```console
$ cargo install jup-cli
```

## Usage

```console
$ jup-cli swap --amount={amount} --mint-from={from} --mint-to={to} --key-pair-path={path-to-key-pair}
```

```console
$ jup-cli price --mints={mint1,mint2,...}
```

## Example

```console
$ jup-cli swap --dry-run --amount=1 --mint-from=7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU --mint-to=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --key-pair-path=key_pair.json
[2024-10-17T10:22:52Z INFO  jup_cli] expected amount: 0.007395
```

```console
$ jup-cli swap --amount=1 --mint-from=7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU --mint-to=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --key-pair-path=key_pair.json
[2024-10-17T10:42:40Z INFO  jup_cli] expected amount: 0.007379
[2024-10-17T10:42:12Z INFO  jup_cli] Transaction: xxxxxxxx
```

```console
$ jup-cli price --mints=So11111111111111111111111111111111111111112,JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN
{"So11111111111111111111111111111111111111112":166.566327111,"JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN":0.980275}
```

## Tips

### Create a Keypair

```console
$ solana-keygen new -o key_pair.json --no-bip39-passphrase
```
