use log::Level;
use log::debug;
use log::error;
use log::info;
use log::log;
use log::log_enabled;
use log::trace;
use log::warn;

/// Rust的日志系统基于 log crate，提供了所有关于日志的门面（facade），即没有提供任何功能，只有接口
/// env_logger是一种实现
///
/// > 选择哪种日志实现取决于你代码的消费者，如果你写的是一个lib crate，不要使用实现，只要通过log打印日志即可。你/或其他人的可执行程序只需要初始化它们的logger就可以处理日志调用了
///
/// log crate提供了 log! 宏，接收一个 Level，一条可以被格式化的信息和一个可选的 target，还提供了其它的更方便的日志宏。日志的 target 是一个额外属性（可选），让日志的实现方组织日志。如果不写默认是当前的模块名
///
/// log_enabled! 宏会返回当前激活的logger是否处理指定警告级别，这通常和 Debug 日志组合使用，展示高代价操作的信息
///
/// env_logger是Rust nursery项目。它打印日志到 stderr。依赖 RUST_LOG 环境变量来过滤什么日志被展示，默认是error，只会显示指定等级及以上级别的日志，除此之外，可以设置为target，这样它会显示所有该目标的日志，无论级别。还可以组合起来 例如 logging=warn，可以使用逗号来分隔多个过滤器。还可以使用 / 后加上过滤词来展示仅包含指定正则表达式的日志内容
///
/// 很多语言也包含 Fatal 日志级别，Rust中之前的 panic!() 是这样做的，如果你想以其它方式打印panic信息，你可以调用 std::panic::set_hook() 和你要执行的操作，来替换默认行为
///
/// env_logger的好的替代产品是slog，提供了扩展的结构化日志，学习曲线更陡峭
fn main() {
    env_logger::init();
    // level seq: error > warn > info > debug > trace
    log!(Level::Debug, "env_logger has been initialized ");

    info!("The program has started");
    // log的目标（target）是它的父模块，当前例子就是 logging ，因为在一个bin crate中
    info!(target: "extra_info", "This is additional info that will only show if you \
    activate info level logging for the extra_info target");

    warn!("Something that requires your attention happened");

    if log_enabled!(Level::Debug) {
        let data = expensive_operation();
        debug!("The expensive operation returned: \"{}\"", data)
    }
    error!("Something terrible happened!");
}

fn expensive_operation() -> String {
    trace!("Starting an expensive operation");
    let data = "Imaging this is a very very expensive task".to_string();
    trace!("Finished the expensive operation");
    data
}
