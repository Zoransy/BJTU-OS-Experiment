> OS-Lab3 批处理与特权级 21301117 周书扬
>
> 本实验的主要目的是实现一个简单的批处理操作系统并理解特权级的概念。

# 一. 实验步骤

## 1. 设计和实现应用程序

### 1.1 创建user目录

![image-20231129233028933](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129233028933.png)

### 1.2 实现应用程序与系统约定的两个系统调用sys_write和sys_exit

- 实现syscall.rs

![image-20231129233211863](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129233211863.png)

- 进一步封装lib.rs

![image-20231129233632193](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129233632193.png)

### 1.1.2 实现格式化输出

![image-20231129233731208](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129233731208.png)

### 1.1.3 实现语义支持

![image-20231129234100022](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129234100022.png)

### 1.1.4 将应用程序的起始物理地址调整为 0x80400000

![image-20231129234203260](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129234203260.png)

### 1.1.5 增加配置文件使用linker.ld文件

![image-20231129234458869](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129234458869.png)

### 1.1.6 定义用户库入口点，形成用户运行时库lib.rs

![image-20231129234809911](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129234809911.png)

### 1.1.7 基于模板实现应用程序

- 00hello_world.rs

![image-20231129235035327](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129235035327.png)

- 01store_fauld.rs

![image-20231129235110564](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129235110564.png)

- 02power.rs

![image-20231129235154447](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129235154447.png)

### 1.1.8 实现Makefile文件进行编译

![image-20231129235306578](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231129235306578.png)

- *然后编译失败了，所以重新看了一遍之前的步骤，发现删除main.rs的时候要y加回车所以没删掉，删除之后编译通过了

![image-20231130002159013](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130002159013.png)

## 2. 链接应用程序到内核

![image-20231130002341667](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130002341667.png)

## 3. 找到并加载应用程序二进制码

- 实现batch.rs

![image-20231130002557306](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130002557306.png)

- 增加依赖

![image-20231130003756144](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130003756144.png)

## 4. 实现用户栈和内核栈

- 增加batch.rs中的实现

![image-20231130004021115](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004021115.png)

- 定义TrapContext

![image-20231130003734672](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130003734672.png)

## 5. 实现trap管理

### 5.1 Trap上下文的保存与恢复

- 实现mod.rs

![image-20231130004147976](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004147976.png)

- 实现trap.S

![image-20231130004235296](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004235296.png)

### 5.2 Trap分发与管理

- 在mod.rs中实现trap_handler

![image-20231130004419043](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004419043.png)

- 增加依赖

![image-20231130004521538](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004521538.png)

### 5.3 系统调用处理

- 实现syscall模块 mod.rs

![image-20231130004638860](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004638860.png)

- 实现fs.rs

![image-20231130004719053](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004719053.png)

- 实现process.rs

![image-20231130004753936](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004753936.png)

## 6. 执行应用程序

![image-20231130004948441](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130004948441.png)

## 7. 修改main.rs

![image-20231130005116931](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130005116931.png)

## 8. 运行系统内核

![image-20231130111359943](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130111359943.png)

# 二. git提交截图

![image-20231130122410469](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130122410469.png)

# 三. 思考题

## 1. 分析应用程序的实现过程，并实现一个自己的应用程序

### 应用程序执行过程：

- 首先编写正确的程序源代码
- 编译源代码到二进制机器码
- 使用编译脚本动态链接编译后的机器码
- 根据batch模块加载应用程序

### 实现一个自己的应用程序

实现了一个打印数组的功能

- 在user目录编译

![image-20231130112330478](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130112330478.png)

- 运行系统内核查看输出

![image-20231130113423858](C:\Users\Zoransy\AppData\Roaming\Typora\typora-user-images\image-20231130113423858.png)

程序代码：

```rust
#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("My first rust array program.");
    let mut numbers = [1, 2, 3, 4, 5];
    print!("Numbers: ");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!("Array End"); // 换一下行，因为打印数组用的是print不是println
    0
}
```

## 2. 应用程序的链接、加载和执行过程

### 2.1 链接

应用程序编译为机器码后通过`build.rs`脚本生成一个汇编文件 `src/link_app.S`，该汇编文件包括关于应用程序的元数据，在链接时将应用程序的数据正确地嵌入到可执行文件中。该脚本由主要由以下几部分组成：

- **`main` 函数：**

  - `println!("cargo:rerun-if-changed=../user/src/");`: 通知 Cargo，如果 `../user/src/` 目录下的文件发生变化，就重新运行该脚本。

  - `println!("cargo:rerun-if-changed={}", TARGET_PATH);`: 通知 Cargo，如果 `TARGET_PATH` 目录下的文件发生变化，就重新运行该脚本。

  - `insert_app_data().unwrap();`: 调用 `insert_app_data` 函数，该函数将应用程序的元数据插入到 `src/link_app.S` 文件中。

- **`static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";`:**
  - 定义了目标路径，指定了用户应用程序编译输出的位置。

- **`insert_app_data` 函数：**

  - 打开 `src/link_app.S` 文件以供写入。

  - 通过 `read_dir("../user/src/bin")` 读取用户应用程序目录下的所有二进制文件。

  - 构建一个排序后的应用程序名称的列表。

  - 在 `src/link_app.S` 文件中写入关于应用程序数量和地址的元数据。

  - 遍历应用程序列表，为每个应用程序生成对应的汇编代码，将应用程序二进制数据嵌入到可执行文件中。

### 2.2 加载

通过`batch.rs`模块实现保存程序内存位置以及运行索引等信息，主要函数功能如下：

- **`AppManager` 结构体和 `AppManagerInner` 结构体：**

  - `AppManager` 是一个包装了 `AppManagerInner` 的结构体，使用 `RefCell` 来提供内部可变性。

  - `AppManagerInner` 存储了应用程序的相关信息，包括应用程序数量、当前应用程序索引以及应用程序在内存中的起始地址。

- **`lazy_static!` 宏：**
  - 使用 `lazy_static!` 宏声明了一个全局的 `APP_MANAGER` 变量，该变量在第一次使用时进行实际初始化。这个全局变量用于管理应用程序的加载和运行。

- **`init` 函数和 `print_app_info` 函数：**

  - `init` 函数调用了 `print_app_info` 函数，用于打印应用程序的信息。在实际应用中，可能还会进行一些初始化操作。

  - `print_app_info` 函数输出应用程序的数量以及它们在内存中的位置信息。

- **run_next_app` 函数：**

  - 获取当前应用程序的索引。

  - 调用 `load_app` 函数加载当前应用程序的二进制数据到内存中。

  - 将应用程序索引递增，以便下次运行下一个应用程序。

  - 调用外部函数 `__restore` 来切换到应用程序的上下文，实现从内核态到用户态的切换。这里使用了一个名为 `KERNEL_STACK` 的堆栈，以及 `TrapContext` 结构体来保存和恢复上下文信息。

  - 最后，通过 `panic!` 宏表明在 `run_next_app` 函数中的代码应该是不可达的，因为切换到应用程序上下文后，控制流不会回到这里。

### 2.3 执行过程

> 因为感觉实验手册写得足够详细了所以直接复制了

在执行应用程序之前，需要跳转到应用程序入口`0x80400000`，切换到用户栈，设置`sscratch`指向内核栈，并且用S特权级切换到U特权级。我们可以通过复用`restore`的代码来实现这些操作。这样的话，只需要在内核栈上压入一个启动应用程序而特殊构造的Trap上下文，再通过`restore`函数就可以实现寄存在为启动应用程序所需的上下文状态。

## 3. Trap的实现方式

- **Trap 上下文的保存与恢复：**

  - 在 `os/src/trap/trap.S` 中，`__alltraps` 宏保存了 Trap 上下文。

  - 在 `os/src/trap/trap.S` 中，`__restore` 宏从保存的内核栈上的 Trap 上下文中恢复寄存器状态。

- **Trap 分发与处理：**

  - `os/src/trap/mod.rs` 中的 `trap_handler` 函数用于根据 `scause` 寄存器的值进行 Trap 的分发与处理。

  - 当 Trap 是用户环境调用 (`UserEnvCall`) 时，调用 `syscall` 处理系统调用，并更新上下文中的寄存器。

  - 当 Trap 是存储故障 (`StoreFault` 或 `StorePageFault`) 时，输出错误信息并调用 `run_next_app` 启动下一个应用程序。

  - 当 Trap 是非法指令 (`IllegalInstruction`) 时，同样输出错误信息并调用 `run_next_app` 启动下一个应用程序。

  - 对于其他类型的 Trap，抛出 panic。

- **Trap 处理入口点的设置：**
  - 在 `os/src/trap/mod.rs` 中的 `init` 函数中，通过 `stvec::write` 设置 `stvec` 寄存器，将 Trap 处理入口点指向 `__alltraps`。