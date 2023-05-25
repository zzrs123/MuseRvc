// use crate::lexer::Token;

// step21,想完善这个error系统，因为每次调用error！都需要一些额外的let来处理
/*=====================================================================
                            错误处理系统
                    CCURRENT_INPUT存储当前输入语句
// ====================================================================== */
pub(crate) static mut CURRENT_INPUT : Option<String> = None;

/*=====================================================================
                        verror_at
                    指示错误位置，输出报错消息
 ======================================================================*/ 
pub fn verror_at(loc:usize, err_str:&str) -> (){
    unsafe{
        eprintln!("{}", CURRENT_INPUT.as_deref().unwrap()); // 输出源串
    }
    // 根据loc找到出错位置，另起一行用^+报错消息输出
    let padding = " ".repeat(loc);
    eprint!("{}",padding);
    eprint!("{}", '^');
    eprint!("{}\n", err_str);
}

 /*=====================================================================
                error 宏
    尝试了很多方式，最后还是返璞归真复用error!宏
    在前面的error版本中直接向下增加就可以，但总感觉写的不够好。
    因为C中使用变参函数，Rust不支持，所以寻求使用宏匹配来解决
    跟C有点不同，C中区分了两种错误消息宏
    第一种处理：字符串解析为token过程中的errorAT，
              这个使用error宏的第一个匹配逻辑就可以达到同样效果
    第二种处理：解析token流时的错误errorTok，
              使用输入为token类型的第二个匹配逻辑
 ======================================================================*/ 
#[macro_export]
macro_rules! error {
    ($tok:ident, $msg:expr) => {
        verror_at($tok.loc, $msg);
        std::process::exit(1);
    };
    // 使用 format! 宏将 $c 的值插入到 $fmt 中，生成新的字符串 message。
    // 最后，我们将 $i 和 &message 一起传递给 verror_at 函数进行错误处理。
    ($i:ident, $fmt:expr, $c:expr) =>{{
        // let str = concat!($fmt, $c:expr) ;
        let message = format!($fmt, $c);
        verror_at($i,&message);
        std::process::exit(1);
    }};
    ($fmt:expr $(, $arg:expr)*) => {{
        eprint!(concat!($fmt, "\n") $(, $arg)*);
        std::process::exit(1);
    }};

}