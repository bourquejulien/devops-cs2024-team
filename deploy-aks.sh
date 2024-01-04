#!/bin/bash
set -e

TEAM_NAME=$1
ACR_NAME=$2
IMAGE_NAME="${ACR_NAME}.azurecr.io/rusters:latest" #teamacr000cee89641c6587
CHART_NAME=rusters

az acr login -n "$ACR_NAME"
docker build --target final -t "$IMAGE_NAME" .
docker push "$IMAGE_NAME"
az aks get-credentials --resource-group "CS-${TEAM_NAME}-rg" --name "team-cluster"

if ! (helm ls  | grep $CHART_NAME) then
   echo "Installing Helm Chart"
   helm install $CHART_NAME helm/
else
   echo "Upgrading Helm Chart"
   helm upgrade $CHART_NAME helm/
fi
