# README

## このテンプレートについて / About This Template

このテンプレートは、RPGツクール MZ の HTML5 出力を Tauri v1 でラップし、 Windows と macOS 用のネイティブアプリを自動ビルドするためのプロジェクトです。 ZIP をダウンロードして展開し、GitHub Desktop で管理することで、 誰でも簡単にクロスプラットフォームの実行ファイルを生成できます。

This template wraps the HTML5 export of RPG Maker MZ using Tauri v1, allowing automatic builds of native applications for Windows and macOS. By downloading the ZIP and managing it with GitHub Desktop, anyone can easily generate cross‑platform executables.

## 必要なもの / Requirements

- GitHub アカウント
- GitHub Desktop
- RPGツクール MZ（HTML5 出力が必要）
- インターネット接続（GitHub Actions によるビルドのため）



- GitHub account
- GitHub Desktop
- RPG Maker MZ (HTML5 export required)
- Internet connection (for GitHub Actions builds)



## GitHub への登録 / Creating a GitHub Account

GitHub を利用するためにはアカウントが必要です。 公式サイトでメールアドレスを登録し、指示に従ってアカウントを作成してください。

To use GitHub, you need an account. Visit the official website, register your email address, and follow the instructions to create an account.

## GitHub Desktop の入手 / Downloading GitHub Desktop

GitHub Desktop は GitHub の公式クライアントで、 ファイルの変更管理やアップロードを簡単に行うことができます。 公式サイトからダウンロードしてインストールしてください。

GitHub Desktop is the official GitHub client, making it easy to manage and upload your project files. Download and install it from the official website.

## GitHub Desktop の基本操作 / Basic Usage of GitHub Desktop

GitHub Desktop を起動し、 「Add existing repository」から ZIP を展開したフォルダを選択します。 ファイルを追加・変更したら、画面下部で Commit を行い、 その後 Push することで GitHub にアップロードされます。

Open GitHub Desktop and select “Add existing repository,” then choose the folder extracted from the ZIP. When you add or modify files, commit the changes at the bottom of the window, and push them to upload the updates to GitHub.

## ZIP の使い方 / How to Use the ZIP Package

ZIP を展開し、GitHub Desktop に読み込ませます。 RPGツクール MZ で HTML5 出力を行い、生成されたファイル一式を `src/` にコピーします。 Commit → Push を行うと、GitHub Actions が自動的にビルドを開始します。

Extract the ZIP and load it into GitHub Desktop. Export your RPG Maker MZ project as HTML5 and copy all generated files into the `src/` folder. Commit and push the changes, and GitHub Actions will automatically start building.

## コンテンツ更新時 / Updating Your Game

ゲーム内容を更新した場合は、再度 HTML5 出力を行い、 `src/` に上書きコピーして Commit → Push するだけで構いません。 GitHub は変更されたファイルだけをアップロードします。

When you update your game, export HTML5 again, overwrite the `src/` folder, and commit → push. GitHub will upload only the modified files.

## 修正方法（画面サイズ・タイトル・アイコン） / How to Modify Window Size, Title, and Icons

### 画面サイズの変更 / Changing Window Size

json

```
"window": {
  "width": 1280,
  "height": 720,
  "resizable": true
}
```

この設定はアプリのウィンドウサイズを指定します。 ゲーム画面に合わせて `width` と `height` を調整してください。 `resizable` を `false` にすると、ユーザーによるサイズ変更を禁止できます。

This configuration defines the application window size. Adjust `width` and `height` to match your game’s resolution. Setting `resizable` to `false` prevents users from resizing the window.

### ウィンドウタイトルの変更 / Changing Window Title

json

```
"package": {
  "productName": "Your Game Title"
}
```

`productName` を変更すると、アプリのウィンドウタイトルが変わります。 Windows と macOS の両方でこのタイトルが表示されます。

Changing `productName` updates the window title of the application. This title appears on both Windows and macOS.

### アイコンの変更 / Changing Application Icons

プロジェクト直下の `icon.png` を差し替えてください。 GitHub Actions が自動で Windows / macOS 用のアイコンを生成します。

Replace the `icon.png` file in the project root. GitHub Actions will automatically generate icons for Windows and macOS.



## 免責事項 / Disclaimer

このテンプレートは無保証で提供されます。 利用による問題・損害、ツクール MZ や Tauri のバージョン変更、 GitHub Actions や OS の更新による動作不良について責任は負いません。 問題が発生した場合は Issue から連絡をいただけると助かります。

This template is provided without warranty. I am not responsible for issues or damages caused by its use, nor for incompatibility due to updates in RPG Maker MZ, Tauri, GitHub Actions, or operating systems. If problems occur, please contact me through GitHub Issues.

## お問い合わせ / Contact

不具合報告や改善提案は GitHub の Issues からお願いします。

For bug reports or suggestions, please use the GitHub Issues page.





