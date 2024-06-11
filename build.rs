use std::option_env;

fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }

    // build_with_serde(include_str!("build_opts.json"));
    tonic_build::configure()
        .out_dir("src/pb") // 设置生成代码的自定义目录
        .compile(
            &["miner_sign.proto"], // 指定.proto文件的位置
            &["./protobuf"],       // 指定.proto文件的搜索目录
        )
        .unwrap();
}
