# **ASP**

## 简介

> 一个简单到不能再简单的调度，可自定义性高

## 主文件选项

- ### `app`

  - **`"package"` = `Mode`**

    - `package`: 字符串，应用包名
      - `Mode`: [Mode 类型](#Mode类型)

- ### `runtime`

  - `cpu` = `policy`

    - `cpu`: 不可变
      - `policy`: usize 类型，必须为`0-7`

## config/里文件选项

- ### `pkg`
  字符串，应用包名
- ### `cpuset`

- `top_app` = `String`
- `String`: 为用户顶层应用的 cpu 使用核心(必须为`0-7`)

- `foreground` = `String`
- `String`: 为可用的 cpu 使用核心(必须为`0-7`)

- `system_background` = `String`
- `String`: 为系统后台程序的 cpu 使用核心(必须为`0-7`)

- `background` = `String`
- `String`: 为用户后台应用的 cpu 使用核心(必须为`0-7`)

- ### `thread`

- `thread` = `String`
- `String`: 为需要绑定的线程

- `cpu` = `usize`
- `usize`: usize 类型，为线程需要绑定的 cpu

- ### `fps`

- `fps` = `i32`
- `i32`: 为游戏帧率(feas 使用)

# 配置文件示例

## config.toml

```toml
[app]
"bin.mt.plus" = "Powersave"

[runtime]
cpu = 6
```

## config/里文件

```toml
pkg = "bin.mt.plus"

[cpuset]
top_app = "0-3"
background = "4-5"
foreground = "0-5"
system_background = "4-5"

[[thread]]
thread = "io_worker"
cpu = 4

[[thread]]
thread = "net_worker"
cpu = 5

fps = 120
```

## Mode 类型

- ### `Powersave`
  省电模式，限制 cpu 频率，和后台应用
- ### `Balnace`
  均衡模式，日常使用
- ### `Performance`
  游戏模式，性能全开
- ### `Fast`
  比[Performance](#Performance)还激进

## 联系方式

QQ 群：687235389
