## 概要

個人の勉強用にRellyのコードをみながら実装していくリポジトリです。

## 情報

目次情報:

```
第1章：RDBMSを作ろう
作りながら内部のしくみを知る…… KOBA789
第2章：ディスクマネージャの実装
データの塊をディスクに書く・読む…… KOBA789
第3章：バッファプールマネージャの実装
データの塊をメモリにキャッシュする…… KOBA789
第4章：B+Treeの観察
巨大なデータを高速に検索する…… KOBA789
第5章：テーブルの実装
B+Treeにテーブルを格納する…… KOBA789
第6章：セカンダリインデックスの実装
プライマリキー以外でも高速に検索する…… KOBA789
```

## 詰まったとこ

## 勉強

### ヒープとヒープファイル

ヒープは木構造の１つ。親が常に子より大きくなるか等しくなるように設計するもの。ファイルの分割状態を管理するのに適したデータ構造と言えそう。

ヒープファイルとは、ページという大きさにファイルを分割し順序関係なくデータをページに書き込むことができるファイル。

単一のファイル内に、データブロックをいくつも作成するイメージ。

ヒープファイルのヒープってどんな意味だろう。

### Disk Manager

- ファイル操作（読み取り書き込み）のAPIを提供する。
- ページサイズごとにデータを格納・メモリに展開して、書き込み・読み込みする。

### What is `repr(C)`

https://doc.rust-lang.org/nomicon/other-reprs.html#alternative-representations

Rustで定義した構造体を外（外部呼び出し先）に渡す場合に必要。メモリ上のレイアウトがCで書いた場合と同じになるので、C言語を使っていれば外部アプリケーションでもデータを解釈できる。

## 参考

- 筆者のブログ：https://diary.hatenablog.jp/entry/2021/04/08/190000
