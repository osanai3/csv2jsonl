# csv2jsonl

## install
```bash
cargo install --git https://github.com/osanai3/csv2jsonl --tag v2.0.0
```

## usage
```bash
csv2jsonl < input.csv
```

input
```
id,name
1,hoge
2,"fuga"
3,"hoge""fuga"
4,"hoge
fuga"
5,
```

output
```json
{"id":"1","name":"hoge"}
{"id":"2","name":"fuga"}
{"id":"3","name":"hoge\"fuga"}
{"id":"4","name":"hoge\nfuga"}
{"id":"5","name":""}
```
