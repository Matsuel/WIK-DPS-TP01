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

### 1 Seule route est disponible : GET /ping
- Le serveur renvoie la liste des headers de la requête sous forme de JSON
```
curl -X GET http://localhost:8080/ping
```

### 2 Réponses possibles
- 200 OK : La requête a été traitée avec succès
- 404 Not Found : La ressource demandée n'a pas été trouvée (route inexistante ou méthode HTTP incorrecte)