# Axum-start

## Local

```
cargo run
```

## Docker

```
docker build -t axum-start .
docker run -dp 127.0.0.1:3000:3000 axum-start
```

## DB

Local

```
psql -h localhost -U sanghee -d axum
```

- .env 안됨
- axum 이라는 db 에 todo 테이블이 존재해야함

```sql
CREATE TABLE IF NOT EXISTS todo (id TEXT PRIMARY KEY NOT NULL, body TEXT NOT NULL, complete BOOLEAN NOT NULL);
```

## TODO

- .env 되도록 고치고
- todo 대신에 employee 에 대한 것으로 변경
- subscription 이 지금은 안됨 - 고쳐야함 필요할 때, broker 문제임
- DB migration 초기부터 어떻게 하는지 고민해야함
- graphql logger 등 추가
- auth 추가 https://async-graphql.github.io/async-graphql/en/utilities.html
- docker build 가 느림
- cdk 추가

## Reference

- [distroless](https://github.com/GoogleContainerTools/distroless/blob/main/examples/rust/Dockerfile)
