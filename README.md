# **ASP**

## 简介

- 一个简单到不能再简单的调度，可自定义性高

## 主文件选项

- ### `app`

  - **`"package"` = `Mode`**

    - `package`: 字符串，应用包名
      - `Mode`: [Mode类型](#Mode类型)
- ### `runtime`

  - `cpu` = `policy`

    - `cpu`: 不可变
      - `policy`: usize类型，必须为0-7
## config/里文件选项

- ### `pkg`
  字符串，应用包名
- ### `cpuset`

- `top_app` = `String`
- `String`: 为用户顶层应用的cpu使用核心(必须为`0-7`)

- `foreground` = `String`
- `String`: 为可用的cpu使用核心(必须为`0-7`)

- `system_background` = `String`
- `String`: 为系统后台程序的cpu使用核心(必须为`0-7`)

- `background` = `String`
- `String`: 为用户后台应用的cpu使用核心(必须为`0-7`)

## Mode类型

- ### `Powersave`
  省电模式，限制cpu频率，和后台应用
- ### `Balnace`
  均衡模式，日常使用
- ### `Performance`
  游戏模式，性能全开
- ### `Fast`
  比[Performance](#Performance)还激进

## 联系方式

QQ群：687235389
