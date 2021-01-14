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
❯❯❯ vim Cargo.toml                      # Update the minor or major version number
❯❯❯ git add Cargo.toml
❯❯❯ git commit -a -m "Release v0.X.0"   # Change with the new version
❯❯❯ cargo build                         # Rebuild to allow the tag version to be automatically guessed
❯❯❯ make tag                            # When prompted in vim, add a commit message like
                                        # "Update changelog for v0.X.0"
❯❯❯ git push                            # Push the repo
❯❯❯ git push --tagd                     # Push the tags
```

This will kick a special CI mode on [Travis](https://travis-ci.org/github/blackmesalab/fireguard) 
which will build all cross-compile artifacts and release them in the [Github releases page](https://github.com/blackmesalab/fireguard/releases).
