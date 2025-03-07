#!/bin/bash

# Cargo.tomlからバージョンを取得
VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

# バージョンが取得できたか確認
if [ -z "$VERSION" ]; then
  echo "Version not found in Cargo.toml"
  exit 1
fi

TAG_NAME="v$VERSION"


# リポジトリを最新にする
git fetch –all

# ローカルにタグが存在するか確認し、存在する場合は削除
if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
  echo "Deleting local tag $TAG_NAME"
  git tag -d "$TAG_NAME"
else
  echo "Local tag $TAG_NAME does not exist, skipping deletion"
fi

# リモートにタグが存在するか確認し、存在する場合は削除
if git ls-remote --tags origin | grep -q "refs/tags/$TAG_NAME"; then
  echo "Deleting remote tag $TAG_NAME"
  git push origin --delete "$TAG_NAME"
else
  echo "Remote tag $TAG_NAME does not exist, skipping deletion"
fi

# 新しいタグを作成してプッシュ
echo "Creating and pushing new tag $TAG_NAME"
git tag "$TAG_NAME"
git push origin "$TAG_NAME"