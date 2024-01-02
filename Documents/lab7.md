> 21301117 周书扬 OS-Lab7

# 一. 实验步骤

## 1. 增加重要的系统调用

- 修改`user/src/syscall.rs`增加`waitpid` ` getpid` `exec` ` read`系统调用

![image-20240102183012753](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102183012753.png)

- 在`user/src/lib.rs`封装系统调用为应用程序使用的形式

![image-20240102183103557](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102183103557.png)

- 实现用户初始程序`initproc`

![image-20240102183206138](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102183206138.png)

- 封装能够从标准输入读取一个字符的函数`getchar`

![image-20240102191001695](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102191001695.png)

- 实现user shell程序

![image-20240102191112209](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102191112209.png)

- 在用户库`user_lib`中支持动态内存分配

![image-20240102192505459](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102192505459.png)

## 2. 在内核中增加系统调用

- 修改`os/src/syscall/mod.rs`增加`fork`、`waitpid`、`getpid`、`read`系统调用

![image-20240102192804694](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102192804694.png)

- 修改`os/src/syscall/fs.rs`，实现`sys_read`系统调用

![image-20240102193856064](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102193856064.png)

- 修改`os/src/syscall/process.rs`实现其他系统调用

![image-20240102194042514](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102194042514.png)

## 3. 应用的链接与加载

- 修改编译链接辅助文件`os/build.rs`

![image-20240102194528347](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102194528347.png)

- 基于名字的应用加载

![image-20240102194657122](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102194657122.png)

## 4. 进程标识符与内核栈

- 实现进程标识符（还有一系列模块）

![image-20240102204459579](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102204459579.png)

- 修改`os/src/mm/memory_set.rs`

![image-20240102204619127](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102204619127.png)

## 5. 修改实现进程控制块

![image-20240102213751906](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102213751906.png)

## 6. 实现任务管理器

![image-20240102213920129](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102213920129.png)

## 7. 增加处理器管理结构

![image-20240102214029655](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102214029655.png)

## 8. 创建初始进程

![image-20240102214911936](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102214911936.png)

## 9. 进程调度机制

![image-20240102215137718](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102215137718.png)

## 10. 进程的生成机制

![image-20240102215237275](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102215237275.png)

![image-20240102215326667](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102215326667.png)

- 修改页表

![image-20240102215954613](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102215954613.png)

- 重新获取trap上下文

![image-20240102220259845](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102220259845.png)

## 11. 进程资源回收机制

![image-20240102220608942](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102220608942.png)

![image-20240102220653839](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102220653839.png)

- 修改`main.rs`

![image-20240102220749711](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102220749711.png)

## 12. 成功运行shell（debug之后）

![image-20240102230628479](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102230628479.png)

# 二. 思考题

## 1. 应用的链接与加载实现

- 应用链接是通过将应用程序名和对应的ELF（Executable and Linkable Format）格式数据相关联来实现的。这是通过修改编译链接辅助文件`build.rs`来完成的。
- 应用加载则是通过`loader.rs`模块，它使用全局可见的只读向量`APP_NAMES`来存储所有应用的名称。加载一个特定的应用时，系统根据应用名称查找并加载其对应的ELF数据。

## 2. 进程标识符（PID）和进程控制块（PCB）的设计与实现

- 进程标识符（PID）是唯一的，实现为`PidHandle`类型。PID是通过`PidAllocator`进行管理的，它可以分配和回收PID。
- PCB是通过`TaskControlBlock`结构体实现的，它包含了与进程相关的所有信息，如PID、内核栈、任务上下文（`TaskContext`）、任务状态（`TaskStatus`）、内存集（`MemorySet`）等。

## 3. 任务管理的实现

- 任务管理是通过`TaskManager`类实现的，它维护一个准备队列（`ready_queue`），用于存储就绪状态的进程。
- 当进程切换时，任务管理器从准备队列中取出一个进程并运行。

## 4. 进程的调度、生成及资源回收

- 调度是通过`Processor`类来管理的。这个类负责保存当前运行的任务和空闲任务上下文（`idle_task_cx`）。
- 进程的生成主要依靠`fork`和`exec`系统调用。`fork`用于创建与父进程几乎相同的子进程，而`exec`用于加载并执行一个新的程序。
- 进程资源的回收是通过`exit_current_and_run_next`函数实现的，该函数会结束当前进程并将控制权转移给下一个进程。父进程通过`sys_waitpid`系统调用来回收子进程的资源。

# 三. Git提交截图



# 四. 其他问题

- 在user目录执行`make build`指令后报错

![1e7ce3de9431d14a665a621babf1231](C:\Users\Zoransy\AppData\Local\Temp\WeChat Files\1e7ce3de9431d14a665a621babf1231.png)

返回值不正确，在`syscall.rs`中修改`sys_exit`，添加panic并修改返回值

![image-20240102224929364](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102224929364.png)

- 找不到模块的报错（和一堆其他报错）

![image-20240102225926928](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20240102225926928.png)

最后对照示例代码发现少了很多代码，可能是我复制漏了也可能是手册漏了，总之最后修好了