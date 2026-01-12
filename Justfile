release: new-version push

new-version:
	#!/bin/bash
	VERSION=$(git cliff --bumped-version | cut -d'v' -f2)
	sed -i -e "s/^version.*/version = \"$VERSION\"/" Cargo.toml
	git cliff -o CHANGELOG.md --tag v$VERSION
	git commit --allow-empty -am "chore: release v$VERSION"
	git tag -f v$VERSION

push:
	git push
	git push --tags
