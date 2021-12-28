# RTT-RS

### 简介：

为 **rt-thread / rt-smart** 提供的 **rust** 支持层，提供了线程及线程同步，文件系统和网络功能



### rt-thread 使用说明：

1. 新建一个 lib 工程 crate ，并依赖此库。

2. 指定库的类型

   ```
   [lib]
   name = "app"
   crate-type = ["staticlib"]
   ```

3. 添加相应的标头，编译成 libxxx.a 库

   ```rust
   #![no_std]
   
   use rtt_rs::*;
   
   // 指定入口函数
   entry!(main);
   
   fn main() {}
   ```

4. 在 rt-thread 中的 main 中运行库的入口函数 rust_main

   ```c
   int main(void)
   {
       // 你也可以选择新建一个线程来执行 rust_main
       // main 函数在　rtt　中一般也是以一个线程的方式执行，但是可能栈空间满足不了你的需求
       int8_t ret = rust_main();
       return 0;
   }
   ```

5. 添加相应的库到 rt-thread 中并完成编译。