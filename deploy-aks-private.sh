#!/bin/bash
set -e

INFO_COLOR="\e[1;34m"
OK_COLOR="\e[1;32m"
WARNING_COLOR="\e[3;31m"
ENDCOLOR="\e[0m"

TEAM_NAME=$1

DOMAIN_NAME="dev.cs2024.one"
IMAGE_TAG="latest"
PROJECT_NAME="rusters"
CLUSTER_NAME="team"
RG_NAME="CS-${TEAM_NAME}-rg"

echo -e "${INFO_COLOR}Retrieving random id:${ENDCOLOR}"
RANDOM_ID=$(az acr list --resource-group Global-rg | jq -r ".[0].name")
echo "$RANDOM_ID"

ACR_NAME="${CLUSTER_NAME}${TEAM_NAME}${RANDOM_ID}"
REPO_NAME="${ACR_NAME}.azurecr.io"
IMAGE_NAME="${REPO_NAME}/${PROJECT_NAME}:${IMAGE_TAG}"

echo -e "${INFO_COLOR}Pushing to ACR:${ENDCOLOR}"
az acr login -n "$ACR_NAME"
docker build --target final -t "$IMAGE_NAME" .
docker push "$IMAGE_NAME"
az aks get-credentials --overwrite-existing --resource-group "$RG_NAME" --name "${CLUSTER_NAME}cluster"

deploy_to_aks() {
    if ! (helm ls  | grep "$1") then
       echo -e "${OK_COLOR}Installing Helm Chart${ENDCOLOR}"
       eval helm install -f "${2}values.yaml" $@
    else
       echo -e "${OK_COLOR}Upgrading Helm Chart${ENDCOLOR}"
       eval helm upgrade -f "${2}values.yaml" $@
    fi
}

echo -e "\n${INFO_COLOR}Deploying map to team cluster:${ENDCOLOR}"
deploy_to_aks "map" "map_helm/"

echo -e "\n${INFO_COLOR}Deploying projet to team cluster:${ENDCOLOR}"

VARIABLES+=("--set=image.repository=\"${REPO_NAME}/${PROJECT_NAME}\"")
VARIABLES+=("--set=image.tag=\"${IMAGE_TAG}\"")
VARIABLES+=("--set=ingress.hosts[0].host=\"team${TEAM_NAME}.${DOMAIN_NAME}\"")
VARIABLES="$(IFS=" " ; echo "${VARIABLES[*]}")"

deploy_to_aks $PROJECT_NAME "helm/" $VARIABLES
