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

## Utilisation

### 1 Variables d'environnement
```
 export "PING_LISTEN_PORT=VOTRE_PORT" && cargo run
```
*Si aucune valeur n'est renseignée, le port aura comme valeur 8080*
```
 cargo run
```

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


## Docker

### Build en une étape

```bash
docker build -t apirust -f .\Step1.Dockerfile .
docker run -p 8080:8080 apirust
```

[Analyse de l'image Docker avec Docker Scout](1.md)

### Build multi-étapes
```bash
docker build -t apirust -f .\Step2.Dockerfile .
docker run -p 8080:8080 apirust
```

[Analyse de l'image Docker avec Docker Scout](2.md)