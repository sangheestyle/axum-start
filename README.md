# axum-start

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

### Migration

```
sqlx database drop
sqlx database create
sqlx migrate add <name>
sqlx migrate run
```

- reference: https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md

### Offline Mode

```
cargo sqlx prepare
```

- DB 접속된 상태에서(.env 등) 위 명령어 실행하면 .sqlx 폴더 아래 코드상의 모든 쿼리를 validate 하고 그 결과를 남겨둠
- DATABASE_URL 가 없어도 문제 없이 빌드됨 - offline
- 쿼리 추가되면 다시 위 명령어를 DATABASE_URL 에 있는 디비 연결해서 업데이트하고 commit push 하고 진행하면 됨
- 참고: https://docs.rs/sqlx/latest/sqlx/macro.query.html#offline-mode-requires-the-offline-feature

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

## Stack

- axum: tokio 에서 만든 웹프레임워크다. 러스트쪽은 아직 독보적인 웹프레임웍이 나오지 않았다. 여전히 진행중인데 axum 을 선택한 이유는 그간 아쉬운 점들을 반영해서 나온 프로젝트이기도 하고(반대로 말하면 여전히 young 하다) 현실적으로 많이 사용하는 tokio 쪽에서 만든 얇은 레이어의 웹프레임웍이다. 또한 가장 많은 미들웨어를 가진 tower, tower-http 와 미들웨어 호환이 된다.
- sqlx: 그 유명한 diesel 을 사용하고 싶었는데 diesel 은 async 지원을 하지 않는다. async 로 모두 사용해서 퍼포먼스를 낼려면 그에 맞게 async 지원하는 sqlx 를 사용하기도 했다. sqlx 위에서 만든 SeaORM 을 사용해보려고 했는데 migration 등을 비교해보니 sqlx 도 migration 잘 지원하고 async-graphql 과 엮어서 생각해보니 sqlx 가 나한테는 더 편하고 migration 을 그냥 sql 문으로 쓰면 되서 sqlx 로. sqlx 의 FromRow 매크로와 async-graphql 의 SimpleObject + ComplextObject 로 바르면 딱히 SeaORM 등이 필요 없다고 느꼈다.
- async-graphql: graphql을 사용하려고 하는데 juniper 를 쓰고 싶었지만 async 지원이 되지 않았다. 더불어 async-graphql 을 사용하는 프로젝트들도 좀 있고, SeaORM 위에 async-graphql 을 사용하는 프로젝트가 SeaORM 진영에서 열심히 개발중이여서 써도 되겠다... 문서화도 잘 되어 있고.
- tokio: 러스트는 스탠다드 런타임이 golang 이나 nodejs 처럼 존재하지 않아서 런타임도 골라야 하는데 런타임은 tokio 아니면 async-std 인데 tokio 안에서 모두 해결하고 싶어서 tokio 로. 많이 쓰기도 하고.
- postgresql: 늘 쓰던거라서...

이정도.

몇 가지 남은점은

- subscription: graphql subscription 을 쓰러면 그에 맞게 pubsub 이 필요한데 고민중인다. sns 를 써도 되고 redis 를 써도 되는데 그에 맞게 구현하면 될 것 같다.
- auth: auth 의 폭이 아주 넓지 않은데 그래도 존재하는거 같은데 아직 돌아보지 못했다. 일단 db 에 auth 구현해서 async-graphql 의 guard 를 써보려고 한다. Field guard 인데 apollo 등에서도 많이들 쓰이고 해서 통합해서 쓸 때 편하다.

https://async-graphql.github.io/async-graphql/en/field_guard.html
