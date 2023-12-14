> OS-Lab4 多道程序与协作式调度 21301117 周书扬
>
> 本实验的主要目的是实现一个支持多道程序和协作式调度的操作系统。

# 一. 实验步骤

## 1. 实现应用程序

### 1.1 应用程序的放置

- 编写`build.py`脚本

![image-20231201105349846](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201105349846.png)

- 修改`user/Makefile`

![image-20231201105627842](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201105627842.png)

### 1.2 增加` yield`系统调用

- 增加sys_yield系统调用

![image-20231201110117368](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201110117368.png)

- 增加yield_用户库的封装

![image-20231201110606588](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201110606588.png)

### 1.3 实现测试应用程序

- `write_a.rs`

![image-20231201110915555](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201110915555.png)

- `write_b.rs`

![image-20231201110956330](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201110956330.png)

- `write_c.rs`

![image-20231201111027479](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201111027479.png)

- 编译应用程序

![image-20231201111713292](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201111713292.png)

## 2. 多道程序的加载

- 把常量定义到`os/src/config.rs`中

![image-20231201112703319](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201112703319.png)

- 在`os/src/loader`中增加加载应用程序的代码

![image-20231201113205752](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201113205752.png)

## 3. 任务的设计与实现

### 3.1 任务的上下文

![image-20231201114248002](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201114248002.png)

### 3.2 任务的运行状态及任务控制块

![image-20231201115012908](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201115012908.png)

### 3.3 任务切换

- 汇编代码

![image-20231201115159127](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201115159127.png)

- rust函数

![image-20231201115253066](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231201115253066.png)

### 3.4 任务管理器

![image-20231213221424591](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213221424591.png)

上述代码还调用了`loader`子模块的`init_app_cx`。因此，还需要在`os/src/loader.rs`增加如下代码：

![image-20231213221518647](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213221518647.png)

## 4. 实现sys_yield和sys_exit系统调用

- 修改`/os/src/syscall/process.rs`

![image-20231213222626962](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213222626962.png)

- 修改`os/src/syscall/mod.rs`

![image-20231213224852637](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213224852637.png)

## 5. 修改其他部分代码

- 注释trap子模块中`run_next_app()`部分的代码

![image-20231213222911082](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213222911082.png)

![image-20231213225158420](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213225158420.png)

- 注释掉`trap.S`中`__restore` 中的`mv sp`, `a0`。

![image-20231213223128618](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213223128618.png)

- 修改`main.rs`

![image-20231213223324907](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231213223324907.png)

## 6. 成功运行！

> 这里如果直接运行会报一个数组越界的错
>
> ![e0d5f5aa585d9109b3c2d4c84cd8557](C:\Users\Zoransy\AppData\Local\Temp\WeChat Files\e0d5f5aa585d9109b3c2d4c84cd8557.png)
>
> 把`user/src/bin`里面之前的程序除了本次实验的打印ABC全部删除，就好了

![image-20231214000935327](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231214000935327.png)

# 二. 思考问题

## 1. 分析应用程序是如何加载的

通过`builder.py`脚本实现将每个应用程序分配不同的内存地址用于加载，应用程序应该加载的内存地址由`os/src/loader.rs`中的`get_base_i`计算。另外，不同于批处理操作系统，多道程序操作系统所用的应用程序在内核初始化的时候就一起加载到内存中。

## 2. 分析多道程序如何设计和实现的

多道程序支持应用程序主动交出CPU的使用权。我们把一个计算执行过程称之为任务。一个应用程序的任务切换到另外一个应用程序的任务称为任务切换。任务切换过程中需要保存的恢复任务重新执行所需的寄存器、栈等内容称为任务的上下文。任务切换需要有任务上下文的支持。

我们通过实现在`os/src/task/context.rs`中的`TaskContext`数据结构来记录任务的上下文信息；在`os/src/task/task.rs`中实现在内核中维护任务的运行状态；在`os/src/task/switch.S`中实现任务切换的汇编代码，然后在`switch.rs`中将其封装为rust的函数；最后在`os/src/task/mod.rs`中实现一个全局的任务管理器来管理任务控制描述的应用程序。

# 3. 分析所实现的多道程序操作系统中的任务是如何实现的，以及它和理论课程里的进程和线程有什么区别和联系

任务实现过程和问题2中描述的方式一致

与进程线程的区别：

本实验中的多道程序仅支持应用程序主动交出CPU的使用权，而课程中的进程线程支持操作系统分配CPU资源

联系：

都支持上下文切换

# 三. Git提交截图

