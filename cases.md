Places features show up:
- command-line
- `[features]`
- `[dependencies]`

Packages:

```json
{
  "name": "dep_feature_with_explicit_feature",
  "features": {
    "a": [
      "dep_name/feature_name"
    ],
    "dep_name": [
      "dep:unrelated"
    ]
  }
}
{
  "name": "dep_feature_with_implicit_feature",
  "features": {
    "a": [
      "dep_name/feature_name"
    ],
    "dep_name": [
      "dep:dep_name"
    ]
  }
}
{
  "name": "dep_name",
  "features": {
    "feature_name": []
  }
}
{
  "name": "namespaced_dep",
  "features": {
    "a": [
      "dep:dep_name",
      "dep_name/feature_name"
    ]
  }
}
{
  "name": "unrelated",
  "features": {
    "feature_name": []
  }
}
```

```console
$ cases/dep_feature_with_explicit_feature
info: running `cargo run --no-default-features` on dep_feature_with_explicit_feature (1/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_explicit_feature`
dep_feature_with_explicit_feature

info: running `cargo run --no-default-features --features a,dep_name` on dep_feature_with_explicit_feature (2/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_explicit_feature`
dep_feature_with_explicit_feature
dep_feature_with_explicit_feature a
dep_feature_with_explicit_feature->dep_name
dep_feature_with_explicit_feature->dep_name feature_name
dep_feature_with_explicit_feature dep_name
dep_feature_with_explicit_feature->unrelated

info: running `cargo run --no-default-features --features a` on dep_feature_with_explicit_feature (3/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_explicit_feature`
dep_feature_with_explicit_feature
dep_feature_with_explicit_feature a
dep_feature_with_explicit_feature->dep_name
dep_feature_with_explicit_feature->dep_name feature_name
dep_feature_with_explicit_feature dep_name
dep_feature_with_explicit_feature->unrelated

info: running `cargo run --no-default-features --features dep_name` on dep_feature_with_explicit_feature (4/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_explicit_feature`
dep_feature_with_explicit_feature
dep_feature_with_explicit_feature dep_name
dep_feature_with_explicit_feature->unrelated
```
```console
$ cases/dep_feature_with_implicit_feature
info: running `cargo run --no-default-features` on dep_feature_with_implicit_feature (1/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_implicit_feature`
dep_feature_with_implicit_feature

info: running `cargo run --no-default-features --features a,dep_name` on dep_feature_with_implicit_feature (2/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_implicit_feature`
dep_feature_with_implicit_feature
dep_feature_with_implicit_feature a
dep_feature_with_implicit_feature->dep_name
dep_feature_with_implicit_feature->dep_name feature_name
dep_feature_with_implicit_feature dep_name
dep_feature_with_implicit_feature->dep_name
dep_feature_with_implicit_feature->dep_name feature_name

info: running `cargo run --no-default-features --features a` on dep_feature_with_implicit_feature (3/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_implicit_feature`
dep_feature_with_implicit_feature
dep_feature_with_implicit_feature a
dep_feature_with_implicit_feature->dep_name
dep_feature_with_implicit_feature->dep_name feature_name
dep_feature_with_implicit_feature dep_name
dep_feature_with_implicit_feature->dep_name
dep_feature_with_implicit_feature->dep_name feature_name

info: running `cargo run --no-default-features --features dep_name` on dep_feature_with_implicit_feature (4/4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/dep_feature_with_implicit_feature`
dep_feature_with_implicit_feature
dep_feature_with_implicit_feature dep_name
dep_feature_with_implicit_feature->dep_name
```
```console
$ cases/namespaced_dep
info: running `cargo run --no-default-features` on namespaced_dep (1/2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/namespaced_dep`
namespaced_dep

info: running `cargo run --no-default-features --features a` on namespaced_dep (2/2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/epage/src/personal/cargo-feature-demo/target/debug/namespaced_dep`
namespaced_dep
namespaced_dep a
namespaced_dep->dep_name
namespaced_dep->dep_name feature_name
```
