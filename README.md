# 介绍
本项目是对huggingface的tokenizers源码的阅读分析并增加中文注解，以便帮助大家更好的理解tokenizers的源码。
为了更好的阅读，我将删除官方源码中不需要的内容，如授权文件、文档目录等，以及其他语言调用的桥接层SDK的源码。专注于tokenizers核心源码实现的阅读。
详细关于tokenizers的文档请见：[官方文档](https://huggingface.co/docs/tokenizers/intro)  或者访问我的使用翻译笔记：[翻译笔记](https://github.com/wcpsoft/huggingface-tokenizers-rust-zh_cn-doc)
<p>
    <br>
    <img src="https://huggingface.co/landing/assets/tokenizers/tokenizers-logo.png" width="600"/>
    <br>
<p>
<p>
    <img alt="Build" src="https://github.com/huggingface/tokenizers/workflows/Rust/badge.svg">
    <a href="https://github.com/huggingface/tokenizers/blob/main/LICENSE">
        <img alt="GitHub" src="https://img.shields.io/github/license/huggingface/tokenizers.svg?color=blue&cachedrop">
    </a>
    <a href="https://pepy.tech/project/tokenizers">
        <img src="https://pepy.tech/badge/tokenizers/week" />
    </a>
</p>

tokenizers是开源的分词库，提供了当今最常用的分词器的实现。

## 主要功能:

- 使用当今最常用的分词器训练新的词汇表并进行分词。
- 由于采用了Rust实现，以极快的速度完成包括训练和分词等功能，支持在设备上使用CPU进行处理，1GB文本所需时间不到20秒。
- 易于使用，同时非常多功能。
- 专为研究和生产设计。
- 可以正则化过程中会跟踪对齐信息。始终可以获取对应于给定令牌的原始句子部分。
- 可以完成所有预处理工作：截断、填充、添加模型所需的特殊令牌等。

