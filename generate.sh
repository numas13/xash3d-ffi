#!/bin/sh

set -e

script=$(readlink -f "$0")
root=$(dirname "$script")

cd "$root"
exec bindgen \
    --rust-target 1.64 \
    --use-core \
    --generate-cstr \
    --ignore-functions \
    --no-doc-comments \
    --no-layout-tests \
    --use-array-pointers-in-arguments \
    --allowlist-file "xash3d-fwgs/.*" \
    --blocklist-file "/usr/.*" \
    --blocklist-file "xash3d-fwgs/public/build.h" \
    --blocklist-item "NUM_AMBIENTS" \
    --blocklist-type "mnode_s" \
    --blocklist-type "mnode_s__.*" \
    --blocklist-var "boxpnt" \
    --blocklist-var "gEntityInterface" \
    --blocklist-var "gNewDLLFunctions" \
    --blocklist-var "m_bytenormals" \
    --blocklist-var "svc_.*strings" \
    --opaque-type va_list \
    --new-type-alias "vec[234]_t" \
    "wrapper.h" \
    -- \
    -Ixash3d-fwgs \
    -Ixash3d-fwgs/common \
    -Ixash3d-fwgs/public \
    -Ixash3d-fwgs/pm_shared \
    -Ixash3d-fwgs/filesystem \
    -Ixash3d-fwgs/engine \
    "$@" > "$root/src/ffi.rs"
