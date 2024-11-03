# rust-web-starter
rust-web-starter demo


rust web demo
整合了axum,过滤器，配置文件整合。

另拉一个分支整合mysql

本分支在主分支的基础上，整合了mysql


windows编译说明
windows上编译直接编译即可
1. 进入项目 执行`cargo build --release `
2. 找到编译好的文件, 直接拿去用即可

linux编译说明

windows上编译Linux软件，使用 wls 进行编译
1. 安装wls cmd运行  `wsl --install`
2. 安装rust cmd 运行`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. 进入wls， 安装linux编译组件，`sudo apt-get install musl-tools`
4. 进入项目 执行`cargo build --release --target x86_64-unknown-linux-musl`
5. 编译好的文件在`target/x86_64-unknown-linux-musl/release`, 复制编译好的文件到home目录`cp 编译好的文件 ~`
6. windows文件浏览器打开`\\wsl.localhost\Ubuntu\root`即可看到文件，记得下载后删除
7. 拿着打好的文件到目标系统去执行测试吧


[参考资料](https://juejin.cn/post/7380163683009495103)