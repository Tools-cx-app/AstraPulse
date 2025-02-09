# 轻调度
## 使用方法
直接使用`magisk`安装模块
## 配置文件
- 配置文件在`/data/adb/modules/LightScheduling/config.toml`
### `min_freq` 最小频率，单位Khz
### `max_freq` 最大频率，单位Khz
### `model` 调速器。
```toml
[default.cpu.big]
min_freq = 1300000
max_freq = 3050000
model = "sugov_ext"

[default.cpu.middle]
min_freq = 400000
max_freq = 2850000
model = "sugov_ext"

[default.cpu.small]
min_freq = 200000
max_freq = 1800000
model = "sugov_ext"

[app]
"mt.bin.plus" = "/path/to/file"
```
App的配置模板
```toml
[cpu.big]
min_freq = 1300000
max_freq = 3050000
model = "sugov_ext"

[cpu.middle]
min_freq = 400000
max_freq = 2850000
model = "sugov_ext"

[cpu.small]
min_freq = 200000
max_freq = 1800000
model = "sugov_ext"
```

## 编译方式
以ubuntu为例
```
# NDK是必须的
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup target add aarch64-linux-android
git clone https://github.com/Tools-cx-app/LightScheduling
cd LightScheduling
cargo install cargo-ndk
cargo ndk -t arm64-v8a build --release
cp target/aarch64-linux-android/release/LightScheduling ./modules/
zip -9 -rq LightScheduling.zip modules/* # 生成LightScheduling.zip，直接刷入即可
```
## TODE
1. 适配GPU调整
2. 对某些游戏，启动专属调度
