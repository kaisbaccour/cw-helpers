# Maintenance

## Create a release

1. `export CONTRACTS_VERSION=0.1.0`
2. Run `./devtools/set_version.sh "$NEW_CONTRACTS_VERSION"`
3. Set release version and date in CHANGELOG.md and amend the commit from 1.
4. Run `git tag "v$NEW_CONTRACTS_VERSION"`
5. `git push && git push --tags`
