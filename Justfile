new-version:
	#!/bin/bash
	VERSION=$(git cliff --bumped-version | cut -d'v' -f2)
	git cliff -o CHANGELOG.md --tag v$VERSION
	git commit --allow-empty -m "chore: release v$VERSION"
	git tag -f v$VERSION
