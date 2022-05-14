#!/usr/bin/env bash

# Check for uncommitted changes, because git checkout is needed
[[ "$(git diff-index HEAD)" ]] && {
  echo "There are uncommitted changes. Please commit or stash them before continuing!"
  exit 1
}

# Check if current branch is main
[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ] || {
  git checkout main
}

./scripts/generate_appcast || exit 1

WORKDIR="$(mktemp -d)"

mv appcast.xml "$WORKDIR"
git checkout gh-pages || exit 1
mv "$WORKDIR/appcast.xml" .
git add appcast.xml
git commit -m "Update appcast.xml"
git push
git checkout main
