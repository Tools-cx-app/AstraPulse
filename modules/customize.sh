# 注意 这不是占位符！！这个代码的作用是将模块里的东西全部塞系统里，然后挂上默认权限

ui_print "专为萌新做的调度"
ui_print "配置文件在/data/adb/modules/AstraPulse/config.toml"
ui_print "/data/adb/modules/AstraPulse/config/目录下的文件是自定义频率等文件，文件必须以toml结尾"
set_perm_recursive $MODPATH 0 0 0755 0644
set_perm $MODPATH/AstraPulse 0 0 0755