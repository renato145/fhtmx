#!/usr/bin/env bash
# Keeps the vendored htmx-guidance skill pinned to an upstream htmx release tag.
set -euo pipefail

REPO="bigskysoftware/htmx"
UPSTREAM_PATH="dist/skills/htmx-guidance.md"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILL_FILE="$ROOT/.pi/skills/htmx-guidance/SKILL.md"
PROVENANCE_FILE="$ROOT/.pi/skills/htmx-guidance/.upstream"

mode=check
ref_override=""
tmp=""

usage() {
  cat <<'EOF'
Usage: sync_htmx_skill.sh [--check | --update] [--ref <tag>]

  --check       Verify SKILL.md matches the pinned upstream release (default).
  --update      Re-download the skill from the latest htmx release tag.
  --ref <tag>   Use a specific htmx tag instead of the latest release.
  -h, --help    Show this help.

Set GITHUB_TOKEN to avoid GitHub API rate limits.
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

cleanup() {
  if [ -n "$tmp" ]; then
    rm -f "$tmp"
  fi
  return 0
}

file_sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d' ' -f1
  else
    shasum -a 256 "$1" | cut -d' ' -f1
  fi
}

github_get() {
  if [ -n "${GITHUB_TOKEN:-}" ]; then
    curl -fsSL -H "Authorization: Bearer $GITHUB_TOKEN" "$1"
  else
    curl -fsSL "$1"
  fi
}

resolve_latest_tag() {
  local tag
  tag=$(github_get "https://api.github.com/repos/$REPO/releases/latest" \
    | awk -F'"' '/"tag_name"/ && !tag { tag = $4 } END { print tag }')
  if [ -z "$tag" ]; then
    tag=$(github_get "https://api.github.com/repos/$REPO/tags?per_page=1" \
      | awk -F'"' '/"name"/ && !tag { tag = $4 } END { print tag }')
  fi
  [ -n "$tag" ] || die "could not resolve the latest htmx release tag"
  printf '%s\n' "$tag"
}

download_skill() {
  local ref="$1" dest="$2"
  local url="https://raw.githubusercontent.com/$REPO/$ref/$UPSTREAM_PATH"
  curl -fsSL -o "$dest" "$url" \
    || die "failed to download $url (check the ref, network, or GITHUB_TOKEN)"
  grep -qx -- '---' "$dest" \
    || die "downloaded file is not a skill: $url"
  grep -q '^name: htmx-guidance$' "$dest" \
    || die "downloaded skill does not declare name: htmx-guidance"
}

check() {
  [ -f "$SKILL_FILE" ] || die "missing $SKILL_FILE"
  [ -f "$PROVENANCE_FILE" ] \
    || die "missing $PROVENANCE_FILE; run: just skills-update"

  # shellcheck source=/dev/null
  . "$PROVENANCE_FILE"
  : "${upstream_ref:?}" "${upstream_sha256:?}"

  tmp=$(mktemp)
  download_skill "$upstream_ref" "$tmp"

  local upstream_sha local_sha
  upstream_sha=$(file_sha256 "$tmp")
  local_sha=$(file_sha256 "$SKILL_FILE")

  if [ "$upstream_sha" != "$upstream_sha256" ]; then
    die "upstream $upstream_ref now hashes to $upstream_sha but .upstream pins $upstream_sha256; run: just skills-update"
  fi
  if [ "$local_sha" != "$upstream_sha" ]; then
    echo "error: $SKILL_FILE differs from $REPO@$upstream_ref" >&2
    diff -u "$tmp" "$SKILL_FILE" | head -n 40 || true
    exit 1
  fi

  echo "ok: htmx-guidance matches $REPO@$upstream_ref"
}

update() {
  local old_ref="" ref
  [ -f "$PROVENANCE_FILE" ] \
    && old_ref=$(sed -n 's/^upstream_ref=//p' "$PROVENANCE_FILE")

  ref="${ref_override:-$(resolve_latest_tag)}"
  tmp=$(mktemp)
  download_skill "$ref" "$tmp"

  local sha
  sha=$(file_sha256 "$tmp")

  if [ "$old_ref" = "$ref" ] && cmp -s "$tmp" "$SKILL_FILE" \
    && [ "$sha" = "$(sed -n 's/^upstream_sha256=//p' "$PROVENANCE_FILE")" ]; then
    echo "ok: htmx-guidance is already at $REPO@$ref"
    return
  fi

  cp "$tmp" "$SKILL_FILE"
  cat > "$PROVENANCE_FILE" <<EOF
upstream_repo=$REPO
upstream_ref=$ref
upstream_path=$UPSTREAM_PATH
upstream_sha256=$sha
EOF

  if [ -n "$old_ref" ] && [ "$old_ref" != "$ref" ]; then
    echo "updated: htmx-guidance $old_ref -> $ref"
  else
    echo "updated: htmx-guidance at $REPO@$ref"
  fi
}

trap cleanup EXIT

while [ $# -gt 0 ]; do
  case "$1" in
    --check) mode=check ;;
    --update) mode=update ;;
    --ref)
      [ $# -ge 2 ] || die "--ref requires a value"
      ref_override="$2"
      shift
      ;;
    -h|--help) usage; exit 0 ;;
    *) usage >&2; exit 2 ;;
  esac
  shift
done

if [ -n "$ref_override" ]; then
  [[ "$ref_override" =~ ^[A-Za-z0-9._-]+$ ]] \
    || die "invalid ref '$ref_override' (expected a tag such as v4.0.0)"
fi

case "$mode" in
  check) check ;;
  update) update ;;
esac
