#!/usr/bin/env bash

[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ] || {
  git checkout main
}

./scripts/generate_appcast

git checkout gh-pages
git add appcast.xml
git push
