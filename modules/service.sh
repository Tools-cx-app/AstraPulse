#!/system/bin/sh
# 请不要硬编码 /magisk/modname/... ; 请使用 $MODDIR/...
# 这将使你的脚本更加兼容，即使Magisk在未来改变了它的挂载点
MODDIR=${0%/*}

wait_until_login() {

    # in case of /data encryption is disabled
    while [ "$(getprop sys.boot_completed)" != "1" ]; do
        sleep 5
    done

    # we doesn't have the permission to rw "/sdcard" before the user unlocks the screen
    # shellcheck disable=SC2039
    local test_file="/sdcard/Android/.PERMISSION_TEST_FREEZEIT"
    true >"$test_file"
    while [ ! -f "$test_file" ]; do
        sleep 5
        true >"$test_file"
    done
    rm "$test_file"
}

wait_until_login

$MODDIR/LightScheduling > $MODDIR/run.log
