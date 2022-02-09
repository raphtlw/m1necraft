#!/usr/bin/env bash

git tag -f resources
git push origin :refs/tags/resources
git push origin resources
