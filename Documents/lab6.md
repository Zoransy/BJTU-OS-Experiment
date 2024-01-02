# 一. 实验步骤

## 1. 在内核中支持动态内存分配

- 增加依赖

![image-20231215103950702](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215103950702.png)

- 在`main.rs`中引入`alloc`依赖

![image-20231215104103231](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104103231.png)

- 实现全局动态内存分配器

![image-20231215104200275](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104200275.png)

- 处理动态内存分配失败的情况

![image-20231215104426109](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104426109.png)

![image-20231215104452715](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104452715.png)

- 测试动态内存分配

![image-20231215104527638](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104527638.png)

- 修改`main.rs` `config.rs`中代码

![image-20231215104837370](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104837370.png)

![image-20231215104929935](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215104929935.png)

- 实现`os/src/mod.rs`

![image-20231215105018100](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215105018100.png)

## 2. 实现虚拟地址与物理地址的基本定义

> 总之复制很多代码到`os/src/mm/address.rs`中，实现最基本的数据结构和相关操作

![image-20231215105741840](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215105741840.png)

# 3. 定义页表项数据结构

- 实现标志位`PTEFlags`

![image-20231215113600430](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215113600430.png)

![image-20231215113542719](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215113542719.png)

- 增加依赖

![image-20231215110043157](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215110043157.png)

- 实现`PageTableEntry`

![image-20231215110422987](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215110422987.png)

- 修改`os/src/mm/mod.rs`

![image-20231215110503358](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215110503358.png)

## 4. 实现物理帧的管理与分配

- 设置物理内存终止地址

![image-20231215111428505](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215111428505.png)

- 实现物理帧管理

![image-20231215111537175](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215111537175.png)

- 增加`sync`模块，实现`UPSafeCell`

![image-20231215111729561](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215111729561.png)![image-20231215111805016](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215111805016.png)

![image-20231215111837910](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215111837910.png)

- 物理帧管理测试

![image-20231215112127855](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215112127855.png)

![image-20231215112606931](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215112606931.png)

- 测试成功

![image-20231215113728808](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215113728808.png)

## 5. 多级页表管理

> 依然是粘贴各种代码，实现基本数据结构和操作

![image-20231215113927511](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215113927511.png)

## 6. 内核与应用的地址空间

- 内核地址空间

> 粘贴粘贴粘贴粘贴 主要实现了地址空间的抽象和创建内核地址空间

![image-20231215114751818](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231215114751818.png)

- 修改链接脚本

> 这里不知道是实验手册有问题还是我看漏了，选中的这行`sbss_with_stack`手册里好像是没有的，如果没有这一行最后build会失败

![image-20231227224005458](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227224005458.png)

- 修改`loader`子模块

![image-20231226234223273](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231226234223273.png)

- 实现解析ELF格式的数据

![image-20231226234453011](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231226234453011.png)

- 增加`xmas-elf`依赖

![image-20231226234559273](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231226234559273.png)

- 实现`memory_set`子模块

![image-20231226234823252](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231226234823252.png)

- 修改`config.rs`

![image-20231226234857199](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231226234857199.png)

## 7. 实现基于地址空间的分时多任务

- 创建内核地址空间

![image-20231227164207939](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227164207939.png)

- 进行内存管理子系统的初始化

![image-20231227214807830](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227214807830.png)

- 检查内核地址空间的多级页表设置

![image-20231227164446550](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227164446550.png)

- 实现跳板机制
- 扩展`Trap`上下文

![image-20231227170453661](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227170453661.png)

- 实现地址空间的切换

![image-20231227170542294](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227170542294.png)

- 建立跳板页面

![image-20231227170741136](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227170741136.png)

- 加载和执行应用程序
- 修改任务子模块，并更新任务控制块的管理

![image-20231227222214707](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227222214707.png)

- 更新`config.rs`，在内核初始化时加载所有应用程序

![image-20231227171151202](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227171151202.png)

- 修改`TaskManager`的实现

![image-20231227171416837](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227171416837.png)

- 修改`/os/src/task/switch.S`

![image-20231227171513762](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227171513762.png)

- 修改`switch.rs`

![image-20231227171550910](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227171550910.png)

- 改进`Trap`的处理

![image-20231227174245473](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227174245473.png)

- 在每一个应用程序第一次获得CPU权限时，内核栈顶放置在内核加载应用的时候构造的一个任务上下文

![image-20231227174331734](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227174331734.png)

- 改进`sys_write`的实现

![image-20231227174502333](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227174502333.png)

- 修改`sys_write`系统调用

![image-20231227174906156](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227174906156.png)

## 8. 修改应用程序

- 删除`user/src/lib.rs`中的`clear_bss()`

  除了删除`clear_bss()`的实现外，注意删除`_start()`中调用的`clear_bss()`。

![image-20231227175044716](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227175044716.png)

![image-20231227175110514](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227175110514.png)

- 删除`build.py`

![image-20231227180514531](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227180514531.png)

- 修改`Makefile`文件

把`user/Makefile`文件中的`build.py`替换为`cargo build`

![image-20231228002850924](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231228002850924.png)

- 修改`main.rs`

![image-20231227180148698](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227180148698.png)

- 修改`build.rs`

![image-20231227180346793](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231227180346793.png)

## 8. 运行成功

应该算成功了吧，虽然有很多警告，好像是因为删代码没删干净

![image-20231228002931894](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231228002931894.png)

![image-20231228002939461](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231228002939461.png)

# 二. 思考题

### 1. 虚拟地址和物理地址的设计与实现

- **设计**: 虚拟地址（`VirtAddr`）和物理地址（`PhysAddr`）是为了方便内存管理和实现虚拟内存系统而设计的。在操作系统中，虚拟地址提供了一种抽象，允许每个程序感觉像是在使用一个大的、连续的内存空间，而实际上这些虚拟地址被映射到物理内存的不同部分。
- **实现**: 在您的代码中，这两种地址都被封装为含有一个 `usize` 字段的结构体。提供了从 `usize` 到这些类型的转换，以及这些类型之间的相互转换（例如，通过页号和页表来转换）。这种实现方式简化了地址的处理，并允许在物理和虚拟地址空间之间轻松转换。

### 2. 物理帧的管理与分配

- **管理**: 物理帧（内存的物理页）通过 `StackFrameAllocator` 来管理。这个分配器负责跟踪哪些物理页被使用，哪些是空闲的。
- **分配**: 通过 `frame_alloc` 函数分配物理帧。如果有可用的帧，它会返回一个包含物理页号的 `FrameTracker` 实例。当 `FrameTracker` 被丢弃时，对应的物理帧会被释放回分配器。

### 3. 内核和应用程序的地址空间实现

- **内核地址空间**: 通过 `MemorySet` 结构表示。在启动时，内核地址空间包括代码段、数据段、BSS段等。此地址空间始终激活并被所有进程共享。
- **应用程序地址空间**: 每个应用程序都有自己的 `MemorySet` 实例，表示其私有的地址空间。这些空间包括应用程序的代码、数据、堆栈等。在上下文切换时，会切换活跃的地址空间，确保每个应用程序只能访问自己的内存。

### 4. 基于地址空间的分时多任务实现

- **多任务**: 利用任务控制块（`TaskControlBlock`），系统维护了多个任务的状态。每个任务都有自己的虚拟地址空间（`MemorySet`），堆栈，以及其他必要的上下文信息。
- **上下文切换**: 当一个任务的时间片用完时，系统会保存其上下文（包括CPU寄存器等），并加载下一个任务的上下文。这个过程通过保存和恢复任务上下文来完成，确保每个任务在适当的时候运行。

### 5. 编写新的应用程序并测试验证结果

- 实现了一个新的计算2次幂的程序

![image-20231228004324027](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231228004324027.png)

- 运行结果：

看起来没什么问题

![image-20231228004407463](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231228004407463.png)

# 三. Git提交截图

![image-20231228004823505](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231228004823505.png)
