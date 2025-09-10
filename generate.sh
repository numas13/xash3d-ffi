#!/bin/sh

set -e

script=$(readlink -f "$0")
root=$(dirname "$script")
cd "$root"

target="${1:-i686-unknown-linux-gnu}"

MSRV=1.64

CFLAGS=""
CFLAGS+=" -target $target"
CFLAGS+=" -Ixash3d-fwgs"
CFLAGS+=" -Ixash3d-fwgs/common"
CFLAGS+=" -Ixash3d-fwgs/public"
CFLAGS+=" -Ixash3d-fwgs/pm_shared"
CFLAGS+=" -Ixash3d-fwgs/filesystem"
CFLAGS+=" -Ixash3d-fwgs/engine"

function generate() {
    wrapper_h="$1"
    output="$2"
    shift 2
    bindgen \
        "$wrapper_h" \
        --rust-target $MSRV \
        --use-core \
        --generate-cstr \
        --ignore-functions \
        --no-doc-comments \
        --no-layout-tests \
        --use-array-pointers-in-arguments \
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
        "$@" -- $CFLAGS > "$output"
}

##############################################################################
# common definitions
##############################################################################

generate "wrapper-common.h" "src/generated/common.rs" \
    --allowlist-file "xash3d-fwgs/.*" \
    --blocklist-type "mstudio.*"

##############################################################################
# shared apis
##############################################################################

generate "wrapper-net-api.h" "src/generated/net_api.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/net_api.h"

generate "wrapper-studio-api.h" "src/generated/studio_api.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/r_studioint.h" \
    --allowlist-file "xash3d-fwgs/common/studio_event.h" \
    --allowlist-file "xash3d-fwgs/engine/studio.h"

generate "wrapper-tri-api.h" "src/generated/tri_api.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/triangleapi.h"

generate "wrapper-render-api.h" "src/generated/render_api.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/lightstyle.h" \
    --allowlist-file "xash3d-fwgs/common/render_api.h"

generate "wrapper-fs-api.h" "src/generated/fs_api.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/filesystem/filesystem.h"

generate "wrapper-event-api.h" "src/generated/event_api.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/event_api.h"

generate "wrapper-player-move.h" "src/generated/player_move.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/pm_shared/pm_movevars.h" \
    --allowlist-file "xash3d-fwgs/pm_shared/pm_defs.h"

##############################################################################
# dlls
##############################################################################

generate "wrapper-server.h" "src/generated/server.rs" \
    --no-recursive-allowlist \
    --allowlist-type "edict_t" \
    --allowlist-type "delta_s" \
    --allowlist-file "xash3d-fwgs/engine/progdefs.h" \
    --allowlist-file "xash3d-fwgs/engine/edict.h" \
    --allowlist-file "xash3d-fwgs/engine/eiface.h"

generate "wrapper-client.h" "src/generated/client.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/ivoicetweak.h" \
    --allowlist-file "xash3d-fwgs/common/demo_api.h" \
    --allowlist-file "xash3d-fwgs/common/r_efx.h" \
    --allowlist-file "xash3d-fwgs/engine/cdll_int.h" \
    --allowlist-file "xash3d-fwgs/engine/cdll_exp.h"

generate "wrapper-menu.h" "src/generated/menu.rs" \
    --no-recursive-allowlist \
    --allowlist-file "xash3d-fwgs/common/gameinfo.h" \
    --allowlist-file "xash3d-fwgs/engine/menu_int.h"

generate "wrapper-render.h" "src/generated/render.rs" \
    --no-recursive-allowlist \
    --allowlist-type "convar_[st]" \
    --allowlist-file "xash3d-fwgs/common/com_image.h" \
    --allowlist-file "xash3d-fwgs/engine/ref_api.h"
