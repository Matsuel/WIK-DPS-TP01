# WIK-DPS-TP01
## Installation du projet

### Prérequis
- Installer [Rust](https://www.rust-lang.org/tools/install)
- Installer [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation
```bash
git clone https://github.com/Matsuel/WIK-DPS-TP01
cd WIK-DPS-TP01
cargo build
```
```bash
cargo run
```

## Utilisation

### 1 Créer un fichier .env à la racine du projet
```
PING_LISTEN_PORT=VOTRE_PORT
```
*Si aucune valeur n'est renseignée, le port aura comme valeur 8080*

### 2 Seule route est disponible : GET /ping
- Le serveur renvoie la liste des headers de la requête sous forme de JSON
```
curl -X GET http://localhost:8080/ping -> 200 OK
```

- Le serveur renvoie une erreur 404 si la route est incorrecte
```
curl -X GET http://localhost:8080/pong -> 404 Not Found
```

### 3 Réponses possibles
- 200 OK : La requête a été traitée avec succès
- 404 Not Found : La ressource demandée n'a pas été trouvée (route inexistante ou méthode HTTP incorrecte)
