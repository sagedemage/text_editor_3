#!/bin/sh
# lint the project using clippy
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::cargo -W clippy::unwrap_used -A clippy::multiple_crate_versions -A clippy::too-many-lines -A clippy::module-name-repetitions -D warnings
