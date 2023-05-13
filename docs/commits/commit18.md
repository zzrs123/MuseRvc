# commit18: 添加辅助信息

这个commit就是一个辅助step，增加生成汇编代码的注释（因为生成的汇编可读性已经特别特别低了，看起来很容易晕），添加注释更容易理解和静态debug。

但是这样看上去又会显得codegen很乱，可读性很差（因为rust的注释和println!的注释有一点重复）。

下面是我执行 `qemu-riscv64 -L $RISCV/sysroot/  target/riscv64gc-unknown-linux-gnu/debug/muservc '{ i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j; }'  > src/test.s` 的效果。

```as
# 定义全局main段
   .globl main
main:
   # 将fp压栈,fp属于“被调用者保存”的寄存器,需要恢复原值
   addi sp, sp, -8
   sd fp, 0(sp)
   # 将sp的值写入fp
   mv fp, sp
   # sp腾出StackSize大小的栈空间
   addi sp, sp, -16

# =====程序主体===============
   # 获取变量i的栈内地址为-8(fp)
   addi a0, fp, -8
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 将0加载到a0中
   li a0, 0
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # 将a0的值,写入到a1中存放的地址
   sd a0, 0(a1)
   # 获取变量j的栈内地址为-16(fp)
   addi a0, fp, -16
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 将0加载到a0中
   li a0, 0
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # 将a0的值,写入到a1中存放的地址
   sd a0, 0(a1)

# =====循环语句1===============

# Init语句1
   # 获取变量i的栈内地址为-8(fp)
   addi a0, fp, -8
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 将0加载到a0中
   li a0, 0
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # 将a0的值,写入到a1中存放的地址
   sd a0, 0(a1)

# 循环1的.L.begin.1段标签
.L.begin.1:
# Cond表达式1
   # 将10加载到a0中
   li a0, 10
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 获取变量i的栈内地址为-8(fp)
   addi a0, fp, -8
   # 读取a0中存放的地址,得到的值存入a0
   ld a0, 0(a0)
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # 判断a0<=a1
   slt a0, a1, a0
   xori a0, a0, 1
   # 若a0为0,则跳转到循环1的.L.end.1段
   beqz a0, .L.end.1

# Then语句1
   # 获取变量j的栈内地址为-16(fp)
   addi a0, fp, -16
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 获取变量j的栈内地址为-16(fp)
   addi a0, fp, -16
   # 读取a0中存放的地址,得到的值存入a0
   ld a0, 0(a0)
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 获取变量i的栈内地址为-8(fp)
   addi a0, fp, -8
   # 读取a0中存放的地址,得到的值存入a0
   ld a0, 0(a0)
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # a0+a1,结果写入a0
   add a0, a0, a1
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # 将a0的值,写入到a1中存放的地址
   sd a0, 0(a1)

# Inc语句1
   # 获取变量i的栈内地址为-8(fp)
   addi a0, fp, -8
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 将1加载到a0中
   li a0, 1
   # 压栈,将a0的值存入栈顶
   addi sp, sp, -8
   sd a0, 0(sp)
   # 获取变量i的栈内地址为-8(fp)
   addi a0, fp, -8
   # 读取a0中存放的地址,得到的值存入a0
   ld a0, 0(a0)
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # a0+a1,结果写入a0
   add a0, a0, a1
   # 弹栈，将栈顶的值存入a1
   ld a1, 0(sp)
   addi sp, sp, 8
   # 将a0的值,写入到a1中存放的地址
   sd a0, 0(a1)
   # 跳转到循环1的.L.begin.1段
   j .L.begin.1

# 循环1的.L.end.1段标签
.L.end.1:
# 返回语句
   # 获取变量j的栈内地址为-16(fp)
   addi a0, fp, -16
   # 读取a0中存放的地址,得到的值存入a0
   ld a0, 0(a0)
   # 跳转到.L.return段
   j .L.return

# =====程序结束===============
# return段标签
.L.return:
   # 将fp的值写回sp
   mv sp, fp
   # 将最早fp保存的值弹栈,恢复fp和sp
   ld fp, 0(sp)
   addi sp, sp, 8
   # 返回a0值给系统调用
   ret

```
