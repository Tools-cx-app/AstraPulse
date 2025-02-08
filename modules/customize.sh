# 注意 这不是占位符！！这个代码的作用是将模块里的东西全部塞系统里，然后挂上默认权限
echo "欢迎使用轻调度"
echo "本调度只会修改Cpu/Gpu频率"
echo "设置权限中"
echo "配置文件在/data/adb/modules/LightScheduling/config.toml"

set_perm_recursive $MODPATH 0 0 0755 0644
set_perm $MODPATH/LightScheduling 0 0 0755
