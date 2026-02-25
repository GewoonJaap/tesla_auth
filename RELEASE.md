# Release

1. Update `CHANGELOG.md` with the new version and date
2. Bump the version in `Cargo.toml`
3. Run `cargo check` to update `Cargo.lock`
4. Commit: `git commit -am "Release v0.X.0"`
5. Tag: `git tag v0.X.0`
6. Push: `git push && git push --tags`

Pushing the tag triggers the [release workflow](.github/workflows/release.yml),
which uses [cargo-dist](https://opensource.axo.dev/cargo-dist/) to build
binaries for all targets and create a GitHub Release.
