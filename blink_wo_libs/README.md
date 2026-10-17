# Lチカ without libraries

- ライブラリを使わずにLチカするコード
- 以下を参考にした
  - https://github.com/verylowfreq/example_rust_ch32v203/
- それ以外だと以下
  - CH32V2x_V3xRM.PDF (リファレンスマニュアル)
  - https://github.com/openwch/ch32v20x/

## Memo

- link.x では FLASH の先頭アドレスを 0x00000000 としているが実際には 0x08000000 に書き込まれる
  - CH32V203 は BOOT0 ピンが LOW の場合、0x00000000〜 の領域が 0x08000000〜 にエイリアスされのでこれで問題ない
  - なお BOOT0 ピンが HIGH の場合は 0x00000000〜 の領域が 0x1FFF0000〜 (System FLASH) にエイリアスされプログラムを書き込めるようになる
