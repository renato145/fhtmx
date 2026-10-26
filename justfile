_default:
    @just --list

# Verify the vendored htmx skill matches its pinned upstream release
skills-check:
    ./scripts/sync_htmx_skill.sh --check

# Update the vendored htmx skill to the latest htmx release
skills-update:
    ./scripts/sync_htmx_skill.sh --update
