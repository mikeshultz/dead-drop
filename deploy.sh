#!/bin/bash

# Exit on error
set -e

export NAME="dead-drop"
export NAMESPACE="dead-pub"

# Figure out our docker tags
export BUILD_ID="$(date +%Y%m%d%H%M%S)"
export REGISTRY="image.mikes.network/mikeshultz/$NAME"
export TAG="$REGISTRY:$BUILD_ID"

echo "Building $TAG..."

# Build & send it
docker build -f devops/dockerfiles/$NAME -t $TAG .
docker tag $TAG $REGISTRY:latest
docker push $TAG
docker push $REGISTRY:latest

echo "Setting image for deployment/$NAME to $NAME=$TAG"
kubectl -n $NAMESPACE set image deployment/$NAMESPACE-$NAME $NAME=$TAG

echo "Complete"
