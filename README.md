# ポケットモンスター青(日本版)パッチ

## 女の子だってトレーナーになりたい！

ポケットモンスタークリスタルまでは、主人公の性別を選択できず、男主人公としてしかプレイできませんでした。このプログラムで、日本リリースのポケットモンスター青のROMファイルに埋め込まれたスプライトデータを書き換え、女主人公の「ブルー」さんとしてゲームを楽しむことができるようになります！

## 使い方

予め `cc` と `rgbds` がインストールされていることを確認してください。

> 現在 [d3npa/crystal-sav](https://github.com/d3npa/crystal-sav) のライブライが依存されますが、パスをハードコーディングしていますので、ご注意ください。後ほどライブラリを一つのレポジトリに抽出する予定ですが、そのときまでは環境を自分で用意してください！

スプライトをパッチできる形式に変換します

```sh
$ cd sprites && sh make.sh
...
```

ROMファイルにパッチを行います

```sh
$ cargo run '../Pocket Monsters Ao.gb'
...
   Compiling bytemuck v1.7.0
   Compiling sav-tools v0.1.0 (/home/user/projects/202107/gbc-hax/crystal-sav/sav-tools)
   Compiling aopacchi v0.1.0 (/home/user/projects/202107/gbc-hax/blue-patch/aopacchi)
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
     Running `target/debug/aopacchi '../Pocket Monsters Ao.gb'`
Read in 524288 bytes from '../Pocket Monsters Ao.gb'
Patching sprite data...
Found 1 match(es) for 'player_sprite_main': [14180]
Found 1 match(es) for 'player_battle_back': [33e0a]
Found 1 match(es) for 'player_sprite_bike': [14000]
Found 1 match(es) for 'player_fish_front': [7a017]
Found 1 match(es) for 'player_battle_front': [13361]
Found 1 match(es) for 'player_fish_side': [7a057]
Found 1 match(es) for 'player_fish_back': [7a037]
Saving patched game to 'patched_ao.gb'
```
