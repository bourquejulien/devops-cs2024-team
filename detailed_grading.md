# DEVOPS - CSGames 2024 - Correction détaillée

## Critères d'évaluation généraux

| Critères                | Score /20 |
|-------------------------|-----------|
| Code de base et Docker  | /5        |
| Déploiement             | /6        |
| Accéder à la jungle     | /2        |
| Libérer les prisonniers | /6        |
| Qualité de la solution  | /1        |

> Un bonus de 0.5 point est accordé pour les équipes indiquant où se trouve la jungle.

## Critères d'évaluation spécifiques

Les critères décrivent les comportements attendus. Les points associés sont indiqués sous forme de pénalité si un critère n'est pas respecté.
Une sous-catégorie ne peut avoir de score négatif.

### 1. Code de départ (5 points)
#### 1.1 Serveur HTTP (1.5 point)
- Le serveur web doit être fonctionnel : -1.
- Le serveur web doit retourner un code d'erreur 2XX sur les requêtes effectuées : -0.5.
- Le serveur web doit être codé à l'aide de Rust : -1.5
- Le serveur web doit s'arrêter avec un SIGINT : -0.5.

#### 1.2 Conteneur Docker (3.5 points)
- L'image construite doit permettre l'utilisation du serveur web (l'image doit être fonctionnelle) : -2.5.
- Le conteneur docker doit être séparé entre le build et l'exécution : -1.
- L'image choisie pour l'exécution doit limiter les risque de sécurité (une image "lite") : -1
- La commande EXPOSE est utilisée : -0.5.
- Le Dockerfile maximise l'utilisation de la cache (ex. installation des libraries avant de lancer le build du code) : -0.5
- Le programme est lancé par un utilisateur ``unprivileged`` : -0.5.

### 2. Déploiement (6 points)
#### 2.1 Helm (2 points)
- Il est possible de déployer le conteneur : -1.5.
- Le pod est accessible depuis le cluster AI : -0.5.
- Il est possible d'accéder au pod depuis internet : -0.5.

#### 2.2 Azure (2 point)
- Le script permet de déployer les charts sur kubernetes : -1.
- Le script permet de pousser l'image de l'équipe sur Azure : -1.
- Le script permet de se connecter au ACR : -0.5.

#### 2.3 Pipeline Gitlab (2 points)
- Le pipeline contient une étape permettant de compiler le code : -1.
- Utilisation de la cache : -0.5.
- Le pipeline contient une étape permettant de linter (ou formatter) le code : -0.5.
- Le pipeline permet de compiler et pousser l'image de l'équipe : -1.
- Le pipeline permet d'effectuer le déploiement à l'aide d'Helm : -1.
- Ces étapes doivent être exécutées à partir d'un script : -0.25.
- Docker-in-docker doit être utilisé : -0.25.
- Les étapes de lint et de build doivent avoir lieu lors d'un commit sur main et lors d'une merge request : -0.25.
- L'étape de déploiement doit uniquement avoir lieu lors d'un commit sur main et lors de l'utilisation d'un tag : -0.25.

### 3. Accéder à la jungle (2 points)
- Les ``Steps`` retournées sont éclairements afficheés : -2.
- L'interface est conviviale, contient un tableau html : -1.
- L'interface contient un bouton "refresh" : -1.
- Le logo est affiché suite à un appel au serveur : -0.5
- Le compteur est présent et fonctionnel : -0.5

### 4. Libérer les prisonniers (6 points)
L'ensemble de ces étapes doivent être réalisé à l'aide de Rust. Autrement aucun point ne sera accordé.

#### 4.1 Fournir un accès (1 point)
- Un accès doit être fourni, le pod répond : -1.
- Le serveur lit correctement le paramètre : -0.5.
- Le status est retourné : -0.5.

#### 4.2 Météo (1 point)
- Le serveur répond à la requête : -1.5.
- Le payload météo retourné est lisible : -1.5.
- Les coordonnées sont utilisées : -0.5.
- Les informations météo retournées sont à jour : -0.5.
- Une api météo est utilisée : -0.5.

#### 4.3 Carte (1 point)
- Le conteneur de la carte est déployé dans le cluster : -1.
- Le conteneur de la carte est déployé à l'aide d'Helm (ou au moins d'un script) : -1.
- La carte est accessible depuis le pod de l'équipe : -1.
- Des requêtes sont effectué sur le conteneur (map) : -1.
- Les bonnes valeurs sont retournées à la validation : -1.

#### 4.4 Ouverture de la porte (1 point)
- Le mot de passé retourné en temps : -0.5
- Le hash est correctement reçu et traité : -0.5
- Un mot de passe (pas nécessairement le bon) est retournée à la bonne adresse : -0.5
- La liste des 10 derniers mots de passes est accessible et correctement triée : -0.25.

#### 4.5 Gain de popularité (2 point)
- Le load balancer est fonctionnel : -1.
  - Le load balancer déploie 3 pods : -0.25.
- La cache est fonctionnelle : -0.5.
  - La cache se met à jour à chaque 15 secondes : -0.25.
  - La requête est mise en attente si aucune donnée n'est disponible : -0.25.
- Le rate limit est fonctionnel et appliqué par l'ingress : -1.
  - La limite est appliqué au bon blocs d'Ips
  - En cas de rate limit, le code retourné est 503 : -0.25.

### Qualité de la solution
Le dernier critère sera évalué en fonction de la cohérence générale de la solution et sera évalué selon les
critères suivants (perte de 1 point au maximum) :
- Aucun linter n'est utilisé : -0.5.
- Code dupliqué (ex. ne pas utiliser le script de déploiement dans le pipeline) : -0.5.
- Présence de secrets dans le code (ex. mot de passe) : -0.5.
- Valeurs propres à environment présentes directement dans le code (tentez d'utiliser des variables d'environnement) : -0.25.
