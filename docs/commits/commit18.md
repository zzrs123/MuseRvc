# commit18: 添加辅助信息

这个commit就是一个辅助step，增加生成汇编代码的注释（因为生成的汇编可读性已经特别特别低了，看起来很容易晕），添加注释更容易理解和静态debug。

但是这样看上去又会显得codegen很乱，可读性很差（因为rust的注释和println!的注释有一点重复）。

下面是我执行 `qemu-riscv64 -L $RISCV/sysroot/  target/riscv64gc-unknown-linux-gnu/debug/muservc '{ i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j; }'  > src/test.s` 的效果。
