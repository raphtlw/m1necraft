#!/usr/bin/env bash

[ -z "$1" ] && {
  echo "ERROR: tag name not set."
  echo "=> Example: $(basename $0) resources"
  exit 1
}

git tag -f $1
git push origin :refs/tags/$1
git push origin $1
