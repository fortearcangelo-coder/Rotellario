# Rotellario 📖

**Dizionario della Lingua Rotellese** — app nativa per **iOS** e **Android**, costruita con [Tauri 2](https://v2.tauri.app/).

Tutti i lemmi sono salvati **in locale sul dispositivo**: l'app funziona completamente offline, senza account né connessione. La sincronizzazione cloud (Gist GitHub) è opzionale e disattivata di default.

---

## Cos'è

Il Rotellario è un dizionario consultabile e modificabile: cerca i lemmi, filtra per categoria (aggettivo, sostantivo, verbo, locuzione, frase), passa dalla vista a griglia a quella alfabetica, aggiungi/modifica/elimina voci (modalità editor protetta da password). È la stessa interfaccia della versione web (`index.html`), impacchettata in un'app installabile sul telefono.

## Struttura del progetto

```
Rotellario/
├── index.html              # Versione web originale (invariata, per GitHub Pages)
├── src/
│   └── index.html          # Frontend dell'app mobile (local-first)
├── src-tauri/              # Backend nativo Tauri (Rust)
│   ├── Cargo.toml
│   ├── tauri.conf.json     # Configurazione app (nome, id, icone, bundle)
│   ├── build.rs
│   ├── capabilities/       # Permessi della webview
│   ├── icons/              # Icone generate per tutte le piattaforme
│   └── src/
│       ├── main.rs         # Entrypoint desktop
│       └── lib.rs          # Entrypoint condiviso (mobile + desktop)
├── package.json            # Script npm (dev/build/android/ios)
└── app-icon.png            # Icona sorgente 1024×1024 (per rigenerare le icone)
```

> Nota: `src-tauri/gen/` (progetti nativi Android/iOS generati) e `src-tauri/target/` non sono versionati — si rigenerano con i comandi qui sotto.

---

## Requisiti comuni

- **Rust** (stable) — installa con [rustup](https://rustup.rs/)
- **Node.js** ≥ 18 — per la CLI di Tauri

Poi, dalla cartella del progetto:

```bash
npm install
```

---

## 📱 Android

### Requisiti
- **Android Studio** (include SDK + emulatore)
- **Android NDK** (installabile da Android Studio → SDK Manager → SDK Tools → NDK)
- **JDK 17**
- Variabili d'ambiente:
  ```bash
  export ANDROID_HOME="$HOME/Android/Sdk"       # o il percorso del tuo SDK
  export NDK_HOME="$ANDROID_HOME/ndk/<versione>"
  ```
- Target Rust per Android:
  ```bash
  rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
  ```

### Comandi
```bash
npm run android:init      # una volta sola: genera il progetto Android
npm run android:dev       # esegue su emulatore o dispositivo collegato
npm run android:build     # genera APK/AAB firmabili per il Play Store
```

Gli artefatti finali (`.apk` / `.aab`) vengono creati in
`src-tauri/gen/android/app/build/outputs/`.

---

## 🍎 iOS

> La build iOS richiede **macOS** con **Xcode** (Apple non consente di compilare per iOS da Windows/Linux).

### Requisiti
- **macOS** + **Xcode** (con Command Line Tools) + **CocoaPods** (`brew install cocoapods`)
- Un **Apple ID** / account sviluppatore per firmare l'app
- Target Rust per iOS:
  ```bash
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
  ```

### Comandi
```bash
npm run ios:init          # una volta sola: genera il progetto Xcode
npm run ios:dev           # esegue su simulatore o iPhone collegato
npm run ios:build         # genera l'app per l'App Store / installazione
```

Per pubblicare (o installare su un iPhone reale) apri il progetto generato in
`src-tauri/gen/apple/` con Xcode e imposta il tuo Team di firma.

---

## 💻 Desktop (opzionale)

L'app gira anche come applicazione desktop:

```bash
npm run dev               # sviluppo con hot-reload
npm run build             # crea l'installer per il tuo sistema
```

Su Linux servono le dipendenze di sistema (webkit2gtk, ecc.): vedi
[prerequisiti Tauri](https://v2.tauri.app/start/prerequisites/).

---

## 💾 Dati e privacy

- I lemmi sono salvati nel `localStorage` della webview, **dentro la sandbox dell'app**: restano sul telefono e sopravvivono alle riaperture.
- **Al primo avvio**, se il dispositivo è online, l'app scarica una volta il dizionario condiviso dal Gist come punto di partenza; se sei offline parte dai lemmi predefiniti. Da quel momento in poi **tutto resta locale** e non serve più la rete.
- **Sincronizzazione cloud (opzionale):** se in futuro vuoi condividere le modifiche tra più dispositivi, puoi inserire un Personal Access Token GitHub dall'app; senza token l'app ignora completamente il cloud.
- La modalità **editor** (aggiungi/modifica/elimina) è protetta da password; in sola lettura chiunque può consultare i lemmi. La password predefinita è `rotellese` (modificabile cambiando l'hash `PW_HASH` in `src/index.html`).

---

## 🎨 Rigenerare le icone

Le icone di tutte le piattaforme si generano dall'immagine sorgente `app-icon.png`:

```bash
npx tauri icon app-icon.png
```

---

## Personalizzazione rapida

| Cosa | Dove |
|------|------|
| Nome app / versione / bundle id | `src-tauri/tauri.conf.json` |
| Interfaccia, lemmi, logica | `src/index.html` |
| Password editor (hash SHA-256) | costante `PW_HASH` in `src/index.html` |
| Icona | `app-icon.png` → `npx tauri icon` |
