@echo off

bindgen ul.h -o src/ul_sys.rs ^
    --allowlist-function ul.* --allowlist-type ul.* ^
    --no-layout-tests --no-prepend-enum-name ^
    -- -I %ULTRALIGHT%/include
