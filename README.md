# actix_posts
投稿アプリ

## memo
- ホットリロード時の注意<br/>
DB代わりにdata.jsonファイルにはいてるので、ホットリロード時はこのファイルを対象外とすること。<br/>
現状、クッキーベースのセッション管理により投稿者名を保持しているが、<br/>
新規投稿直後にjsonファイルの変更が検知されて再起動→クッキーの値が変わり保持できなくなる。<br/>
以下のコマンドで回避可能<br/>
`cargo watch -i data.json -x run`

- ディレクトリ構成について<br/>
  レイヤ構造が分かりやすいように、NestJS風 + actix-web公式のネストルーティングサンプルを参考にした</br>
  他もそのうち見直し

## 動作の一例
### 一覧画面
<img width="1504" alt="image" src="https://github.com/user-attachments/assets/253d20ab-3798-46ce-8e78-542bbe11a80f">


### 投稿の詳細
<img width="1508" alt="image" src="https://github.com/user-attachments/assets/6afecba6-69ea-496d-8a4d-32d739c9afea">

 
### 新規投稿
<img width="1507" alt="image" src="https://github.com/user-attachments/assets/b9ff4199-5560-48be-a6fa-f6a51b6a7a51">

<img width="1505" alt="image" src="https://github.com/user-attachments/assets/6f9bd0c4-80c6-4eec-8295-6420e4259d7e">
