# Cellion
## Description

Ce projet est la version 2 du projet [Calendar-viewer](https://github.com/bessantoy/Calendar-viewer). Le projet Calendar-viewer avait pour but de visualiser les résultats d'un [solveur de problème d'emploi du temps](https://ua-usp.github.io/timetabling/), mais il souffrait de lenteurs dues à la grande quantitée de données traitées dans le navigateur.

L'enjeu principal de cette nouvelle version était de délocaliser le traitement des données vers un backend performant, afin de pouvoir effectuer des requêtes sur de plus grandes instances et de pouvoir les afficher en un temps raisonnable. Ce projet fut également l'occasion de refaire le front-end afin d'avoir une interface plus fonctionnelle et plus fluide.


## Installation (Développement) 

### Pré-requis

Avoir une installation relativement récente de rust, cargo, nodejs et npm.

### Cloner le projet

`git clone https://github.com/GregoireBellon/cellion.git`

### Back

`cd cellion/back`

Installation des dépendances requises pour SQLite.

`sudo apt install -y sqlite3 libsqlite3-dev`

Installation de [Diesel](https://diesel.rs/).

`cargo install diesel_cli --no-default-features --features sqlite`

Installation de [cargo-watch](https://github.com/watchexec/cargo-watch
)

`cargo install cargo-watch`

Mise en place de la base de données

```bash
diesel setup
diesel migration run
```

Démarrer l'api
`cargo watch -x  run`



### Front

```bash
cd front
npm install
```

Démarrer le front `npm run dev`


## Déploiement

### Déploiement automatique (Github Actions)

1. Fork le repository
2. Activer les Github Actions dans le menu "Actions"
3. Créer un environnement appelé "Dev" (Settings > Secrets And Variables > Environment Secrets > Manage Environment secrets > New Environment)
4. Lui ajouter ces 3 secrets
```
SERVER_IP=<IP du serveur sur lequel déployer l'application>
ANSIBLE_USER=<Utilisateur avec lequel Ansible va se connecter au serveur>
SSH_PRIVATE_KEY=<Clé privée de cet utilisateur>
```
Attention: l'utilisateur Ansible doit pouvoir executer `sudo` sans mot de passe

5. L'application est désormais déployée automatiquement à chaque push sur la branche `main`.  Pour déclencher le deploiement manuellement, vous pouvez aller dans Actions > On push, et cliquer sur Run workflow

L'application est déployée sur `0.0.0.0:8080` par défaut, pour changer ce comportement, vous pouvez éditer les variables `cellion_port` et `cellion_host` dans `deploy/vars.yml`


### Déploiement manuel

`git clone https://github.com/GregoireBellon/cellion.git`

`docker-compose up --build --force-recreate`
    



