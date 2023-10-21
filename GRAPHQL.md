# Graphql queries and mutations

## 시나리오대로

아래 쿼리와 뮤테이션들은 제공되는 모델 기반에서 운용하려는 기본 시나리오에 맞는 것들을 모아놨다. 이 외에도 graphql schema 를 보면 관련된 더 만은 쿼리와 뮤테이션들이 제공되고 있고 더 업데이트 되어야 한다.

아래는 직원, 역할, 권한, 팀, 고객 생성하는 시나리오다. 생성에 맞게 제거(delete)와 변경(update) 가 존재한다.

* 직원(employee) 생성하고
* 역할(role) 을 생성하고
* 권한(permission)을 생성하고
* 팀(team) 을 생성하고
* 고객(client) 을 생성하고

아래는 만들어진 모델들을 엮어주는 시나리오다. 부여(assign)하는 것과 동시에 제외(remove) 도 존재한다. 

* 직원에게 역할을 부여하고(1..1)
* 역할에 권한들을 부여하고(1..*)
* 직원에게 팀을 부여하고(1..1)
* 고객에게 담당팀을 부여하고(1..1)

### employee 생성

```graphql
mutation {
  createEmployee(name: "박남정") {
    id
  }
}
```

### employee 제거

```graphql
mutation {
  deleteEmployee(id: 3)
}
```

### role 생성

```graphql
mutation {
  createRole(name: "가수", description: "노래하는 사람") {
    id
  }
}
```

### permission 생성

```graphql
mutation {
  createPermission(name: "노래하기", description: "노래를 할 수 있는 권한") {
    id
  }
}

mutation {
  createPermission(name: "춤추기", description: "춤출 수 있는 권한") {
    id
  }
}
```

### role 에 permission 주기

```graphql
mutation {
  addPermissionsToRole(roleId: 2, permissionIds: [4, 5])
}
```

### employee 에게 role 부여하기

```graphql
mutation {
  assignRoleToEmployee(employeeId: 4, roleId: 2) {
    id
  }
}
```

### employee 에게 role 제외시키기

```graphql
mutation {
  removeRoleFromEmployee(employeeId: 3)
}
```

### team 생성

```graphql
mutation {
  createTeam(name: "연예인팀") {
    id
  }
}
```

### employee 에게 team 할당하기

```graphql
mutation {
  assignEmployeeToTeam(employeeId: 4, teamId: 2) {
    id
  }
}
```

### employee 를 team 에서 제외하기

```graphql
mutation {
  removeEmployeeFromTeam(employeeId: 3)
}
```

### client 생성하기

```graphql
mutation {
  createClient(name: "별나라레코드") {
    id
  }
}
```

### client 를 team 에 할당하기

```graphql
mutation {
  assignClientToTeam(clientId: 3, teamId: 2)
}
```

### client 를 team 에서 배제하기

```graphql
mutation {
  removeClientFromTeam(clientId: 3)
}
```

### 모든 employee 에 대한 자세한 정보 보기

```graphql
{
  employees {
    id
    name
    teamId
    team {
      id
      name
      clients {
        id
        name
        teamId
        team {
          id
        }
      }
    }
    roleId
    role {
      id
      name
      description
      permissions {
        id
        name
      }
    }
  }
}
```

### employee 한 명에 대한 자세한 정보 보기

```graphql
{
  employeeById(id: 1) {
    id
    name
    teamId
    team {
      id
      name
      clients {
        id
        name
        teamId
        team {
          id
        }
      }
    }
    roleId
    role {
      id
      name
      description
      permissions {
        id
        name
      }
    }
  }
}
```