# Actix Basics

Simple `actix_web` server example emulating the behavior of `https://jsonplaceholder.typicode.com/api/v1/users`

## Running the Server

```bash
cargo run
```

This server then is bound and listening on `127.0.0.1:9090`.

## Querying

There is only one endpoint presently located at `/api/v1/users`.

- `GET` responds with a json with all users.
- `POST` with a json formatted user paylod (see `src/user.rs` for fields) populated with a minimum of a `name` and `email` field.
  > **NOTE:** A user is created only if the `name` is between **3 and 80** characters long and the `email` address **must be valid**.

### Examples

#### Get Users with `curl`

curl to get:

```bash
curl -v -d '{ "name": "Martin Fowler", "email": "martin@martinfowler.com" }' -H "Content-Type: application/json" -X POST 127.0.0.1:9090/api/v1/users
```

Response (verbose) JSON:

```bash
*   Trying 127.0.0.1:9090...
# ... snip ...
< HTTP/1.1 200 OK
< content-length: 5646
< content-type: application/json; charset=utf-8
< date: Wed, 02 Sep 2020 02:51:24 GMT
<
{ [5646 bytes data]
[
  {
    "id": 1,
    "name": "Leanne Graham",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "address": {
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "city": "Gwenborough",
      "zipcode": "92998-3874",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      }
    },
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets"
    }
  },
# ... snip ...
```

#### Create a User with `curl`

Request JSON:

```
{ "name": "Martin Fowler", "email": "martin@martinfowler.com"}
```

curl to POST this:

```bash
curl -v -d '{ "name": "Martin Fowler", "email": "martin@martinfowler.com" }' -H "Content-Type: application/json" -X POST 127.0.0.1:9090/api/v1/users
```

Response (Verbose):

```bash
*   Trying 127.0.0.1:9090...
# ... snip ...
< HTTP/1.1 201 Created
< content-length: 235
< content-type: application/json
< date: Wed, 02 Sep 2020 02:46:45 GMT
<
* Connection #0 to host 127.0.0.1 left intact
{"address":{"city":"","geo":{"lat":"","lng":""},"street":"","suite":"","zipcode":""},"company":{"bs":"","catchPhrase":"","name":""},"email":"martin@martinfowler.com","id":11,"name":"Martin Fowler","phone":"","username":"","website":""}
```

## Testing

We use [cargo's testing system](https://doc.rust-lang.org/cargo/commands/cargo-test.html). To run tests, we want **two terminals open** - one running the server, and one to run tests against it.

### 1. Run the Server:

In a fresh terminal:

```bash
cargo run
```

### 2. Run the Tests against the Server:

In a fresh terminal:

```bash
cargo test
```
