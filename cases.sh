#!/usr/bin/env bash

echo "Places features show up:"
echo "- command-line"
echo "- \`[features]\`"
echo "- \`[dependencies]\`"
echo
echo "Packages:"
echo
echo "\`\`\`json"
cargo metadata --format-version 1 | ~/Downloads/jq-linux-amd64 '.packages[] | pick(.name, .features)'
echo "\`\`\`"
echo

echo "\`\`\`console"
echo "$ cases/dep_feature_with_explicit_feature"
pushd cases/dep_feature_with_explicit_feature > /dev/null && cargo hack run --feature-powerset --include-deps-features --optional-deps 2>&1 && popd > /dev/null
echo "\`\`\`"

echo "\`\`\`console"
echo "$ cases/dep_feature_with_implicit_feature"
pushd cases/dep_feature_with_implicit_feature > /dev/null && cargo hack run --feature-powerset --include-deps-features --optional-deps 2>&1 && popd > /dev/null
echo "\`\`\`"

echo "\`\`\`console"
echo "$ cases/namespaced_dep"
pushd cases/namespaced_dep > /dev/null && cargo hack run --feature-powerset --include-deps-features --optional-deps 2>&1 && popd > /dev/null
echo "\`\`\`"
