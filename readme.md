# HomeCloud

根据网络流量启停家中Linux服务器

# 配置文件

请看example.toml


# 对于Openwrt的特别说明

由于部分厂商出场系统啥都没有，所以我编译了最新的ssh客户端，这部分在release里，并在红米ax6官方固件验证可用

`dbclient` 最新的dropbear ssh客户端

`dropbearconvert` dropbear的密钥转换器，需要将openssh的密钥转换为它的才能用

`etherwake` 如果没有这个就去opkg下一个吧 https://openwrt.org/packages/pkgdata/etherwake

有的系统也没有nohup，可以使用`(./homecloud example.toml  >/dev/null 2>&1 )&` 来后台运行

# 编译说明

首先需要编译libpcap
* 创建`build_lib`文件夹, clone libpcap
* 输入` sudo docker run -it --rm -v libpcap文件夹:/home/data  muslcc/x86_64:aarch64-linux-musl`进入交叉编译环境
* 在里面创建`build-release`文件夹，并输入`cmake -DCMAKE_BUILD_TYPE=Release ..`
* 使用`make`完成编译

编译主项目

* 更改build.sh，将路径和你的电脑匹配
* 按照 https://github.com/cross-rs/cross 安装交叉编译环境
* sh build.sh