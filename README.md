ATLrus
======

Atlassian x Rust CLI


🦉 For users
------------

### Enviromental variables

Set all environmental variables.

|        Name         |           Description            |    Example     |
| ------------------- | -------------------------------- | -------------- |
| ATLRUS_USER_NAME    | Account name for Bitbucket cloud | tadashi-aikawa |
| ATLRUS_APP_PASSWORD | App password for Bitbucket cloud | di3948vmshg2i  |

App password needs below permissions.

|      Permission      |    Level     |
| -------------------- | ------------ |
| Account              | Write        |
| Workspace membership | Read         |
| Project              | Write        |
| Repository           | Administrate |


### Input file

Create `input.json` as following.

```json
{
  "create_groups": {
    "workspace_uuid": "{29878156-d095-4fd8-94d4-xxxxxxxxxxxx}",
    "group_names": [
      "Group name"
    ]
  },
  "invite_members": {
    "repository": "orgnization/10take",
    "permission": "read",
    "emails": [
      "user1@xxx.com",
      "user2@xxx.com"
    ]
  },
  "add_group_members": {
    "workspace_uuid": "{29878156-d095-4fd8-94d4-xxxxxxxxxxxx}",
    "groups": [
      {
        "slug": "group_slug",
        "emails": [
          "user1@xxx.com",
          "user3@xxx.com"
        ]
      }
    ]
  }
}
```

### Run

```
$ atlrus input.json
```



🦉 For developers
-----------------

### Build

`cargo build --release`

### Release

1. Update a version in `Cargo.toml`
2. Commit and Tags `v_._._`
3. Push
