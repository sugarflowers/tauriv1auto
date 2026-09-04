# 📄 README 用：説明＋免責事項（日本語 / 英語）


## 🎮 このレポジトリについて / About This Repository
このレポジトリは **RPGツクール MZ の HTML5 出力を Tauri v1 でラップし、  
Windows / macOS 用のネイティブアプリを自動ビルドするためのテンプレート**です。

GitHub Desktop を使うことで、  
ツクール MZ の HTML5 コンテンツを `src/` にコピーして push するだけで、  
GitHub Actions が自動的にアプリを生成します。

This repository is a **template for wrapping RPG Maker MZ HTML5 export with Tauri v1**,  
allowing you to automatically build native applications for **Windows and macOS**.

By using GitHub Desktop, you can simply copy your RPG Maker MZ HTML5 output into the `src/` folder and push the changes.  
GitHub Actions will automatically build the application for you.

---

## 🧩 使い方（初心者向け） / How to Use (Beginner Friendly) 
1. GitHub Desktop をインストールしてログインする  
2. このレポジトリを Clone する  
3. RPGツクール MZ でゲームを HTML5 書き出しする  
4. 出力されたファイル一式を `src/` フォルダにコピーする  
5. GitHub Desktop で Commit → Push  
6. GitHub Actions が自動でビルドし、  
   - Windows：`exe / msi / nsis`  
   - macOS：`.app / .dmg`  
   が artifact として生成されます

1. Install GitHub Desktop and log in  
2. Clone this repository  
3. Export your RPG Maker MZ project as **HTML5**  
4. Copy all exported files into the `src/` folder  
5. Commit and Push using GitHub Desktop  
6. GitHub Actions will automatically build:  
   - Windows: `exe / msi / nsis`  
   - macOS: `.app / .dmg`  
   These files will appear as downloadable artifacts.

---

## 🔄 コンテンツ更新時 / When Updating Your Game 
ゲーム内容を更新した場合は、  
HTML5 書き出し → `src/` に上書きコピー → Commit → Push  
とするだけで、**変更分だけ** GitHub にアップロードされます。

When you update your game,  
export HTML5 again → overwrite the `src/` folder → Commit → Push.  
GitHub will upload **only the changed files**, making updates efficient.

---

## 🖼 カスタマイズ / Customization 
以下の項目を自由に変更できます：

- ウィンドウサイズの調整  
- ウィンドウタイトルの変更  
- アプリアイコン（Windows / macOS）の変更  
- `tauri.conf.json` の設定  
- HTML5 側の画面サイズ調整  

You can freely customize:

- Window size  
- Window title  
- Application icons (Windows / macOS)  
- `tauri.conf.json` configuration  
- HTML5 canvas / screen size adjustments  

---

## ⚠️ 免責事項 / Disclaimer 
このレポジトリはテンプレートとして提供されるものであり、以下について保証はありません：

- このテンプレートを利用したことによる **いかなる問題・損害についても責任は負いません**  
- RPGツクール MZ や Tauri の **バージョン変更により動作しなくなる可能性があります**  
- GitHub Actions の仕様変更により **ビルドが失敗する場合があります**  
- OS のアップデートにより **生成されたアプリが動作しなくなる可能性があります**

問題が発生した場合や動作しなくなった場合は、  
**Issue で連絡をいただけると助かります。**

This repository is provided as a template, and the following are **not guaranteed**:

- I am **not responsible for any issues or damages** caused by using this template  
- Future updates to RPG Maker MZ or Tauri may cause the project to stop working  
- GitHub Actions changes may cause builds to fail  
- OS updates may prevent the generated applications from running

If you encounter problems or the project stops working,  
please feel free to contact me through **GitHub Issues**.

---

## 📬 お問い合わせ / Contact
不具合報告や改善提案は GitHub の Issues からお願いします。

For bug reports or suggestions, please use the GitHub Issues page.

