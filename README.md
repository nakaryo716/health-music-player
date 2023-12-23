# コマンドラインオーディオ再生アプリ
## 動機
モバイルアプリで**nightsound**というアプリの著作権フリーの音楽が好きで愛用しているが、パソコン上でも流したいが、わざわざ調べたり、finderで選択するのがめんどくさいため、作成に至った。
## 実装
- ダウンロードしたmp3ファイルを読み込んでデコードし、再生するだけ  
- 現段階では一度再生したらプロセスを終了させない限り曲の停止が出来ないため、tokioを使用して割り込み処理をおこなうか、chanel等を使って書き直す予定
## 使用したクレート
- コマンドライン入力が簡単になる"proconio"
- オーディオプレイバックライブラリ"rodio"

## 実行
```cargo run```または```./target/debug/nightsound-sli-player``` 