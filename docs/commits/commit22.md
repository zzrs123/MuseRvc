# commit22: 支持 int 关键字定义变量

## 1. 程序功能

此前我们总是默认 我们使用的变量 和 数字 都是Int类型。对它增加显式的声明。测试用例更改如下：

![1](pics/commit22-pic/func.png)

## 2. Rust实现

这里又困扰了我很久很久，但实际上并没有我想的那么难。主要是rust语法本身的问题。

### 2.1 关于int关键字的实现思路

主要是文法，引入一个变量声明与赋值的解析层次，在combound_stmt 轮询 stmt的循环中，增加  declaration ，也就是说 declaration 和 stmt 是平行的。

接着在声明处理中，依次解析变量的类型（给Obj结构体的ty成员赋type），解析声明多变量的情况，最后调用赋值解析Assign函数。

### 2.2 rust生命周期引发的bug

之前自己的Token系统的数据结构，与现在的比较如下：

![1](pics/commit22-pic/diff1.png)

正是&str引发了一系列的生命周期问题，如果继续沿用&str切片，会引发一系列rust函数和变量的周期问题，所以果断改了。

下面是运行结果。

![1](pics/commit22-pic/result.png)
