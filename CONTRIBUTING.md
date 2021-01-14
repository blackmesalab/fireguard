# 🖋 Contributing 🖋

Contributions are welcome and we will happily review them.

## How do I contribute?
* Fork `blackmesalab/fireguard`.
* Shift the bits.
* Push the code on your fork.
* Create a PR.
* EXPLAIN WHY THIS CHANGE IS IMPORTANT and HOW IT IS DONE in the PR description.
* Publish the PR.
* Check that the CI is not complaining about your change.

## How do I release a new version?
To release a new version, update the crate version in `Cargo.toml`, create a new git tag and push the tags:
```sh
❯❯❯ VERSION=0.1.0
❯❯❯ vim Cargo.toml
❯❯❯ CHANGELOG=$(git log $(git describe --tags --abbrev=0)..HEAD --oneline)
❯❯❯ # add ${CHANGELOG} to CHANGELOG.md
❯❯❯ git commit -a -m "Release version $VERSION"
❯❯❯ git tag v$VERSION
❯❯❯ git push --tags
```

This will kick a special CI mode on [Travis](https://travis-ci.org/github/blackmesalab/fireguard) 
which will build all cross-compile artifacts and release them in the [Github releases page](https://github.com/blackmesalab/fireguard/releases).
