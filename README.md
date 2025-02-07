# 轻调度
## 使用方法
直接使用`magisk`安装模块
## 配置文件
配置文件在`/data/adb/modules/LightScheduling/config.toml`
"""toml
[default.cpu.big]
min_freq = 1000  # 示例值，大核最小频率
max_freq = 2000  # 示例值，大核最大频率

[default.cpu.middle]
min_freq = 800   # 示例值，中核最小频率
max_freq = 1800  # 示例值，中核最大频率

[default.cpu.small]
min_freq = 500   # 示例值，小核最小频率
max_freq = 1500  # 示例值，小核最大频率

[app]
"mt.bin.plus" = "/path/to/file"
"""
App的配置模板
"""
[cpu.big]
min_freq = 2000000   # 2.0 GHz
max_freq = 3000000   # 3.0 GHz

[cpu.middle]
min_freq = 1800000
max_freq = 2500000

[cpu.small]
min_freq = 1800000
max_freq = 2500000 # 频率表中的数值，单位Khz
"""

## TODE
1. 适配GPU调整
2. 对某些游戏，启动专属调度
