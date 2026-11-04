# aidoku-scanvf

Source [Aidoku](https://aidoku.app) pour **Scan VF** (https://www.scan-vf.net), utilisable dans Aidoku (iOS) et dans [Rakuyomi](https://github.com/hanatsumi/rakuyomi) (KOReader).

Le site tourne sous *My Manga Reader CMS* : la source réutilise le template `mmrcms` de [Aidoku-Community/sources](https://github.com/Aidoku-Community/sources) (Apache-2.0 / MIT), légèrement adapté car scan-vf sert ses mangas à `/{slug}` et non à `/manga/{slug}`.

> Statut : sélecteurs vérifiés sur le HTML du site (novembre 2026) : fiche, chapitres, pages, recherche, derniers chapitres et populaires. Pas encore testé dans Rakuyomi.

## Utilisation

Une fois GitHub Pages activé (Settings → Pages → branche `gh-pages`), la liste de sources est disponible à :

```
https://esbag.github.io/aidoku-scanvf/index.min.json
```

Dans le `settings.json` de Rakuyomi :

```json
"source_lists": [
  { "url": "https://esbag.github.io/aidoku-scanvf/index.min.json" }
]
```

## Structure

```
sources/fr.scanvf/      la source (Rust → WebAssembly)
  res/source.json       métadonnées (id, nom, version, listings)
  res/icon.png
  src/lib.rs
templates/mmrcms/       template partagé My Manga Reader CMS
.github/workflows/      compilation + publication sur gh-pages
```

## Développer

Prérequis : Rust, la cible WebAssembly et `aidoku-cli`.

```sh
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli
```

Compiler et empaqueter :

```sh
cd sources/fr.scanvf
aidoku package          # produit package.aix
aidoku verify package.aix
```

Vérification rapide sans la cible WebAssembly (contrôle de types seulement ; les erreurs « global memory allocator » et « panic_handler » sont normales hors wasm) :

```sh
./scripts/check.sh
```

Chaque push sur `main` compile la source avec GitHub Actions et publie la liste sur la branche `gh-pages`. Pensez à incrémenter `version` dans `res/source.json` à chaque modification pour que Rakuyomi propose la mise à jour.

## Licence

Apache-2.0 ou MIT, au choix (voir `LICENSE-APACHE` et `LICENSE-MIT`).
