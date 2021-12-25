# latest-files
一个简单命令行程序，用于展示最近修改的文件。

这是我初学Rust语言后的第一个练手作品，同时也是为了让我在使用命令行时可以快速以最近修改时间排序的方式查看文件和文件夹。

## 使用方式
使用`latest-files.exe --help`可以查看自带的帮助信息

(TODO: 重写此帮助文档)

```plaintext
USAGE:
    latest [FLAGS] [OPTIONS] [COUNT]

FLAGS:
    -h, --help             Prints help information
    -n, --no-index         Do NOT show index of files.
    -r, --relative-time    Show relative time(timeago). Repeat 2 times('-rr') to use more detailed relative time format.      
    -l, --short-list       Only list file names.
    -V, --version          Prints version information

OPTIONS:
    -p, --path <PATH>    Sets a custom path instead of the current directory. [default: .]

ARGS:
    <COUNT>    Specify how many files to show. [default: 3]
```

------
这程序只是我的练手之作，我深知其中代码可能含有bug、性能问题等质量问题，并且本质上是各种开源软件库的混合体，本身并没有什么技术含量，所以这个程序使用[WTFPL](http://www.wtfpl.net/)开源。我未来可能会继续更新也可能不更新，但仍希望获得指导和建议。
