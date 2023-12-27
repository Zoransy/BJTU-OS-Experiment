# 一. 实验步骤

## 1. 时钟中断与计时器

### 1.1 实现timer子模块获取mtime的值

![image-20231214220953601](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214220953601.png)

### 1.2 实现sbi子模块

![image-20231214221238552](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214221238552.png)

### 1.3 在timer子模块进行封装

![image-20231214221404473](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214221404473.png)

![image-20231214221545983](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214221545983.png)

### 1.4 修改config.js增加常量

![image-20231214221632908](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214221632908.png)

### 1.5 修改syscall子模块

![image-20231214221831804](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214221831804.png)

### 1.6 修改mod.rs增加获取时间的系统调用

![image-20231214221927013](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214221927013.png)

## 2. 修改应用程序

### 2.1 增加get_time系统调用

![image-20231214222059242](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214222059242.png)

### 2.2 增加get_time用户库封装

![image-20231214222136699](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214222136699.png)

### 2.3 实现新测试应用

`00power_3.rs`

![image-20231214222356939](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214222356939.png)

`01power_5.rs`

![image-20231214222900600](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214222900600.png)

`02power_7.rs`

![image-20231214223009963](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214223009963.png)

`03power_7.rs`

![image-20231214222507892](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214222507892.png)

## 3. 实现抢占式调度

修改`os/src/trap/mod.rs`

![image-20231214223205294](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214223205294.png)

修改`main.rs`

![image-20231214223451487](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214223451487.png)

## 4. 运行成功！

> 要在user目录下再次执行make build将应用程序编译成二进制文件

![image-20231214225703313](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214225703313.png)

> 箭头指出的地方显示power_3运行时切换为power_5，又在power_5执行一段时间后切换回power_3

# 二. 思考题

## 1. 分析分时多任务是如何实现的

首先基于`mtime`和`mtimecmp`实现定时器，当时间超过`mtimecmp`时触发时钟中断，然后通过调用`suspend_current_and_run_next`实现应用的切换，实现分时多任务

## 2. 分析抢占式调度是如何设计和实现的

本实验中抢占式调度是通过时钟中断实现的，当中断发生时，表示当前任务的时间片已用尽，操作系统将保存当前任务的状态，使用`suspend_current_and_run_next()`选择另一个任务继续执行。这种机制确保了系统对任务的执行有更严格的控制，避免了某些任务过度占用CPU资源。

## 3. 对比上个实验实现的协作式调度与本实验实现的抢占式调度

实验4中的协作式调度是由应用程序调用`yield`主动让出CPU资源，而本实验中切换任务是在任务运行到达时间限制后触发中断实现的，由操作系统触发

# 三. Git提交截图

![image-20231214234224223](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214234224223.png)