#!/bin/sh

set -e

CHART_NAME=osrs-hiscore-proxy
NAMESPACE=osrs-hiscore-proxy
# Grab the latest *published* master
VERSION=$(git rev-parse origin/master)

if $(helm list --namespace $NAMESPACE | grep -q $CHART_NAME); then
    SUBCOMMAND=upgrade
else
    SUBCOMMAND=install
fi

set -x
helm $SUBCOMMAND $CHART_NAME helm/ --namespace $NAMESPACE --create-namespace --set versionSha=$VERSION
